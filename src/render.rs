use image::{Rgb, RgbImage};

use crate::animation::AnimationKeyframe;
use crate::grid::{Grid, NUM_COLS, NUM_ROWS};
use crate::palette::Palette;

fn parse_hex_color(hex: &str) -> Rgb<u8> {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    Rgb([r, g, b])
}

/// Maximum number of frames to prevent OOM (20 fps × 300s = 6000 frames)
pub const MAX_FRAMES: usize = 6_000;

/// Render frames and stream them directly to disk as PNGs.
/// Returns the number of frames written.
pub fn render_frames_to_disk(
    grid: &Grid,
    keyframes: &[AnimationKeyframe],
    palette: &Palette,
    duration: f64,
    cell_size: u32,
    cell_gap: u32,
    fps: u32,
    output_dir: &std::path::Path,
) -> Result<usize, String> {
    let pitch = cell_size + cell_gap;
    let img_width = NUM_COLS as u32 * pitch - cell_gap;
    let img_height = NUM_ROWS as u32 * pitch - cell_gap;
    let total_frames = (duration * fps as f64).ceil() as usize;

    if total_frames > MAX_FRAMES {
        return Err(format!(
            "Animation would produce {} frames (max {}). Shorten your message or increase --speed.",
            total_frames, MAX_FRAMES
        ));
    }

    struct ParsedKf {
        cell_x: usize,
        cell_y: usize,
        start_time: f64,
        end_time: f64,
        highlight_color: Rgb<u8>,
    }

    let parsed_kfs: Vec<ParsedKf> = keyframes
        .iter()
        .map(|kf| ParsedKf {
            cell_x: kf.cell_x,
            cell_y: kf.cell_y,
            start_time: kf.start_time,
            end_time: kf.end_time,
            highlight_color: parse_hex_color(&kf.highlight_color),
        })
        .collect();

    let gray_color = parse_hex_color(palette.colors[0]);

    let mut animated_cells = std::collections::HashSet::new();
    for kf in &parsed_kfs {
        animated_cells.insert((kf.cell_x, kf.cell_y));
    }

    for frame_idx in 0..total_frames {
        let t = if total_frames <= 1 {
            0.0
        } else {
            frame_idx as f64 / total_frames as f64
        };

        let mut img = RgbImage::new(img_width, img_height);

        for pixel in img.pixels_mut() {
            *pixel = gray_color;
        }

        for col in 0..NUM_COLS {
            for row in 0..NUM_ROWS {
                if let Some(cell) = &grid[col][row] {
                    let color = parse_hex_color(palette.colors[cell.level as usize]);
                    let px = col as u32 * pitch;
                    let py = row as u32 * pitch;
                    for dy in 0..cell_size {
                        for dx in 0..cell_size {
                            if px + dx < img_width && py + dy < img_height {
                                img.put_pixel(px + dx, py + dy, color);
                            }
                        }
                    }
                }
            }
        }

        for &(cx, cy) in &animated_cells {
            let px = cx as u32 * pitch;
            let py = cy as u32 * pitch;
            for dy in 0..cell_size {
                for dx in 0..cell_size {
                    if px + dx < img_width && py + dy < img_height {
                        img.put_pixel(px + dx, py + dy, gray_color);
                    }
                }
            }
        }

        for kf in &parsed_kfs {
            if kf.start_time <= t && t < kf.end_time {
                let px = kf.cell_x as u32 * pitch;
                let py = kf.cell_y as u32 * pitch;
                for dy in 0..cell_size {
                    for dx in 0..cell_size {
                        if px + dx < img_width && py + dy < img_height {
                            img.put_pixel(px + dx, py + dy, kf.highlight_color);
                        }
                    }
                }
            }
        }

        let path = output_dir.join(format!("frame_{:05}.png", frame_idx + 1));
        img.save(&path).map_err(|e| format!("Failed to write frame {}: {}", frame_idx + 1, e))?;
    }

    Ok(total_frames)
}

/// Render all frames in memory (used by tests for small animations).
#[cfg(test)]
pub fn render_frames(
    grid: &Grid,
    keyframes: &[AnimationKeyframe],
    palette: &Palette,
    duration: f64,
    cell_size: u32,
    cell_gap: u32,
    fps: u32,
) -> Vec<RgbImage> {
    let pitch = cell_size + cell_gap;
    let img_width = NUM_COLS as u32 * pitch - cell_gap;
    let img_height = NUM_ROWS as u32 * pitch - cell_gap;
    let total_frames = (duration * fps as f64).ceil() as usize;
    let mut frames = Vec::with_capacity(total_frames);

    struct ParsedKf {
        cell_x: usize,
        cell_y: usize,
        start_time: f64,
        end_time: f64,
        highlight_color: Rgb<u8>,
    }

    let parsed_kfs: Vec<ParsedKf> = keyframes
        .iter()
        .map(|kf| ParsedKf {
            cell_x: kf.cell_x,
            cell_y: kf.cell_y,
            start_time: kf.start_time,
            end_time: kf.end_time,
            highlight_color: parse_hex_color(&kf.highlight_color),
        })
        .collect();

    let gray_color = parse_hex_color(palette.colors[0]);

    let mut animated_cells = std::collections::HashSet::new();
    for kf in &parsed_kfs {
        animated_cells.insert((kf.cell_x, kf.cell_y));
    }

    for frame_idx in 0..total_frames {
        let t = if total_frames <= 1 {
            0.0
        } else {
            frame_idx as f64 / total_frames as f64
        };

        let mut img = RgbImage::new(img_width, img_height);

        for pixel in img.pixels_mut() {
            *pixel = gray_color;
        }

        for col in 0..NUM_COLS {
            for row in 0..NUM_ROWS {
                if let Some(cell) = &grid[col][row] {
                    let color = parse_hex_color(palette.colors[cell.level as usize]);
                    let px = col as u32 * pitch;
                    let py = row as u32 * pitch;
                    for dy in 0..cell_size {
                        for dx in 0..cell_size {
                            if px + dx < img_width && py + dy < img_height {
                                img.put_pixel(px + dx, py + dy, color);
                            }
                        }
                    }
                }
            }
        }

        for &(cx, cy) in &animated_cells {
            let px = cx as u32 * pitch;
            let py = cy as u32 * pitch;
            for dy in 0..cell_size {
                for dx in 0..cell_size {
                    if px + dx < img_width && py + dy < img_height {
                        img.put_pixel(px + dx, py + dy, gray_color);
                    }
                }
            }
        }

        for kf in &parsed_kfs {
            if kf.start_time <= t && t < kf.end_time {
                let px = kf.cell_x as u32 * pitch;
                let py = kf.cell_y as u32 * pitch;
                for dy in 0..cell_size {
                    for dx in 0..cell_size {
                        if px + dx < img_width && py + dy < img_height {
                            img.put_pixel(px + dx, py + dy, kf.highlight_color);
                        }
                    }
                }
            }
        }

        frames.push(img);
    }

    frames
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation::compute_scroll_animation;
    use crate::grid::generate_grid;
    use crate::palette::GITHUB_LIGHT;
    use crate::text::render_text;

    #[test]
    fn test_render_frames_count() {
        let grid = generate_grid();
        let sprite = render_text("A");
        let result = compute_scroll_animation(&grid, &sprite, &GITHUB_LIGHT, "#40c463", 0.15);
        let frames = render_frames(&grid, &result.keyframes, &GITHUB_LIGHT, result.duration, 11, 3, 20);
        let expected = (result.duration * 20.0).ceil() as usize;
        assert_eq!(frames.len(), expected);
    }

    #[test]
    fn test_render_frame_dimensions() {
        let grid = generate_grid();
        let sprite = render_text("A");
        let result = compute_scroll_animation(&grid, &sprite, &GITHUB_LIGHT, "#40c463", 0.15);
        let frames = render_frames(&grid, &result.keyframes, &GITHUB_LIGHT, result.duration, 11, 3, 20);
        let expected_w = NUM_COLS as u32 * 14 - 3;
        let expected_h = NUM_ROWS as u32 * 14 - 3;
        for frame in &frames {
            assert_eq!(frame.width(), expected_w);
            assert_eq!(frame.height(), expected_h);
        }
    }

    #[test]
    fn test_parse_hex_color() {
        let c = parse_hex_color("#ff0000");
        assert_eq!(c, Rgb([255, 0, 0]));
        let c = parse_hex_color("#00ff00");
        assert_eq!(c, Rgb([0, 255, 0]));
        let c = parse_hex_color("#0000ff");
        assert_eq!(c, Rgb([0, 0, 255]));
    }
}
