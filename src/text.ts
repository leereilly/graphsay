import { getGlyph, glyphWidth } from "./font.js";

export interface GridPoint {
  x: number;
  y: number;
}

export interface TextSprite {
  pixels: GridPoint[];
  width: number;
}

export function renderText(message: string): TextSprite {
  const pixels: GridPoint[] = [];
  let cursorX = 0;
  let anyChar = false;

  for (const ch of message) {
    const glyph = getGlyph(ch);
    if (glyph === null) continue;

    const w = glyphWidth(glyph);
    for (let rowIdx = 0; rowIdx < glyph.length; rowIdx++) {
      const row = glyph[rowIdx];
      for (let colIdx = 0; colIdx < row.length; colIdx++) {
        if (row[colIdx] === "#") {
          pixels.push({ x: cursorX + colIdx, y: rowIdx });
        }
      }
    }
    cursorX += w + 1;
    anyChar = true;
  }

  const width = anyChar && cursorX > 0 ? cursorX - 1 : 0;
  return { pixels, width };
}
