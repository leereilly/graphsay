import { describe, it, expect } from "vitest";
import { renderSvg } from "../src/svg.js";
import { computeScrollAnimation } from "../src/animation.js";
import { generateGrid, NUM_COLS, NUM_ROWS } from "../src/grid.js";
import { GITHUB_LIGHT, GITHUB_DARK } from "../src/palette.js";
import { renderText } from "../src/text.js";

function makeTestSvg(dark: boolean): string {
  const grid = generateGrid();
  const sprite = renderText("HI");
  const result = computeScrollAnimation(
    grid,
    sprite,
    GITHUB_LIGHT,
    "#40c463",
    0.15,
  );
  return renderSvg(
    grid,
    result.keyframes,
    GITHUB_LIGHT,
    dark ? GITHUB_DARK : null,
    result.duration,
    11,
    3,
  );
}

describe("svg", () => {
  it("starts and ends correctly", () => {
    const svg = makeTestSvg(false);
    expect(svg.startsWith("<svg")).toBe(true);
    expect(svg.endsWith("</svg>")).toBe(true);
  });

  it("has correct dimensions", () => {
    const svg = makeTestSvg(false);
    const expectedW = NUM_COLS * 14 - 3;
    const expectedH = NUM_ROWS * 14 - 3;
    expect(svg).toContain(`width="${expectedW}"`);
    expect(svg).toContain(`height="${expectedH}"`);
  });

  it("contains rect elements", () => {
    const svg = makeTestSvg(false);
    expect(svg).toContain("<rect");
  });

  it("dark mode includes style and media query", () => {
    const svg = makeTestSvg(true);
    expect(svg).toContain("<style>");
    expect(svg).toContain("prefers-color-scheme: dark");
  });

  it("animated cells have animate elements", () => {
    const svg = makeTestSvg(false);
    expect(svg).toContain("<animate");
  });

  it("keyTimes are properly formatted", () => {
    const svg = makeTestSvg(false);
    const match = svg.match(/keyTimes="([^"]+)"/);
    if (match) {
      const parts = match[1].split(";");
      for (const part of parts) {
        expect(part).toContain(".");
      }
    }
  });

  it("contains palette gray color for light theme", () => {
    const svg = makeTestSvg(false);
    expect(svg).toContain(GITHUB_LIGHT.colors[0]);
  });

  it("both-theme SVG uses CSS variables", () => {
    // Use "." which only covers row 6, leaving most cells without keyframes
    const grid = generateGrid();
    const sprite = renderText(".");
    const result = computeScrollAnimation(
      grid,
      sprite,
      GITHUB_LIGHT,
      "#40c463",
      0.15,
    );
    const svg = renderSvg(
      grid,
      result.keyframes,
      GITHUB_LIGHT,
      GITHUB_DARK,
      result.duration,
      11,
      3,
    );
    expect(svg).toContain("var(--c");
    expect(svg).not.toContain("var(--bg)");
  });
});
