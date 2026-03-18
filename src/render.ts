import sharp from "sharp";
import * as fs from "node:fs";
import * as path from "node:path";

import { AnimationKeyframe } from "./animation.js";
import { Grid, NUM_COLS, NUM_ROWS } from "./grid.js";
import { Palette } from "./palette.js";

function parseHexColor(hex: string): [number, number, number] {
  const h = hex.startsWith("#") ? hex.slice(1) : hex;
  const r = parseInt(h.slice(0, 2), 16) || 0;
  const g = parseInt(h.slice(2, 4), 16) || 0;
  const b = parseInt(h.slice(4, 6), 16) || 0;
  return [r, g, b];
}

export const MAX_FRAMES = 6000;

export async function renderFramesToDisk(
  grid: Grid,
  keyframes: AnimationKeyframe[],
  palette: Palette,
  duration: number,
  cellSize: number,
  cellGap: number,
  fps: number,
  outputDir: string,
): Promise<number> {
  const pitch = cellSize + cellGap;
  const imgWidth = NUM_COLS * pitch - cellGap;
  const imgHeight = NUM_ROWS * pitch - cellGap;
  const totalFrames = Math.ceil(duration * fps);

  if (totalFrames > MAX_FRAMES) {
    throw new Error(
      `Animation would produce ${totalFrames} frames (max ${MAX_FRAMES}). Shorten your message or increase --speed.`,
    );
  }

  interface ParsedKf {
    cellX: number;
    cellY: number;
    startTime: number;
    endTime: number;
    highlightColor: [number, number, number];
  }

  const parsedKfs: ParsedKf[] = keyframes.map((kf) => ({
    cellX: kf.cellX,
    cellY: kf.cellY,
    startTime: kf.startTime,
    endTime: kf.endTime,
    highlightColor: parseHexColor(kf.highlightColor),
  }));

  const grayColor = parseHexColor(palette.colors[0]);
  const bgHex = palette.name === "github-dark" ? "#0d1117" : "#ffffff";
  const bgColor = parseHexColor(bgHex);

  const animatedCells = new Set<string>();
  for (const kf of parsedKfs) {
    animatedCells.add(`${kf.cellX},${kf.cellY}`);
  }

  // Pre-parse palette colors
  const paletteColors = palette.colors.map(parseHexColor);

  for (let frameIdx = 0; frameIdx < totalFrames; frameIdx++) {
    const t = totalFrames <= 1 ? 0.0 : frameIdx / totalFrames;

    // Create raw RGB buffer
    const buffer = Buffer.alloc(imgWidth * imgHeight * 3);

    // Fill with page background color
    for (let i = 0; i < imgWidth * imgHeight; i++) {
      buffer[i * 3] = bgColor[0];
      buffer[i * 3 + 1] = bgColor[1];
      buffer[i * 3 + 2] = bgColor[2];
    }

    // Draw grid cells by level color
    for (let col = 0; col < NUM_COLS; col++) {
      for (let row = 0; row < NUM_ROWS; row++) {
        const cell = grid[col][row];
        if (cell === null) continue;

        const color = paletteColors[cell.level];
        const px = col * pitch;
        const py = row * pitch;
        for (let dy = 0; dy < cellSize; dy++) {
          for (let dx = 0; dx < cellSize; dx++) {
            const ix = px + dx;
            const iy = py + dy;
            if (ix < imgWidth && iy < imgHeight) {
              const offset = (iy * imgWidth + ix) * 3;
              buffer[offset] = color[0];
              buffer[offset + 1] = color[1];
              buffer[offset + 2] = color[2];
            }
          }
        }
      }
    }

    // Gray-out all animated cells
    for (const key of animatedCells) {
      const [cx, cy] = key.split(",").map(Number);
      const px = cx * pitch;
      const py = cy * pitch;
      for (let dy = 0; dy < cellSize; dy++) {
        for (let dx = 0; dx < cellSize; dx++) {
          const ix = px + dx;
          const iy = py + dy;
          if (ix < imgWidth && iy < imgHeight) {
            const offset = (iy * imgWidth + ix) * 3;
            buffer[offset] = grayColor[0];
            buffer[offset + 1] = grayColor[1];
            buffer[offset + 2] = grayColor[2];
          }
        }
      }
    }

    // Paint active keyframe highlights
    for (const kf of parsedKfs) {
      if (kf.startTime <= t && t < kf.endTime) {
        const px = kf.cellX * pitch;
        const py = kf.cellY * pitch;
        for (let dy = 0; dy < cellSize; dy++) {
          for (let dx = 0; dx < cellSize; dx++) {
            const ix = px + dx;
            const iy = py + dy;
            if (ix < imgWidth && iy < imgHeight) {
              const offset = (iy * imgWidth + ix) * 3;
              buffer[offset] = kf.highlightColor[0];
              buffer[offset + 1] = kf.highlightColor[1];
              buffer[offset + 2] = kf.highlightColor[2];
            }
          }
        }
      }
    }

    const framePath = path.join(
      outputDir,
      `frame_${String(frameIdx + 1).padStart(5, "0")}.png`,
    );
    await sharp(buffer, {
      raw: { width: imgWidth, height: imgHeight, channels: 3 },
    })
      .png()
      .toFile(framePath);
  }

  return totalFrames;
}
