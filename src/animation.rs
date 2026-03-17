use crate::grid::{Grid, NUM_COLS, NUM_ROWS};
use crate::palette::Palette;
use crate::text::TextSprite;

#[derive(Debug, Clone)]
pub struct AnimationKeyframe {
    pub cell_x: usize,
    pub cell_y: usize,
    pub start_time: f64,
    pub end_time: f64,
    pub base_color: String,
    pub highlight_color: String,
    #[allow(dead_code)]
    pub begin_offset: f64,
}

#[derive(Debug)]
pub struct AnimationResult {
    pub keyframes: Vec<AnimationKeyframe>,
    pub duration: f64,
}

pub fn compute_scroll_animation(
    grid: &Grid,
    sprite: &TextSprite,
    palette: &Palette,
    text_color: &str,
    speed: f64,
) -> AnimationResult {
    let gray_color = palette.colors[0].to_string();
    let total_steps = NUM_COLS + sprite.width + 1;
    let duration = total_steps as f64 * speed;
    let mut keyframes = Vec::new();

    for step in 0..total_steps {
        let sprite_left = NUM_COLS as i32 - step as i32;
        let start_frac = step as f64 / total_steps as f64;
        let end_frac = (step + 1) as f64 / total_steps as f64;

        for pixel in &sprite.pixels {
            let gx = pixel.x as i32 + sprite_left;
            let gy = pixel.y;

            if gx < 0 || gx >= NUM_COLS as i32 {
                continue;
            }
            if gy >= NUM_ROWS {
                continue;
            }

            let col = gx as usize;
            if grid[col][gy].is_none() {
                continue;
            }

            keyframes.push(AnimationKeyframe {
                cell_x: col,
                cell_y: gy,
                start_time: start_frac,
                end_time: end_frac,
                base_color: gray_color.clone(),
                highlight_color: text_color.to_string(),
                begin_offset: step as f64 * speed,
            });
        }
    }

    // Track which cells got text keyframes
    let mut animated_cells = std::collections::HashSet::new();
    for kf in &keyframes {
        animated_cells.insert((kf.cell_x, kf.cell_y));
    }

    // Force all remaining non-null, non-level-0 cells to gray
    for col in 0..NUM_COLS {
        for row in 0..NUM_ROWS {
            if let Some(cell) = &grid[col][row] {
                if cell.level == 0 {
                    continue; // already gray
                }
                if animated_cells.contains(&(col, row)) {
                    continue; // already has text keyframes
                }
                keyframes.push(AnimationKeyframe {
                    cell_x: col,
                    cell_y: row,
                    start_time: 0.0,
                    end_time: 1.0,
                    base_color: gray_color.clone(),
                    highlight_color: gray_color.clone(),
                    begin_offset: 0.0,
                });
            }
        }
    }

    AnimationResult {
        keyframes,
        duration,
    }
}

pub fn compute_static_animation(
    grid: &Grid,
    sprite: &TextSprite,
    palette: &Palette,
    text_color: &str,
) -> AnimationResult {
    let gray_color = palette.colors[0].to_string();
    let mut keyframes = Vec::new();
    let duration = 3.0;

    let offset_x = if sprite.width < NUM_COLS {
        (NUM_COLS - sprite.width) / 2
    } else {
        0
    };

    for pixel in &sprite.pixels {
        let gx = pixel.x + offset_x;
        let gy = pixel.y;

        if gx >= NUM_COLS || gy >= NUM_ROWS {
            continue;
        }

        if grid[gx][gy].is_none() {
            continue;
        }

        keyframes.push(AnimationKeyframe {
            cell_x: gx,
            cell_y: gy,
            start_time: 0.0,
            end_time: 1.0,
            base_color: gray_color.clone(),
            highlight_color: text_color.to_string(),
            begin_offset: 0.0,
        });
    }

    // Track which cells got text keyframes
    let mut animated_cells = std::collections::HashSet::new();
    for kf in &keyframes {
        animated_cells.insert((kf.cell_x, kf.cell_y));
    }

    // Force all remaining non-null, non-level-0 cells to gray
    for col in 0..NUM_COLS {
        for row in 0..NUM_ROWS {
            if let Some(cell) = &grid[col][row] {
                if cell.level == 0 {
                    continue;
                }
                if animated_cells.contains(&(col, row)) {
                    continue;
                }
                keyframes.push(AnimationKeyframe {
                    cell_x: col,
                    cell_y: row,
                    start_time: 0.0,
                    end_time: 1.0,
                    base_color: gray_color.clone(),
                    highlight_color: gray_color.clone(),
                    begin_offset: 0.0,
                });
            }
        }
    }

    AnimationResult {
        keyframes,
        duration,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::generate_grid;
    use crate::palette::GITHUB_LIGHT;
    use crate::text::render_text;

    #[test]
    fn test_scroll_keyframe_count() {
        let grid = generate_grid();
        let sprite = render_text("HI");
        let result = compute_scroll_animation(&grid, &sprite, &GITHUB_LIGHT, "#40c463", 0.15);
        assert!(!result.keyframes.is_empty(), "Should have keyframes");
    }

    #[test]
    fn test_scroll_times_in_range() {
        let grid = generate_grid();
        let sprite = render_text("A");
        let result = compute_scroll_animation(&grid, &sprite, &GITHUB_LIGHT, "#40c463", 0.15);
        for kf in &result.keyframes {
            assert!(kf.start_time >= 0.0 && kf.start_time <= 1.0,
                "start_time {} out of range", kf.start_time);
            assert!(kf.end_time >= 0.0 && kf.end_time <= 1.0,
                "end_time {} out of range", kf.end_time);
        }
    }

    #[test]
    fn test_scroll_cell_coords_in_bounds() {
        let grid = generate_grid();
        let sprite = render_text("HELLO");
        let result = compute_scroll_animation(&grid, &sprite, &GITHUB_LIGHT, "#40c463", 0.15);
        for kf in &result.keyframes {
            assert!(kf.cell_x < NUM_COLS, "cell_x {} out of bounds", kf.cell_x);
            assert!(kf.cell_y < NUM_ROWS, "cell_y {} out of bounds", kf.cell_y);
        }
    }

    #[test]
    fn test_scroll_forces_non_text_cells_to_gray() {
        let grid = generate_grid();
        // "." only has pixels in row 6, so rows 0-5 won't get text keyframes
        let sprite = render_text(".");
        let result = compute_scroll_animation(&grid, &sprite, &GITHUB_LIGHT, "#40c463", 0.15);
        let gray_kfs: Vec<_> = result.keyframes.iter()
            .filter(|kf| kf.highlight_color == kf.base_color && kf.base_color == GITHUB_LIGHT.colors[0])
            .collect();
        assert!(!gray_kfs.is_empty(), "Should have force-to-gray keyframes for non-text cells");
    }

    #[test]
    fn test_scroll_duration() {
        let grid = generate_grid();
        let sprite = render_text("HI");
        let speed = 0.15;
        let result = compute_scroll_animation(&grid, &sprite, &GITHUB_LIGHT, "#40c463", speed);
        let total_steps = NUM_COLS + sprite.width + 1;
        let expected = total_steps as f64 * speed;
        assert!((result.duration - expected).abs() < 0.001,
            "Duration {} != expected {}", result.duration, expected);
    }

    #[test]
    fn test_static_text_pixels_get_highlight() {
        let grid = generate_grid();
        let sprite = render_text("A");
        let result = compute_static_animation(&grid, &sprite, &GITHUB_LIGHT, "#ff0000");
        // Text keyframes should use the custom highlight color
        let text_kfs: Vec<_> = result.keyframes.iter()
            .filter(|kf| kf.highlight_color != kf.base_color)
            .collect();
        assert!(!text_kfs.is_empty(), "Should have text keyframes");
        for kf in &text_kfs {
            assert_eq!(kf.highlight_color, "#ff0000");
        }
    }

    #[test]
    fn test_static_forces_non_text_cells_to_gray() {
        let grid = generate_grid();
        let sprite = render_text("A");
        let result = compute_static_animation(&grid, &sprite, &GITHUB_LIGHT, "#ff0000");
        // Force-to-gray keyframes should exist for non-text cells with level > 0
        let gray_kfs: Vec<_> = result.keyframes.iter()
            .filter(|kf| kf.highlight_color == kf.base_color)
            .collect();
        assert!(!gray_kfs.is_empty(), "Should have force-to-gray keyframes");
        for kf in &gray_kfs {
            assert_eq!(kf.base_color, GITHUB_LIGHT.colors[0]);
        }
    }

    #[test]
    fn test_static_uses_gray_base() {
        let grid = generate_grid();
        let sprite = render_text("A");
        let result = compute_static_animation(&grid, &sprite, &GITHUB_LIGHT, "#ff0000");
        for kf in &result.keyframes {
            assert_eq!(kf.base_color, GITHUB_LIGHT.colors[0]);
        }
    }
}
