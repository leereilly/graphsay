import { describe, it, expect } from "vitest";
import { GITHUB_LIGHT, GITHUB_DARK } from "../src/palette.js";

describe("palette", () => {
  it("light palette has 5 colors", () => {
    expect(GITHUB_LIGHT.colors).toHaveLength(5);
  });

  it("dark palette has 5 colors", () => {
    expect(GITHUB_DARK.colors).toHaveLength(5);
  });

  it("colors are valid hex", () => {
    for (const palette of [GITHUB_LIGHT, GITHUB_DARK]) {
      for (const color of palette.colors) {
        expect(color).toMatch(/^#[0-9a-f]{6}$/i);
      }
    }
  });

  it("light palette level-0 is gray", () => {
    expect(GITHUB_LIGHT.colors[0]).toBe("#ebedf0");
  });

  it("dark palette level-0 is gray", () => {
    expect(GITHUB_DARK.colors[0]).toBe("#161b22");
  });
});
