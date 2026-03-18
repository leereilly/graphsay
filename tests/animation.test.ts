import { describe, it, expect } from "vitest";
import {
  computeScrollAnimation,
  computeStaticAnimation,
} from "../src/animation.js";
import { generateGrid, NUM_COLS, NUM_ROWS } from "../src/grid.js";
import { GITHUB_LIGHT } from "../src/palette.js";
import { renderText } from "../src/text.js";

describe("animation", () => {
  it("scroll keyframes are non-empty", () => {
    const grid = generateGrid();
    const sprite = renderText("HI");
    const result = computeScrollAnimation(
      grid,
      sprite,
      GITHUB_LIGHT,
      "#40c463",
      0.15,
    );
    expect(result.keyframes.length).toBeGreaterThan(0);
  });

  it("scroll times are in [0, 1]", () => {
    const grid = generateGrid();
    const sprite = renderText("A");
    const result = computeScrollAnimation(
      grid,
      sprite,
      GITHUB_LIGHT,
      "#40c463",
      0.15,
    );
    for (const kf of result.keyframes) {
      expect(kf.startTime).toBeGreaterThanOrEqual(0.0);
      expect(kf.startTime).toBeLessThanOrEqual(1.0);
      expect(kf.endTime).toBeGreaterThanOrEqual(0.0);
      expect(kf.endTime).toBeLessThanOrEqual(1.0);
    }
  });

  it("scroll cell coords are in bounds", () => {
    const grid = generateGrid();
    const sprite = renderText("HELLO");
    const result = computeScrollAnimation(
      grid,
      sprite,
      GITHUB_LIGHT,
      "#40c463",
      0.15,
    );
    for (const kf of result.keyframes) {
      expect(kf.cellX).toBeLessThan(NUM_COLS);
      expect(kf.cellY).toBeLessThan(NUM_ROWS);
    }
  });

  it("scroll forces non-text cells to gray", () => {
    const grid = generateGrid();
    const sprite = renderText(".");
    const result = computeScrollAnimation(
      grid,
      sprite,
      GITHUB_LIGHT,
      "#40c463",
      0.15,
    );
    const grayKfs = result.keyframes.filter(
      (kf) =>
        kf.highlightColor === kf.baseColor &&
        kf.baseColor === GITHUB_LIGHT.colors[0],
    );
    expect(grayKfs.length).toBeGreaterThan(0);
  });

  it("scroll duration matches formula", () => {
    const grid = generateGrid();
    const sprite = renderText("HI");
    const speed = 0.15;
    const result = computeScrollAnimation(
      grid,
      sprite,
      GITHUB_LIGHT,
      "#40c463",
      speed,
    );
    const totalSteps = NUM_COLS + sprite.width + 1;
    const expected = totalSteps * speed;
    expect(Math.abs(result.duration - expected)).toBeLessThan(0.001);
  });

  it("static text pixels get highlight color", () => {
    const grid = generateGrid();
    const sprite = renderText("A");
    const result = computeStaticAnimation(
      grid,
      sprite,
      GITHUB_LIGHT,
      "#ff0000",
    );
    const textKfs = result.keyframes.filter(
      (kf) => kf.highlightColor !== kf.baseColor,
    );
    expect(textKfs.length).toBeGreaterThan(0);
    for (const kf of textKfs) {
      expect(kf.highlightColor).toBe("#ff0000");
    }
  });

  it("static forces non-text cells to gray", () => {
    const grid = generateGrid();
    const sprite = renderText("A");
    const result = computeStaticAnimation(
      grid,
      sprite,
      GITHUB_LIGHT,
      "#ff0000",
    );
    const grayKfs = result.keyframes.filter(
      (kf) => kf.highlightColor === kf.baseColor,
    );
    expect(grayKfs.length).toBeGreaterThan(0);
    for (const kf of grayKfs) {
      expect(kf.baseColor).toBe(GITHUB_LIGHT.colors[0]);
    }
  });

  it("static uses gray base color", () => {
    const grid = generateGrid();
    const sprite = renderText("A");
    const result = computeStaticAnimation(
      grid,
      sprite,
      GITHUB_LIGHT,
      "#ff0000",
    );
    for (const kf of result.keyframes) {
      expect(kf.baseColor).toBe(GITHUB_LIGHT.colors[0]);
    }
  });

  it("static duration is 3.0", () => {
    const grid = generateGrid();
    const sprite = renderText("A");
    const result = computeStaticAnimation(
      grid,
      sprite,
      GITHUB_LIGHT,
      "#ff0000",
    );
    expect(result.duration).toBe(3.0);
  });
});
