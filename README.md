# Animated Contribution Graph Message

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="contribution-graph-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="contribution-graph-light.svg">
    <img alt="Animated contribution graph message" src="contribution-graph-light.svg">
  </picture>
</p>

Generate animated GitHub-style contribution graph messages as SVG, GIF, WebP, or MP4.

## Installation

### Prerequisites

- [Node.js](https://nodejs.org/) v18 or later
- [ffmpeg](https://ffmpeg.org/) (only needed for GIF, WebP, and MP4 output)

### Run with npx (no install needed)

```bash
npx animated-contribution-graph-message -m "HELLO WORLD" -f svg
```

### Install globally

```bash
npm install -g animated-contribution-graph-message
```

### Install from source

```bash
git clone https://github.com/leereilly/contribuart.git
cd contribuart/animated-contribution-graph-message
npm install
npm run build
```

## Usage

```bash
npx animated-contribution-graph-message --message "HELLO WORLD" --format svg
```

### Options

| Flag | Description | Default |
|------|-------------|---------|
| `-m, --message <TEXT>` | Message to display (required, max 500 chars) | — |
| `-t, --theme <THEME>` | `light`, `dark`, or `both` | `both` |
| `-f, --format <FORMAT>` | `svg`, `gif`, `webp`, `mp4` | `mp4` |
| `-o, --output <PATH>` | Output file path | auto-generated |
| `--color <HEX>` | Text color hex code | `#40c463` |
| `--mode <MODE>` | `scroll` or `static` | `scroll` |
| `--speed <SECONDS>` | Step interval in seconds | `0.15` |
| `--no-transparent` | Add a background color instead of transparent | transparent |
| `--no-loop` | Disable looping (SVG plays once and freezes; GIF does not loop) | loops |

### Examples

```bash
# SVG with light theme (scroll)
npx animated-contribution-graph-message -m "HELLO WORLD" -f svg -t light
```

![Light theme scroll](examples/hello-world-light-scroll.gif)

```bash
# SVG with both light/dark mode support
npx animated-contribution-graph-message -m "HELLO WORLD" -f svg -t both
```

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="examples/hello-dark-scroll.gif">
  <source media="(prefers-color-scheme: light)" srcset="examples/hello-light-scroll.gif">
  <img alt="Both themes scroll" src="examples/hello-light-scroll.gif">
</picture>

```bash
# Static centered text (dark theme)
npx animated-contribution-graph-message -m "HELLO WORLD" -f svg -t dark --mode static
```

![Dark theme static](examples/hello-world-dark-static.gif)

```bash
# Custom color
npx animated-contribution-graph-message -m "RUST" -f svg --color "#ff6600"
```

![Custom color](examples/rust-custom-color.gif)

```bash
# GIF output (requires ffmpeg)
npx animated-contribution-graph-message -m "HELLO" -f gif -t light
```

![GIF example](examples/hello-light-scroll.gif)

```bash
# With background color (non-transparent)
npx animated-contribution-graph-message -m "HELLO WORLD" -f svg -t dark --no-transparent
```

![Non-transparent dark](examples/hello-world-dark-no-transparent.gif)

```bash
# MP4 video (requires ffmpeg)
npx animated-contribution-graph-message -m "HELLO" -f mp4
```

➡️ [Output](examples/hello-mp4.mp4)

### Requirements

- **SVG output**: No additional dependencies
- **GIF/WebP/MP4 output**: Requires [ffmpeg](https://ffmpeg.org/) to be installed and available on PATH

## Testing

```bash
npm test
```

## License

[MIT](LICENSE)
