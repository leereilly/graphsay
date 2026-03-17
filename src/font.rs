// Complete 5x7 pixel font for printable ASCII 32-126
// Each glyph is 7 rows, '#' = on, '.' = off

static GLYPH_SPACE: [&str; 7] = ["...", "...", "...", "...", "...", "...", "..."];
static GLYPH_EXCL: [&str; 7] = ["#", "#", "#", "#", "#", ".", "#"];
static GLYPH_DQUOTE: [&str; 7] = ["#.#", "#.#", "...", "...", "...", "...", "..."];
static GLYPH_HASH: [&str; 7] = [".#.#.", "#####", ".#.#.", ".#.#.", "#####", ".#.#.", "....."];
static GLYPH_DOLLAR: [&str; 7] = [".#...", ".####", "#....", ".###.", "....#", "####.", "..#.."];
static GLYPH_PERCENT: [&str; 7] = ["##..#", "##.#.", "..#..", ".#...", ".#.##", "#..##", "....."];
static GLYPH_AMP: [&str; 7] = [".##..", "#..#.", ".##..", ".##.#", "#..#.", "#..#.", ".##.#"];
static GLYPH_SQUOTE: [&str; 7] = ["#", "#", ".", ".", ".", ".", "."];
static GLYPH_LPAREN: [&str; 7] = [".#", "#.", "#.", "#.", "#.", "#.", ".#"];
static GLYPH_RPAREN: [&str; 7] = ["#.", ".#", ".#", ".#", ".#", ".#", "#."];
static GLYPH_STAR: [&str; 7] = [".....", ".#.#.", "..#..", "#####", "..#..", ".#.#.", "....."];
static GLYPH_PLUS: [&str; 7] = [".....", "..#..", "..#..", "#####", "..#..", "..#..", "....."];
static GLYPH_COMMA: [&str; 7] = ["..", "..", "..", "..", "..", ".#", "#."];
static GLYPH_MINUS: [&str; 7] = [".....", ".....", ".....", "#####", ".....", ".....", "....."];
static GLYPH_PERIOD: [&str; 7] = [".", ".", ".", ".", ".", ".", "#"];
static GLYPH_SLASH: [&str; 7] = ["....#", "...#.", "..#..", ".#...", "#....", ".....", "....."];

static GLYPH_0: [&str; 7] = [".###.", "#...#", "#..##", "#.#.#", "##..#", "#...#", ".###."];
static GLYPH_1: [&str; 7] = [".#.", "##.", ".#.", ".#.", ".#.", ".#.", "###"];
static GLYPH_2: [&str; 7] = [".###.", "#...#", "....#", "..##.", ".#...", "#....", "#####"];
static GLYPH_3: [&str; 7] = [".###.", "#...#", "....#", "..##.", "....#", "#...#", ".###."];
static GLYPH_4: [&str; 7] = ["#..#.", "#..#.", "#..#.", "#####", "...#.", "...#.", "...#."];
static GLYPH_5: [&str; 7] = ["#####", "#....", "####.", "....#", "....#", "#...#", ".###."];
static GLYPH_6: [&str; 7] = [".###.", "#....", "#....", "####.", "#...#", "#...#", ".###."];
static GLYPH_7: [&str; 7] = ["#####", "....#", "...#.", "..#..", ".#...", ".#...", ".#..."];
static GLYPH_8: [&str; 7] = [".###.", "#...#", "#...#", ".###.", "#...#", "#...#", ".###."];
static GLYPH_9: [&str; 7] = [".###.", "#...#", "#...#", ".####", "....#", "....#", ".###."];

static GLYPH_COLON: [&str; 7] = [".", ".", "#", ".", ".", "#", "."];
static GLYPH_SEMI: [&str; 7] = ["..", "..", ".#", "..", "..", ".#", "#."];
static GLYPH_LT: [&str; 7] = ["...#", "..#.", ".#..", "#...", ".#..", "..#.", "...#"];
static GLYPH_EQ: [&str; 7] = [".....", ".....", "#####", ".....", "#####", ".....", "....."];
static GLYPH_GT: [&str; 7] = ["#...", ".#..", "..#.", "...#", "..#.", ".#..", "#..."];
static GLYPH_QUES: [&str; 7] = [".###.", "#...#", "....#", "..##.", "..#..", ".....", "..#.."];
static GLYPH_AT: [&str; 7] = [".###.", "#...#", "#.###", "#.#.#", "#.##.", "#....", ".###."];

static GLYPH_A: [&str; 7] = [".###.", "#...#", "#...#", "#####", "#...#", "#...#", "#...#"];
static GLYPH_B: [&str; 7] = ["####.", "#...#", "#...#", "####.", "#...#", "#...#", "####."];
static GLYPH_C: [&str; 7] = [".###.", "#...#", "#....", "#....", "#....", "#...#", ".###."];
static GLYPH_D: [&str; 7] = ["####.", "#...#", "#...#", "#...#", "#...#", "#...#", "####."];
static GLYPH_E: [&str; 7] = ["#####", "#....", "#....", "####.", "#....", "#....", "#####"];
static GLYPH_F: [&str; 7] = ["#####", "#....", "#....", "####.", "#....", "#....", "#...."];
static GLYPH_G: [&str; 7] = [".###.", "#...#", "#....", "#.###", "#...#", "#...#", ".###."];
static GLYPH_H: [&str; 7] = ["#...#", "#...#", "#...#", "#####", "#...#", "#...#", "#...#"];
static GLYPH_I: [&str; 7] = ["###", ".#.", ".#.", ".#.", ".#.", ".#.", "###"];
static GLYPH_J: [&str; 7] = ["..###", "...#.", "...#.", "...#.", "#..#.", "#..#.", ".##.."];
static GLYPH_K: [&str; 7] = ["#...#", "#..#.", "#.#..", "##...", "#.#..", "#..#.", "#...#"];
static GLYPH_L: [&str; 7] = ["#....", "#....", "#....", "#....", "#....", "#....", "#####"];
static GLYPH_M: [&str; 7] = ["#...#", "##.##", "#.#.#", "#...#", "#...#", "#...#", "#...#"];
static GLYPH_N: [&str; 7] = ["#...#", "##..#", "#.#.#", "#..##", "#...#", "#...#", "#...#"];
static GLYPH_O: [&str; 7] = [".###.", "#...#", "#...#", "#...#", "#...#", "#...#", ".###."];
static GLYPH_P: [&str; 7] = ["####.", "#...#", "#...#", "####.", "#....", "#....", "#...."];
static GLYPH_Q: [&str; 7] = [".###.", "#...#", "#...#", "#...#", "#.#.#", "#..#.", ".##.#"];
static GLYPH_R: [&str; 7] = ["####.", "#...#", "#...#", "####.", "#.#..", "#..#.", "#...#"];
static GLYPH_S: [&str; 7] = [".###.", "#...#", "#....", ".###.", "....#", "#...#", ".###."];
static GLYPH_T: [&str; 7] = ["#####", "..#..", "..#..", "..#..", "..#..", "..#..", "..#.."];
static GLYPH_U: [&str; 7] = ["#...#", "#...#", "#...#", "#...#", "#...#", "#...#", ".###."];
static GLYPH_V: [&str; 7] = ["#...#", "#...#", "#...#", "#...#", ".#.#.", ".#.#.", "..#.."];
static GLYPH_W: [&str; 7] = ["#...#", "#...#", "#...#", "#.#.#", "#.#.#", "##.##", "#...#"];
static GLYPH_X: [&str; 7] = ["#...#", "#...#", ".#.#.", "..#..", ".#.#.", "#...#", "#...#"];
static GLYPH_Y: [&str; 7] = ["#...#", "#...#", ".#.#.", "..#..", "..#..", "..#..", "..#.."];
static GLYPH_Z: [&str; 7] = ["#####", "....#", "...#.", "..#..", ".#...", "#....", "#####"];

static GLYPH_LBRACKET: [&str; 7] = ["##", "#.", "#.", "#.", "#.", "#.", "##"];
static GLYPH_BSLASH: [&str; 7] = ["#....", ".#...", "..#..", "...#.", "....#", ".....", "....."];
static GLYPH_RBRACKET: [&str; 7] = ["##", ".#", ".#", ".#", ".#", ".#", "##"];
static GLYPH_CARET: [&str; 7] = ["..#..", ".#.#.", "#...#", ".....", ".....", ".....", "....."];
static GLYPH_UNDER: [&str; 7] = [".....", ".....", ".....", ".....", ".....", ".....", "#####"];
static GLYPH_BTICK: [&str; 7] = ["#.", ".#", "..", "..", "..", "..", ".."];

static GLYPH_LBRACE: [&str; 7] = ["..#", ".#.", ".#.", "#..", ".#.", ".#.", "..#"];
static GLYPH_PIPE: [&str; 7] = ["#", "#", "#", "#", "#", "#", "#"];
static GLYPH_RBRACE: [&str; 7] = ["#..", ".#.", ".#.", "..#", ".#.", ".#.", "#.."];
static GLYPH_TILDE: [&str; 7] = [".....", ".....", ".##.#", "#.##.", ".....", ".....", "....."];

pub fn get_glyph(ch: char) -> Option<&'static [&'static str; 7]> {
    let ch = if ch.is_ascii_lowercase() {
        ch.to_ascii_uppercase()
    } else {
        ch
    };

    match ch {
        ' ' => Some(&GLYPH_SPACE),
        '!' => Some(&GLYPH_EXCL),
        '"' => Some(&GLYPH_DQUOTE),
        '#' => Some(&GLYPH_HASH),
        '$' => Some(&GLYPH_DOLLAR),
        '%' => Some(&GLYPH_PERCENT),
        '&' => Some(&GLYPH_AMP),
        '\'' => Some(&GLYPH_SQUOTE),
        '(' => Some(&GLYPH_LPAREN),
        ')' => Some(&GLYPH_RPAREN),
        '*' => Some(&GLYPH_STAR),
        '+' => Some(&GLYPH_PLUS),
        ',' => Some(&GLYPH_COMMA),
        '-' => Some(&GLYPH_MINUS),
        '.' => Some(&GLYPH_PERIOD),
        '/' => Some(&GLYPH_SLASH),
        '0' => Some(&GLYPH_0),
        '1' => Some(&GLYPH_1),
        '2' => Some(&GLYPH_2),
        '3' => Some(&GLYPH_3),
        '4' => Some(&GLYPH_4),
        '5' => Some(&GLYPH_5),
        '6' => Some(&GLYPH_6),
        '7' => Some(&GLYPH_7),
        '8' => Some(&GLYPH_8),
        '9' => Some(&GLYPH_9),
        ':' => Some(&GLYPH_COLON),
        ';' => Some(&GLYPH_SEMI),
        '<' => Some(&GLYPH_LT),
        '=' => Some(&GLYPH_EQ),
        '>' => Some(&GLYPH_GT),
        '?' => Some(&GLYPH_QUES),
        '@' => Some(&GLYPH_AT),
        'A' => Some(&GLYPH_A),
        'B' => Some(&GLYPH_B),
        'C' => Some(&GLYPH_C),
        'D' => Some(&GLYPH_D),
        'E' => Some(&GLYPH_E),
        'F' => Some(&GLYPH_F),
        'G' => Some(&GLYPH_G),
        'H' => Some(&GLYPH_H),
        'I' => Some(&GLYPH_I),
        'J' => Some(&GLYPH_J),
        'K' => Some(&GLYPH_K),
        'L' => Some(&GLYPH_L),
        'M' => Some(&GLYPH_M),
        'N' => Some(&GLYPH_N),
        'O' => Some(&GLYPH_O),
        'P' => Some(&GLYPH_P),
        'Q' => Some(&GLYPH_Q),
        'R' => Some(&GLYPH_R),
        'S' => Some(&GLYPH_S),
        'T' => Some(&GLYPH_T),
        'U' => Some(&GLYPH_U),
        'V' => Some(&GLYPH_V),
        'W' => Some(&GLYPH_W),
        'X' => Some(&GLYPH_X),
        'Y' => Some(&GLYPH_Y),
        'Z' => Some(&GLYPH_Z),
        '[' => Some(&GLYPH_LBRACKET),
        '\\' => Some(&GLYPH_BSLASH),
        ']' => Some(&GLYPH_RBRACKET),
        '^' => Some(&GLYPH_CARET),
        '_' => Some(&GLYPH_UNDER),
        '`' => Some(&GLYPH_BTICK),
        '{' => Some(&GLYPH_LBRACE),
        '|' => Some(&GLYPH_PIPE),
        '}' => Some(&GLYPH_RBRACE),
        '~' => Some(&GLYPH_TILDE),
        _ => None,
    }
}

pub fn glyph_width(glyph: &[&str; 7]) -> usize {
    glyph.iter().map(|row| row.len()).max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_printable_ascii_have_glyphs() {
        for c in 32u8..=126u8 {
            let ch = c as char;
            assert!(
                get_glyph(ch).is_some(),
                "Missing glyph for char '{}' ({})",
                ch,
                c
            );
        }
    }

    #[test]
    fn test_all_glyphs_have_7_rows() {
        for c in 32u8..=126u8 {
            let ch = c as char;
            if let Some(glyph) = get_glyph(ch) {
                assert_eq!(
                    glyph.len(),
                    7,
                    "Glyph for '{}' should have 7 rows, has {}",
                    ch,
                    glyph.len()
                );
            }
        }
    }

    #[test]
    fn test_glyph_width_positive() {
        for c in 32u8..=126u8 {
            let ch = c as char;
            if let Some(glyph) = get_glyph(ch) {
                let w = glyph_width(glyph);
                assert!(w > 0, "Glyph width for '{}' should be > 0", ch);
            }
        }
    }

    #[test]
    fn test_lowercase_maps_to_uppercase() {
        for c in b'a'..=b'z' {
            let lower = c as char;
            let upper = (c - 32) as char;
            let lower_glyph = get_glyph(lower);
            let upper_glyph = get_glyph(upper);
            assert_eq!(
                lower_glyph, upper_glyph,
                "Lowercase '{}' should map to uppercase '{}'",
                lower, upper
            );
        }
    }

    #[test]
    fn test_unknown_char_returns_none() {
        assert!(get_glyph('\x7f').is_none());
        assert!(get_glyph('\u{00e9}').is_none());
        assert!(get_glyph('\u{1f600}').is_none());
    }
}
