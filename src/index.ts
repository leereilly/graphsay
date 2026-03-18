import { Command } from "commander";
import * as fs from "node:fs";
import * as path from "node:path";
import * as os from "node:os";

import { generateGrid } from "./grid.js";
import { renderText } from "./text.js";
import { GITHUB_LIGHT, GITHUB_DARK, Palette } from "./palette.js";
import {
  computeScrollAnimation,
  computeStaticAnimation,
} from "./animation.js";
import { renderSvg } from "./svg.js";
import { renderFramesToDisk } from "./render.js";
import { checkFfmpeg, ffmpegExport, FfmpegFormat } from "./ffmpeg.js";

export type Theme = "light" | "dark" | "both";
export type Format = "svg" | "gif" | "webp" | "mp4";
export type Mode = "scroll" | "static";

export interface CliOptions {
  message: string;
  theme: Theme;
  format: Format;
  output?: string;
  color: string;
  mode: Mode;
  speed: number;
}

function formatExtension(fmt: Format): string {
  return fmt;
}

function validateHexColor(value: string): string {
  if (!/^#[0-9a-fA-F]{6}$/.test(value)) {
    throw new Error(
      `Invalid hex color '${value}'. Must match #[0-9a-fA-F]{6}`,
    );
  }
  return value;
}

function validateSpeed(value: string): number {
  const v = parseFloat(value);
  if (!isFinite(v) || v <= 0) {
    throw new Error(
      `Speed must be a finite number greater than 0, got '${value}'`,
    );
  }
  return v;
}

function validateMessage(value: string): string {
  if (value.length > 500) {
    throw new Error(`Message too long (${value.length} chars, max 500)`);
  }
  return value;
}

export function parseCli(argv: string[]): CliOptions {
  const program = new Command();
  program
    .name("animated-contribution-graph-message")
    .description("Generate animated GitHub contribution graph messages")
    .requiredOption(
      "-m, --message <text>",
      "Message to display (max 500 characters)",
      validateMessage,
    )
    .option("-t, --theme <theme>", "Theme: light, dark, or both", "both")
    .option(
      "-f, --format <format>",
      "Output format: svg, gif, webp, mp4",
      "mp4",
    )
    .option("-o, --output <path>", "Output file path")
    .option(
      "--color <hex>",
      "Text color hex code",
      validateHexColor,
      "#40c463",
    )
    .option("--mode <mode>", "Animation mode: scroll or static", "scroll")
    .option(
      "--speed <seconds>",
      "Step interval in seconds (must be > 0)",
      validateSpeed,
      0.15,
    );

  program.parse(argv);
  const opts = program.opts();

  // Validate enum values
  const validThemes: Theme[] = ["light", "dark", "both"];
  const validFormats: Format[] = ["svg", "gif", "webp", "mp4"];
  const validModes: Mode[] = ["scroll", "static"];

  if (!validThemes.includes(opts.theme)) {
    throw new Error(
      `Invalid theme '${opts.theme}'. Must be one of: light, dark, both`,
    );
  }
  if (!validFormats.includes(opts.format)) {
    throw new Error(
      `Invalid format '${opts.format}'. Must be one of: svg, gif, webp, mp4`,
    );
  }
  if (!validModes.includes(opts.mode)) {
    throw new Error(
      `Invalid mode '${opts.mode}'. Must be one of: scroll, static`,
    );
  }

  return {
    message: opts.message,
    theme: opts.theme as Theme,
    format: opts.format as Format,
    output: opts.output,
    color: opts.color,
    mode: opts.mode as Mode,
    speed: opts.speed,
  };
}

export async function run(args: CliOptions): Promise<void> {
  const grid = generateGrid();
  const sprite = renderText(args.message);

  if (sprite.width === 0) {
    throw new Error(
      "Message produced no renderable pixels. Check your message text.",
    );
  }

  const themes: {
    palette: Palette;
    darkPalette: Palette | null;
    themeName: string;
  }[] = [];

  switch (args.theme) {
    case "light":
      themes.push({
        palette: GITHUB_LIGHT,
        darkPalette: null,
        themeName: "light",
      });
      break;
    case "dark":
      themes.push({
        palette: GITHUB_DARK,
        darkPalette: null,
        themeName: "dark",
      });
      break;
    case "both":
      themes.push({
        palette: GITHUB_LIGHT,
        darkPalette: GITHUB_DARK,
        themeName: "both",
      });
      break;
  }

  const ext = formatExtension(args.format);

  for (const { palette, darkPalette, themeName } of themes) {
    const animResult =
      args.mode === "scroll"
        ? computeScrollAnimation(
            grid,
            sprite,
            palette,
            args.color,
            args.speed,
          )
        : computeStaticAnimation(grid, sprite, palette, args.color);

    let outputPath: string;
    if (args.output) {
      outputPath = args.output;
    } else if (themeName === "both") {
      outputPath = `contribution-graph.${ext}`;
    } else {
      outputPath = `contribution-graph-${themeName}.${ext}`;
    }

    if (args.format === "svg") {
      const svgContent = renderSvg(
        grid,
        animResult.keyframes,
        palette,
        darkPalette,
        animResult.duration,
        11,
        3,
      );
      fs.writeFileSync(outputPath, svgContent);
      console.log(`Created ${outputPath}`);
    } else {
      checkFfmpeg();

      const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "contrib-frames-"));
      try {
        await renderFramesToDisk(
          grid,
          animResult.keyframes,
          palette,
          animResult.duration,
          11,
          3,
          20,
          tmpDir,
        );

        ffmpegExport(
          tmpDir,
          outputPath,
          args.format as FfmpegFormat,
          20,
          animResult.duration,
        );

        console.log(`Created ${outputPath}`);
      } finally {
        fs.rmSync(tmpDir, { recursive: true, force: true });
      }
    }
  }
}
