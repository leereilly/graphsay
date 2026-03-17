use crate::font;

#[derive(Debug, Clone)]
pub struct GridPoint {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct TextSprite {
    pub pixels: Vec<GridPoint>,
    pub width: usize,
}

pub fn render_text(message: &str) -> TextSprite {
    let mut pixels = Vec::new();
    let mut cursor_x: usize = 0;
    let mut any_char = false;

    for ch in message.chars() {
        if let Some(glyph) = font::get_glyph(ch) {
            let w = font::glyph_width(glyph);
            for (row_idx, row) in glyph.iter().enumerate() {
                for (col_idx, cell) in row.chars().enumerate() {
                    if cell == '#' {
                        pixels.push(GridPoint {
                            x: cursor_x + col_idx,
                            y: row_idx,
                        });
                    }
                }
            }
            cursor_x += w + 1; // glyph width + 1 col inter-char spacing
            any_char = true;
        }
    }

    let width = if any_char && cursor_x > 0 {
        cursor_x - 1 // remove trailing gap
    } else {
        0
    };

    TextSprite { pixels, width }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let sprite = render_text("");
        assert!(sprite.pixels.is_empty());
        assert_eq!(sprite.width, 0);
    }

    #[test]
    fn test_single_char() {
        let sprite = render_text("I");
        // 'I' glyph: ["###", ".#.", ".#.", ".#.", ".#.", ".#.", "###"]
        // width 3, pixels: row 0 has 3, rows 1-5 have 1 each, row 6 has 3
        // total: 3 + 1 + 1 + 1 + 1 + 1 + 3 = 11
        assert_eq!(sprite.width, 3);
        assert_eq!(sprite.pixels.len(), 11);
    }

    #[test]
    fn test_multi_char_spacing() {
        let sprite = render_text("HI");
        // H: width 5, I: width 3
        // Total width: 5 + 1 + 3 = 9
        assert_eq!(sprite.width, 9);
    }

    #[test]
    fn test_known_message_pixel_count() {
        let sprite = render_text("A");
        // 'A' glyph: [".###.", "#...#", "#...#", "#####", "#...#", "#...#", "#...#"]
        // Row 0: 3 pixels (.###.)
        // Row 1: 2 pixels (#...#)
        // Row 2: 2 pixels (#...#)
        // Row 3: 5 pixels (#####)
        // Row 4: 2 pixels (#...#)
        // Row 5: 2 pixels (#...#)
        // Row 6: 2 pixels (#...#)
        // Total: 3 + 2 + 2 + 5 + 2 + 2 + 2 = 18
        assert_eq!(sprite.pixels.len(), 18);
        assert_eq!(sprite.width, 5);
    }

    #[test]
    fn test_unknown_chars_skipped() {
        let sprite = render_text("\u{00e9}");
        assert!(sprite.pixels.is_empty());
        assert_eq!(sprite.width, 0);
    }
}
