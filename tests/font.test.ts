import { describe, it, expect } from "vitest";
import { getGlyph, glyphWidth } from "../src/font.js";

describe("font", () => {
  it("all printable ASCII 32-126 have glyphs", () => {
    for (let c = 32; c <= 126; c++) {
      const ch = String.fromCharCode(c);
      expect(getGlyph(ch), `Missing glyph for '${ch}' (${c})`).not.toBeNull();
    }
  });

  it("all glyphs have 7 rows", () => {
    for (let c = 32; c <= 126; c++) {
      const ch = String.fromCharCode(c);
      const glyph = getGlyph(ch);
      if (glyph) {
        expect(glyph).toHaveLength(7);
      }
    }
  });

  it("glyph widths are positive", () => {
    for (let c = 32; c <= 126; c++) {
      const ch = String.fromCharCode(c);
      const glyph = getGlyph(ch);
      if (glyph) {
        expect(glyphWidth(glyph)).toBeGreaterThan(0);
      }
    }
  });

  it("lowercase maps to uppercase", () => {
    for (let c = 97; c <= 122; c++) {
      const lower = String.fromCharCode(c);
      const upper = String.fromCharCode(c - 32);
      expect(getGlyph(lower)).toEqual(getGlyph(upper));
    }
  });

  it("unknown char returns null", () => {
    expect(getGlyph("\x7f")).toBeNull();
    expect(getGlyph("\u00e9")).toBeNull();
    expect(getGlyph("\u{1f600}")).toBeNull();
  });

  it("space glyph has width 3", () => {
    const glyph = getGlyph(" ");
    expect(glyph).not.toBeNull();
    expect(glyphWidth(glyph!)).toBe(3);
  });

  it("A glyph has width 5", () => {
    const glyph = getGlyph("A");
    expect(glyph).not.toBeNull();
    expect(glyphWidth(glyph!)).toBe(5);
  });

  it("I glyph has width 3", () => {
    const glyph = getGlyph("I");
    expect(glyph).not.toBeNull();
    expect(glyphWidth(glyph!)).toBe(3);
  });
});
