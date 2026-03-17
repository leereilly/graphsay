pub struct Palette {
    #[allow(dead_code)]
    pub name: &'static str,
    pub colors: [&'static str; 5],
}

pub const GITHUB_LIGHT: Palette = Palette {
    name: "github-light",
    colors: ["#ebedf0", "#9be9a8", "#40c463", "#30a14e", "#216e39"],
};

pub const GITHUB_DARK: Palette = Palette {
    name: "github-dark",
    colors: ["#161b22", "#01311f", "#034525", "#0f6d31", "#00c647"],
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_palette_has_5_colors() {
        assert_eq!(GITHUB_LIGHT.colors.len(), 5);
    }

    #[test]
    fn test_dark_palette_has_5_colors() {
        assert_eq!(GITHUB_DARK.colors.len(), 5);
    }

    #[test]
    fn test_colors_are_valid_hex() {
        for palette in [&GITHUB_LIGHT, &GITHUB_DARK] {
            for color in &palette.colors {
                assert!(color.starts_with('#'), "Color {} should start with #", color);
                assert_eq!(color.len(), 7, "Color {} should be 7 chars", color);
                for b in color[1..].bytes() {
                    assert!(
                        (b >= b'0' && b <= b'9') || (b >= b'a' && b <= b'f') || (b >= b'A' && b <= b'F'),
                        "Color {} has invalid hex char", color
                    );
                }
            }
        }
    }
}
