import { execFileSync } from "node:child_process";
import * as path from "node:path";

export type FfmpegFormat = "gif" | "webp" | "mp4";

export function checkFfmpeg(): void {
  try {
    execFileSync("ffmpeg", ["-version"], { stdio: "pipe" });
  } catch {
    throw new Error(
      "ffmpeg not found. Please install ffmpeg to generate GIF/WebP/MP4 output.",
    );
  }
}

export function ffmpegExport(
  framesDir: string,
  outputPath: string,
  format: FfmpegFormat,
  fps: number,
  _duration: number,
): void {
  const inputPattern = path.join(framesDir, "frame_%05d.png");

  let args: string[];
  switch (format) {
    case "gif":
      args = [
        "-y",
        "-framerate", String(fps),
        "-i", inputPattern,
        "-vf", "split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse",
        outputPath,
      ];
      break;
    case "webp":
      args = [
        "-y",
        "-framerate", String(fps),
        "-i", inputPattern,
        "-vcodec", "libwebp",
        "-lossless", "1",
        "-loop", "0",
        outputPath,
      ];
      break;
    case "mp4":
      args = [
        "-y",
        "-framerate", String(fps),
        "-i", inputPattern,
        "-vf", "pad=ceil(iw/2)*2:ceil(ih/2)*2",
        "-c:v", "libx264",
        "-pix_fmt", "yuv420p",
        "-movflags", "+faststart",
        outputPath,
      ];
      break;
  }

  try {
    execFileSync("ffmpeg", args, { stdio: "pipe" });
  } catch (err: unknown) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`ffmpeg failed: ${msg}`);
  }
}

export function buildFfmpegArgs(
  framesDir: string,
  outputPath: string,
  format: FfmpegFormat,
  fps: number,
): string[] {
  const inputPattern = path.join(framesDir, "frame_%05d.png");

  switch (format) {
    case "gif":
      return [
        "-y",
        "-framerate", String(fps),
        "-i", inputPattern,
        "-vf", "split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse",
        outputPath,
      ];
    case "webp":
      return [
        "-y",
        "-framerate", String(fps),
        "-i", inputPattern,
        "-vcodec", "libwebp",
        "-lossless", "1",
        "-loop", "0",
        outputPath,
      ];
    case "mp4":
      return [
        "-y",
        "-framerate", String(fps),
        "-i", inputPattern,
        "-vf", "pad=ceil(iw/2)*2:ceil(ih/2)*2",
        "-c:v", "libx264",
        "-pix_fmt", "yuv420p",
        "-movflags", "+faststart",
        outputPath,
      ];
  }
}
