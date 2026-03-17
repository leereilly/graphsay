use std::collections::HashMap;

use crate::animation::AnimationKeyframe;
use crate::grid::{Grid, NUM_COLS, NUM_ROWS};
use crate::palette::Palette;

pub fn render_svg(
    grid: &Grid,
    keyframes: &[AnimationKeyframe],
    palette: &Palette,
    dark_palette: Option<&Palette>,
    duration: f64,
    cell_size: u32,
    cell_gap: u32,
) -> String {
    let pitch = cell_size + cell_gap;
    let svg_width = NUM_COLS as u32 * pitch - cell_gap;
    let svg_height = NUM_ROWS as u32 * pitch - cell_gap;

    let grouped = group_keyframes_by_cell(keyframes);

    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n",
        svg_width, svg_height, svg_width, svg_height
    ));

    // Add dark mode CSS if needed
    if let Some(dark) = dark_palette {
        svg.push_str("<style>\n");
        svg.push_str(":root {\n");
        svg.push_str(&format!("  --bg: {};\n", "#ffffff"));
        for (i, color) in palette.colors.iter().enumerate() {
            svg.push_str(&format!("  --c{}: {};\n", i, color));
        }
        svg.push_str("}\n");
        svg.push_str("@media (prefers-color-scheme: dark) {\n");
        svg.push_str("  :root {\n");
        svg.push_str(&format!("    --bg: {};\n", "#0d1117"));
        for (i, color) in dark.colors.iter().enumerate() {
            svg.push_str(&format!("    --c{}: {};\n", i, color));
        }
        svg.push_str("  }\n");
        svg.push_str("}\n");
        svg.push_str("</style>\n");
        // Background rect with theme-aware color
        svg.push_str(&format!(
            "  <rect x=\"0\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"var(--bg)\" rx=\"0\" ry=\"0\"/>\n",
            svg_width, svg_height
        ));
    } else {
        // Single-theme background
        let bg = if palette.name == "github-dark" { "#0d1117" } else { "#ffffff" };
        svg.push_str(&format!(
            "  <rect x=\"0\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"{}\" rx=\"0\" ry=\"0\"/>\n",
            svg_width, svg_height, bg
        ));
    }

    for col in 0..NUM_COLS {
        for row in 0..NUM_ROWS {
            if let Some(cell) = &grid[col][row] {
                let x = col as u32 * pitch;
                let y = row as u32 * pitch;
                let key = format!("{},{}", col, row);

                if let Some(kfs) = grouped.get(&key) {
                    // Check if this cell's animation is "constant" (force-to-gray:
                    // highlight == base for all keyframes). If so, render as static.
                    let is_constant = kfs.iter().all(|kf| kf.highlight_color == kf.base_color);

                    // Check if this is a "full duration" static text cell
                    // (single highlight color spanning 0.0–1.0, e.g. static mode text)
                    let is_full_duration_text = !is_constant
                        && kfs.iter().all(|kf| kf.highlight_color == kfs[0].highlight_color)
                        && kfs.iter().any(|kf| kf.start_time <= 0.0 && kf.end_time >= 1.0);

                    if is_constant {
                        // Static forced-to-gray cell
                        let color = &kfs[0].base_color;
                        svg.push_str(&format!(
                            "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"2\" ry=\"2\" fill=\"{}\"/>\n",
                            x, y, cell_size, cell_size, color
                        ));
                    } else if is_full_duration_text {
                        // Static text cell — just show the highlight color, no animation
                        let color = &kfs[0].highlight_color;
                        svg.push_str(&format!(
                            "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"2\" ry=\"2\" fill=\"{}\"/>\n",
                            x, y, cell_size, cell_size, color
                        ));
                    } else {
                        // Animated cell: compute the midpoint color for static fallback
                        let midpoint_color = color_at_time(kfs, 0.5);
                        svg.push_str(&format!(
                            "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"2\" ry=\"2\" fill=\"{}\">\n",
                            x, y, cell_size, cell_size, midpoint_color
                        ));

                        let (key_times, values) = build_animate_attributes(kfs, duration);
                        svg.push_str(&format!(
                            "    <animate attributeName=\"fill\" dur=\"{}s\" repeatCount=\"1\" fill=\"freeze\" keyTimes=\"{}\" values=\"{}\" calcMode=\"discrete\"/>\n",
                            duration, key_times, values
                        ));

                        svg.push_str("  </rect>\n");
                    }
                } else {
                    // No keyframes at all — static cell with palette color
                    let level = cell.level as usize;
                    if dark_palette.is_some() {
                        svg.push_str(&format!(
                            "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"2\" ry=\"2\" fill=\"var(--c{})\"/>\n",
                            x, y, cell_size, cell_size, level
                        ));
                    } else {
                        let color = palette.colors[level];
                        svg.push_str(&format!(
                            "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"2\" ry=\"2\" fill=\"{}\"/>\n",
                            x, y, cell_size, cell_size, color
                        ));
                    }
                }
            }
        }
    }

    svg.push_str("</svg>");
    svg
}

fn group_keyframes_by_cell(keyframes: &[AnimationKeyframe]) -> HashMap<String, Vec<&AnimationKeyframe>> {
    let mut map: HashMap<String, Vec<&AnimationKeyframe>> = HashMap::new();
    for kf in keyframes {
        let key = format!("{},{}", kf.cell_x, kf.cell_y);
        map.entry(key).or_default().push(kf);
    }
    map
}

/// Determine the color a cell displays at a given time fraction (0.0–1.0).
/// Used to set the static `fill` fallback for when `<animate>` is stripped.
fn color_at_time<'a>(keyframes: &'a [&'a AnimationKeyframe], t: f64) -> &'a str {
    let base = &keyframes[0].base_color;
    for kf in keyframes {
        if kf.start_time <= t && t < kf.end_time {
            return &kf.highlight_color;
        }
    }
    base
}

fn build_animate_attributes(keyframes: &[&AnimationKeyframe], _duration: f64) -> (String, String) {
    let mut sorted: Vec<&&AnimationKeyframe> = keyframes.iter().collect();
    sorted.sort_by(|a, b| a.start_time.partial_cmp(&b.start_time).unwrap());

    let base_color = &sorted[0].base_color;

    // Build timeline entries: (time, color)
    let mut entries: Vec<(f64, &str)> = Vec::new();
    entries.push((0.0, base_color));

    for kf in &sorted {
        entries.push((kf.start_time, &kf.highlight_color));
        entries.push((kf.end_time, base_color));
    }

    // Sort by time
    entries.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // Deduplicate: keep last entry for same time
    let mut deduped: Vec<(f64, &str)> = Vec::new();
    for entry in &entries {
        if let Some(last) = deduped.last_mut() {
            if (last.0 - entry.0).abs() < 1e-10 {
                last.1 = entry.1;
                continue;
            }
        }
        deduped.push(*entry);
    }

    // Ensure we end at 1.0
    if let Some(last) = deduped.last() {
        if (last.0 - 1.0).abs() > 1e-10 {
            deduped.push((1.0, base_color));
        }
    }

    let key_times: String = deduped.iter()
        .map(|(t, _)| format!("{:.4}", t))
        .collect::<Vec<_>>()
        .join(";");
    let values: String = deduped.iter()
        .map(|(_, c)| c.to_string())
        .collect::<Vec<_>>()
        .join(";");

    (key_times, values)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation::compute_scroll_animation;
    use crate::grid::generate_grid;
    use crate::palette::{GITHUB_DARK, GITHUB_LIGHT};
    use crate::text::render_text;

    fn make_test_svg(dark: bool) -> String {
        let grid = generate_grid();
        let sprite = render_text("HI");
        let result = compute_scroll_animation(&grid, &sprite, &GITHUB_LIGHT, "#40c463", 0.15);
        let dark_pal = if dark { Some(&GITHUB_DARK) } else { None };
        render_svg(&grid, &result.keyframes, &GITHUB_LIGHT, dark_pal, result.duration, 11, 3)
    }

    #[test]
    fn test_svg_starts_and_ends() {
        let svg = make_test_svg(false);
        assert!(svg.starts_with("<svg"), "SVG should start with <svg");
        assert!(svg.ends_with("</svg>"), "SVG should end with </svg>");
    }

    #[test]
    fn test_svg_dimensions() {
        let svg = make_test_svg(false);
        let expected_w = NUM_COLS as u32 * 14 - 3;
        let expected_h = NUM_ROWS as u32 * 14 - 3;
        assert!(svg.contains(&format!("width=\"{}\"", expected_w)));
        assert!(svg.contains(&format!("height=\"{}\"", expected_h)));
    }

    #[test]
    fn test_svg_contains_rects() {
        let svg = make_test_svg(false);
        assert!(svg.contains("<rect"), "SVG should contain <rect elements");
    }

    #[test]
    fn test_dark_mode_includes_style() {
        let svg = make_test_svg(true);
        assert!(svg.contains("<style>"), "Dark mode SVG should contain <style>");
        assert!(svg.contains("prefers-color-scheme: dark"), "Should have dark mode media query");
    }

    #[test]
    fn test_animated_cells_have_animate() {
        let svg = make_test_svg(false);
        assert!(svg.contains("<animate"), "Should contain <animate> elements");
    }

    #[test]
    fn test_keytimes_format() {
        let svg = make_test_svg(false);
        // Find a keyTimes attribute and verify format
        if let Some(pos) = svg.find("keyTimes=\"") {
            let start = pos + "keyTimes=\"".len();
            if let Some(end) = svg[start..].find('"') {
                let kt = &svg[start..start + end];
                for part in kt.split(';') {
                    // Each should be formatted to 4 decimal places
                    assert!(part.contains('.'), "keyTime '{}' should have decimal", part);
                }
            }
        }
    }
}
