use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    Both,
}

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum Format {
    Svg,
    Gif,
    Webp,
    Mp4,
}

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum Mode {
    Scroll,
    Static,
}

fn validate_hex_color(s: &str) -> Result<String, String> {
    let re = regex_lite(s);
    if re {
        Ok(s.to_string())
    } else {
        Err(format!("Invalid hex color '{}'. Must match #[0-9a-fA-F]{{6}}", s))
    }
}

fn regex_lite(s: &str) -> bool {
    if s.len() != 7 {
        return false;
    }
    let bytes = s.as_bytes();
    if bytes[0] != b'#' {
        return false;
    }
    for &b in &bytes[1..] {
        if !((b >= b'0' && b <= b'9') || (b >= b'a' && b <= b'f') || (b >= b'A' && b <= b'F')) {
            return false;
        }
    }
    true
}

fn validate_speed(s: &str) -> Result<f64, String> {
    let v: f64 = s.parse().map_err(|_| format!("Invalid speed value '{}'", s))?;
    if !v.is_finite() || v <= 0.0 {
        return Err(format!("Speed must be a finite number greater than 0, got '{}'", s));
    }
    Ok(v)
}

impl Format {
    pub fn extension(&self) -> &'static str {
        match self {
            Format::Svg => "svg",
            Format::Gif => "gif",
            Format::Webp => "webp",
            Format::Mp4 => "mp4",
        }
    }
}

fn validate_message(s: &str) -> Result<String, String> {
    if s.len() > 500 {
        return Err(format!("Message too long ({} chars, max 500)", s.len()));
    }
    Ok(s.to_string())
}

#[derive(Parser, Debug)]
#[command(name = "animated-contribution-graph-message")]
#[command(about = "Generate animated GitHub contribution graph messages")]
#[command(version)]
pub struct Cli {
    /// Message to display (max 500 characters)
    #[arg(short, long = "message", value_parser = validate_message)]
    pub message: String,

    /// Theme: light, dark, or both
    #[arg(short, long, default_value = "both")]
    pub theme: Theme,

    /// Output format: svg, gif, webp, mp4
    #[arg(short, long, default_value = "mp4")]
    pub format: Format,

    /// Output file path
    #[arg(short, long)]
    pub output: Option<String>,

    /// Text color hex code
    #[arg(long, default_value = "#40c463", value_parser = validate_hex_color)]
    pub color: String,

    /// Animation mode: scroll or static
    #[arg(long, default_value = "scroll")]
    pub mode: Mode,

    /// Step interval in seconds (must be > 0)
    #[arg(long, default_value = "0.15", value_parser = validate_speed)]
    pub speed: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_default_values() {
        let cli = Cli::parse_from(["prog", "-m", "HELLO"]);
        assert_eq!(cli.theme, Theme::Both);
        assert_eq!(cli.format, Format::Mp4);
        assert_eq!(cli.mode, Mode::Scroll);
        assert_eq!(cli.color, "#40c463");
        assert!((cli.speed - 0.15).abs() < 0.001);
        assert!(cli.output.is_none());
    }

    #[test]
    fn test_required_message() {
        let result = Cli::try_parse_from(["prog"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_all_theme_variants() {
        for (val, expected) in [("light", Theme::Light), ("dark", Theme::Dark), ("both", Theme::Both)] {
            let cli = Cli::parse_from(["prog", "-m", "X", "-t", val]);
            assert_eq!(cli.theme, expected);
        }
    }

    #[test]
    fn test_all_format_variants() {
        for (val, expected) in [("svg", Format::Svg), ("gif", Format::Gif), ("webp", Format::Webp), ("mp4", Format::Mp4)] {
            let cli = Cli::parse_from(["prog", "-m", "X", "-f", val]);
            assert_eq!(cli.format, expected);
        }
    }

    #[test]
    fn test_all_mode_variants() {
        for (val, expected) in [("scroll", Mode::Scroll), ("static", Mode::Static)] {
            let cli = Cli::parse_from(["prog", "-m", "X", "--mode", val]);
            assert_eq!(cli.mode, expected);
        }
    }

    #[test]
    fn test_invalid_hex_color() {
        let result = Cli::try_parse_from(["prog", "-m", "X", "--color", "red"]);
        assert!(result.is_err());
        let result = Cli::try_parse_from(["prog", "-m", "X", "--color", "#GGG000"]);
        assert!(result.is_err());
        let result = Cli::try_parse_from(["prog", "-m", "X", "--color", "#12345"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_speed_zero() {
        let result = Cli::try_parse_from(["prog", "-m", "X", "--speed", "0"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_speed_negative() {
        let result = Cli::try_parse_from(["prog", "-m", "X", "--speed", "-1"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_speed_inf() {
        let result = Cli::try_parse_from(["prog", "-m", "X", "--speed", "inf"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_speed_nan() {
        let result = Cli::try_parse_from(["prog", "-m", "X", "--speed", "NaN"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_speed() {
        let cli = Cli::parse_from(["prog", "-m", "X", "--speed", "0.5"]);
        assert!((cli.speed - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_valid_hex_colors() {
        let cli = Cli::parse_from(["prog", "-m", "X", "--color", "#ff00ff"]);
        assert_eq!(cli.color, "#ff00ff");
        let cli = Cli::parse_from(["prog", "-m", "X", "--color", "#AABB00"]);
        assert_eq!(cli.color, "#AABB00");
        let cli = Cli::parse_from(["prog", "-m", "X", "--color", "#123456"]);
        assert_eq!(cli.color, "#123456");
    }
}
