import { describe, it, expect } from "vitest";
import { renderSvg } from "../src/svg.js";
import { computeScrollAnimation } from "../src/animation.js";
import { generateGrid, NUM_COLS, NUM_ROWS } from "../src/grid.js";
import { GITHUB_LIGHT, GITHUB_DARK } from "../src/palette.js";
import { renderText } from "../src/text.js";

function makeTestSvg(dark: boolean, transparent: boolean = true, loop: boolean = true): string {
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
    transparent,
    loop,
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
      true,
    );
    expect(svg).toContain("var(--c");
    expect(svg).not.toContain("var(--bg)");
  });

  it("transparent true does not include background rect", () => {
    const svg = makeTestSvg(false, true);
    // Should not have a full-width background rect
    const svgWidth = NUM_COLS * 14 - 3;
    const svgHeight = NUM_ROWS * 14 - 3;
    expect(svg).not.toContain(`width="${svgWidth}" height="${svgHeight}" fill=`);
  });

  it("transparent false includes background rect for light theme", () => {
    const svg = makeTestSvg(false, false);
    expect(svg).toContain('fill="#ffffff"');
    expect(svg).toContain('x="0" y="0"');
  });

  it("transparent false with both theme uses CSS variable for background", () => {
    const svg = makeTestSvg(true, false);
    expect(svg).toContain("var(--bg)");
    expect(svg).toContain("--bg: #ffffff");
    expect(svg).toContain("--bg: #0d1117");
  });

  it("loop true uses repeatCount indefinite and no fill freeze", () => {
    const svg = makeTestSvg(false, true, true);
    expect(svg).toContain('repeatCount="indefinite"');
    expect(svg).not.toContain('fill="freeze"');
  });

  it("loop false uses repeatCount 1 and fill freeze", () => {
    const svg = makeTestSvg(false, true, false);
    expect(svg).toContain('repeatCount="1"');
    expect(svg).toContain('fill="freeze"');
    expect(svg).not.toContain('repeatCount="indefinite"');
  });

  it("default loop behavior is indefinite", () => {
    const svg = makeTestSvg(false);
    expect(svg).toContain('repeatCount="indefinite"');
  });
});
