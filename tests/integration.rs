use std::process::Command;

fn cargo_bin() -> String {
    let output = Command::new("cargo")
        .args(["build", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to build");
    assert!(output.status.success(), "Build failed: {}", String::from_utf8_lossy(&output.stderr));

    // Return path to binary
    let target_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("target/debug/animated-contribution-graph-message");
    target_dir.to_str().unwrap().to_string()
}

#[test]
fn test_svg_light_output() {
    let bin = cargo_bin();
    let output_path = "/tmp/test_contribuart_light.svg";

    let output = Command::new(&bin)
        .args(["--message", "TEST", "--format", "svg", "--theme", "light", "-o", output_path])
        .output()
        .expect("Failed to run binary");

    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let content = std::fs::read_to_string(output_path).expect("Output file not created");
    assert!(content.starts_with("<svg"), "Should be valid SVG");
    assert!(content.ends_with("</svg>"), "Should end with </svg>");

    std::fs::remove_file(output_path).ok();
}

#[test]
fn test_svg_both_has_dark_mode() {
    let bin = cargo_bin();
    let output_path = "/tmp/test_contribuart_both.svg";

    let output = Command::new(&bin)
        .args(["--message", "HI", "--format", "svg", "--theme", "both", "-o", output_path])
        .output()
        .expect("Failed to run binary");

    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let content = std::fs::read_to_string(output_path).expect("Output file not created");
    assert!(content.contains("prefers-color-scheme: dark"), "Should have dark mode CSS");
    assert!(content.contains("<style>"), "Should have style block");

    std::fs::remove_file(output_path).ok();
}

#[test]
fn test_static_mode() {
    let bin = cargo_bin();
    let output_path = "/tmp/test_contribuart_static.svg";

    let output = Command::new(&bin)
        .args(["--message", "OK", "--format", "svg", "--mode", "static", "-o", output_path])
        .output()
        .expect("Failed to run binary");

    assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));
    let content = std::fs::read_to_string(output_path).expect("Output file not created");
    assert!(content.contains("<svg"), "Should be valid SVG");

    std::fs::remove_file(output_path).ok();
}

#[test]
fn test_missing_message_error() {
    let bin = cargo_bin();
    let output = Command::new(&bin)
        .output()
        .expect("Failed to run binary");

    assert!(!output.status.success(), "Should fail without --message");
}

#[test]
fn test_invalid_color_error() {
    let bin = cargo_bin();
    let output = Command::new(&bin)
        .args(["--message", "X", "--color", "notacolor"])
        .output()
        .expect("Failed to run binary");

    assert!(!output.status.success(), "Should fail with invalid color");
}
