mod cli;
mod grid;
mod font;
mod text;
mod animation;
mod palette;
mod svg;
mod render;
mod ffmpeg;

use std::fs;
use std::path::PathBuf;

use clap::Parser;

use cli::{Cli, Format, Mode, Theme};
use palette::{GITHUB_DARK, GITHUB_LIGHT};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    // Generate grid
    let grid = grid::generate_grid();

    // Render text sprite
    let sprite = text::render_text(&args.message);
    if sprite.width == 0 {
        return Err("Message produced no renderable pixels. Check your message text.".into());
    }

    // Determine palettes to use
    let themes: Vec<(&palette::Palette, Option<&palette::Palette>, &str)> = match args.theme {
        Theme::Light => vec![(&GITHUB_LIGHT, None, "light")],
        Theme::Dark => vec![(&GITHUB_DARK, None, "dark")],
        Theme::Both => vec![(&GITHUB_LIGHT, Some(&GITHUB_DARK), "both")],
    };

    let ext = args.format.extension();

    for (pal, dark_pal, theme_name) in &themes {
        // Compute animation
        let anim_result = match args.mode {
            Mode::Scroll => animation::compute_scroll_animation(
                &grid, &sprite, pal, &args.color, args.speed,
            ),
            Mode::Static => animation::compute_static_animation(
                &grid, &sprite, pal, &args.color,
            ),
        };

        // Determine output path
        let output_path = if let Some(ref out) = args.output {
            PathBuf::from(out)
        } else if *theme_name == "both" {
            PathBuf::from(format!("contribution-graph.{}", ext))
        } else {
            PathBuf::from(format!("contribution-graph-{}.{}", theme_name, ext))
        };

        match args.format {
            Format::Svg => {
                let svg_content = svg::render_svg(
                    &grid,
                    &anim_result.keyframes,
                    pal,
                    *dark_pal,
                    anim_result.duration,
                    11,
                    3,
                );
                fs::write(&output_path, &svg_content)?;
                println!("Created {}", output_path.display());
            }
            Format::Gif | Format::Webp | Format::Mp4 => {
                ffmpeg::check_ffmpeg().map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;

                let tmp_dir = tempfile::tempdir()?;
                render::render_frames_to_disk(
                    &grid,
                    &anim_result.keyframes,
                    pal,
                    anim_result.duration,
                    11,
                    3,
                    20,
                    tmp_dir.path(),
                ).map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;

                ffmpeg::export(tmp_dir.path(), &output_path, &args.format, 20, anim_result.duration)
                    .map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;

                println!("Created {}", output_path.display());
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
