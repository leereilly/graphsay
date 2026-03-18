import { describe, it, expect } from "vitest";
import * as fs from "node:fs";
import * as path from "node:path";
import * as os from "node:os";
import { renderFramesToDisk, MAX_FRAMES } from "../src/render.js";
import { computeStaticAnimation } from "../src/animation.js";
import { generateGrid, NUM_COLS } from "../src/grid.js";
import { GITHUB_LIGHT } from "../src/palette.js";
import { renderText } from "../src/text.js";

describe("render", () => {
  it("MAX_FRAMES is 6000", () => {
    expect(MAX_FRAMES).toBe(6000);
  });

  it("renders frames to disk", async () => {
    const grid = generateGrid();
    const sprite = renderText("A");
    const result = computeStaticAnimation(
      grid,
      sprite,
      GITHUB_LIGHT,
      "#40c463",
    );

    const tmpDir = fs.mkdtempSync(
      path.join(os.tmpdir(), "render-test-"),
    );
    try {
      const count = await renderFramesToDisk(
        grid,
        result.keyframes,
        GITHUB_LIGHT,
        result.duration,
        11,
        3,
        20,
        tmpDir,
      );

      const expectedFrames = Math.ceil(result.duration * 20);
      expect(count).toBe(expectedFrames);

      // Check first frame exists
      const firstFrame = path.join(tmpDir, "frame_00001.png");
      expect(fs.existsSync(firstFrame)).toBe(true);

      // Check last frame exists
      const lastFrame = path.join(
        tmpDir,
        `frame_${String(count).padStart(5, "0")}.png`,
      );
      expect(fs.existsSync(lastFrame)).toBe(true);
    } finally {
      fs.rmSync(tmpDir, { recursive: true, force: true });
    }
  });

  it("rejects when too many frames", async () => {
    const grid = generateGrid();
    const sprite = renderText("A");
    const result = computeStaticAnimation(
      grid,
      sprite,
      GITHUB_LIGHT,
      "#40c463",
    );

    const tmpDir = fs.mkdtempSync(
      path.join(os.tmpdir(), "render-test-"),
    );
    try {
      await expect(
        renderFramesToDisk(
          grid,
          result.keyframes,
          GITHUB_LIGHT,
          result.duration,
          11,
          3,
          999999, // absurd fps to exceed MAX_FRAMES
          tmpDir,
        ),
      ).rejects.toThrow("max");
    } finally {
      fs.rmSync(tmpDir, { recursive: true, force: true });
    }
  });
});
