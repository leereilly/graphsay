use std::path::Path;
use std::process::Command;

use crate::cli::Format;

pub fn check_ffmpeg() -> Result<(), String> {
    match Command::new("ffmpeg").arg("-version").output() {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                Err("ffmpeg is installed but returned an error".to_string())
            }
        }
        Err(_) => Err("ffmpeg not found. Please install ffmpeg to generate GIF/WebP/MP4 output.".to_string()),
    }
}

pub fn export(frames_dir: &Path, output: &Path, format: &Format, fps: u32, _duration: f64) -> Result<(), String> {
    let input_pattern = frames_dir.join("frame_%05d.png");
    let input_str = input_pattern.to_str().ok_or("Invalid path")?;
    let output_str = output.to_str().ok_or("Invalid output path")?;

    let status = match format {
        Format::Gif => {
            Command::new("ffmpeg")
                .args([
                    "-y",
                    "-framerate", &fps.to_string(),
                    "-i", input_str,
                    "-vf", "split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse",
                    output_str,
                ])
                .output()
                .map_err(|e| format!("Failed to run ffmpeg: {}", e))?
        }
        Format::Webp => {
            Command::new("ffmpeg")
                .args([
                    "-y",
                    "-framerate", &fps.to_string(),
                    "-i", input_str,
                    "-vcodec", "libwebp",
                    "-lossless", "1",
                    "-loop", "0",
                    output_str,
                ])
                .output()
                .map_err(|e| format!("Failed to run ffmpeg: {}", e))?
        }
        Format::Mp4 => {
            Command::new("ffmpeg")
                .args([
                    "-y",
                    "-framerate", &fps.to_string(),
                    "-i", input_str,
                    "-vf", "pad=ceil(iw/2)*2:ceil(ih/2)*2",
                    "-c:v", "libx264",
                    "-pix_fmt", "yuv420p",
                    "-movflags", "+faststart",
                    output_str,
                ])
                .output()
                .map_err(|e| format!("Failed to run ffmpeg: {}", e))?
        }
        Format::Svg => {
            return Err("SVG format should not use ffmpeg export".to_string());
        }
    };

    if status.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&status.stderr);
        Err(format!("ffmpeg failed: {}", stderr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_ffmpeg_runs() {
        // This test just verifies the function doesn't panic
        // It may pass or fail depending on whether ffmpeg is installed
        let _result = check_ffmpeg();
    }
}
