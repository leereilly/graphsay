# Animated Contribution Graph Message

Generate animated GitHub-style contribution graph messages as SVG, GIF, WebP, or MP4.

## Installation

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (install via `rustup`)
- [ffmpeg](https://ffmpeg.org/) (only needed for GIF, WebP, and MP4 output)

### Install from source

```bash
git clone https://github.com/leereilly/contribuart.git
cd contribuart/animated-contribution-graph-message
cargo install --path .
```

### Build only

```bash
cargo build --release
```

The binary will be at `target/release/animated-contribution-graph-message`.

## Usage

```bash
animated-contribution-graph-message --message "HELLO WORLD" --format svg
```

### Options

| Flag | Description | Default |
|------|-------------|---------|
| `-m, --message <TEXT>` | Message to display (required) | — |
| `-t, --theme <THEME>` | `light`, `dark`, or `both` | `both` |
| `-f, --format <FORMAT>` | `svg`, `gif`, `webp`, `mp4` | `mp4` |
| `-o, --output <PATH>` | Output file path | auto-generated |
| `--color <HEX>` | Text color hex code | `#40c463` |
| `--mode <MODE>` | `scroll` or `static` | `scroll` |
| `--speed <SECONDS>` | Step interval in seconds | `0.15` |

### Examples

```bash
# SVG with light theme (scroll)
animated-contribution-graph-message -m "HELLO WORLD" -f svg -t light
```

![Light theme scroll](examples/hello-world-light-scroll.gif)

```bash
# SVG with both light/dark mode support
animated-contribution-graph-message -m "HELLO WORLD" -f svg -t both
```

![Both themes scroll](examples/hello-world-both-scroll.gif)

```bash
# Static centered text (dark theme)
animated-contribution-graph-message -m "HELLO WORLD" -f svg -t dark --mode static
```

![Dark theme static](examples/hello-world-dark-static.gif)

```bash
# Custom color
animated-contribution-graph-message -m "RUST" -f svg --color "#ff6600"
```

![Custom color](examples/rust-custom-color.gif)

```bash
# GIF output (requires ffmpeg)
animated-contribution-graph-message -m "HELLO" -f gif -t light
```

![GIF example](examples/hello-light-scroll.gif)

```bash
# MP4 video (requires ffmpeg)
animated-contribution-graph-message -m "HELLO" -f mp4
```

### Requirements

- **SVG output**: No additional dependencies
- **GIF/WebP/MP4 output**: Requires [ffmpeg](https://ffmpeg.org/) to be installed

## Testing

```bash
cargo test
```
