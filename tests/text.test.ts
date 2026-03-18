import { describe, it, expect } from "vitest";
import { renderText } from "../src/text.js";

describe("text", () => {
  it("empty string produces empty sprite", () => {
    const sprite = renderText("");
    expect(sprite.pixels).toHaveLength(0);
    expect(sprite.width).toBe(0);
  });

  it("single char I has correct width and pixel count", () => {
    const sprite = renderText("I");
    // I glyph: ["###", ".#.", ".#.", ".#.", ".#.", ".#.", "###"]
    // width 3, pixels: row 0 has 3, rows 1-5 have 1 each, row 6 has 3 = 11
    expect(sprite.width).toBe(3);
    expect(sprite.pixels).toHaveLength(11);
  });

  it("multi char spacing is correct", () => {
    const sprite = renderText("HI");
    // H: width 5, I: width 3
    // Total width: 5 + 1 + 3 = 9
    expect(sprite.width).toBe(9);
  });

  it("A has correct pixel count", () => {
    const sprite = renderText("A");
    // 3 + 2 + 2 + 5 + 2 + 2 + 2 = 18
    expect(sprite.pixels).toHaveLength(18);
    expect(sprite.width).toBe(5);
  });

  it("unknown chars are skipped", () => {
    const sprite = renderText("\u00e9");
    expect(sprite.pixels).toHaveLength(0);
    expect(sprite.width).toBe(0);
  });

  it("pixels have correct y range (0-6)", () => {
    const sprite = renderText("HELLO WORLD");
    for (const p of sprite.pixels) {
      expect(p.y).toBeGreaterThanOrEqual(0);
      expect(p.y).toBeLessThanOrEqual(6);
    }
  });
});
