import { AnimationKeyframe } from "./animation.js";
import { Grid, NUM_COLS, NUM_ROWS } from "./grid.js";
import { Palette } from "./palette.js";

function groupKeyframesByCell(
  keyframes: AnimationKeyframe[],
): Map<string, AnimationKeyframe[]> {
  const map = new Map<string, AnimationKeyframe[]>();
  for (const kf of keyframes) {
    const key = `${kf.cellX},${kf.cellY}`;
    const arr = map.get(key);
    if (arr) {
      arr.push(kf);
    } else {
      map.set(key, [kf]);
    }
  }
  return map;
}

function colorAtTime(keyframes: AnimationKeyframe[], t: number): string {
  const base = keyframes[0].baseColor;
  for (const kf of keyframes) {
    if (kf.startTime <= t && t < kf.endTime) {
      return kf.highlightColor;
    }
  }
  return base;
}

function buildAnimateAttributes(
  keyframes: AnimationKeyframe[],
  _duration: number,
): { keyTimes: string; values: string } {
  const sorted = [...keyframes].sort((a, b) => a.startTime - b.startTime);
  const baseColor = sorted[0].baseColor;

  // Build timeline entries: [time, color]
  const entries: [number, string][] = [];
  entries.push([0.0, baseColor]);

  for (const kf of sorted) {
    entries.push([kf.startTime, kf.highlightColor]);
    entries.push([kf.endTime, baseColor]);
  }

  // Sort by time
  entries.sort((a, b) => a[0] - b[0]);

  // Deduplicate: keep last entry for same time
  const deduped: [number, string][] = [];
  for (const entry of entries) {
    if (deduped.length > 0) {
      const last = deduped[deduped.length - 1];
      if (Math.abs(last[0] - entry[0]) < 1e-10) {
        last[1] = entry[1];
        continue;
      }
    }
    deduped.push([entry[0], entry[1]]);
  }

  // Ensure we end at 1.0
  if (deduped.length > 0) {
    const last = deduped[deduped.length - 1];
    if (Math.abs(last[0] - 1.0) > 1e-10) {
      deduped.push([1.0, baseColor]);
    }
  }

  const keyTimes = deduped.map(([t]) => t.toFixed(4)).join(";");
  const values = deduped.map(([, c]) => c).join(";");

  return { keyTimes, values };
}

export function renderSvg(
  grid: Grid,
  keyframes: AnimationKeyframe[],
  palette: Palette,
  darkPalette: Palette | null,
  duration: number,
  cellSize: number,
  cellGap: number,
  transparent: boolean = true,
  loop: boolean = true,
): string {
  const pitch = cellSize + cellGap;
  const svgWidth = NUM_COLS * pitch - cellGap;
  const svgHeight = NUM_ROWS * pitch - cellGap;

  const grouped = groupKeyframesByCell(keyframes);

  let svg = "";
  svg += `<svg xmlns="http://www.w3.org/2000/svg" width="${svgWidth}" height="${svgHeight}" viewBox="0 0 ${svgWidth} ${svgHeight}">\n`;

  const lightBg = "#ffffff";
  const darkBg = "#0d1117";

  if (darkPalette) {
    svg += "<style>\n";
    svg += ":root {\n";
    if (!transparent) {
      svg += `  --bg: ${lightBg};\n`;
    }
    for (let i = 0; i < palette.colors.length; i++) {
      svg += `  --c${i}: ${palette.colors[i]};\n`;
    }
    svg += "}\n";
    svg += "@media (prefers-color-scheme: dark) {\n";
    svg += "  :root {\n";
    if (!transparent) {
      svg += `    --bg: ${darkBg};\n`;
    }
    for (let i = 0; i < darkPalette.colors.length; i++) {
      svg += `    --c${i}: ${darkPalette.colors[i]};\n`;
    }
    svg += "  }\n";
    svg += "}\n";
    svg += "</style>\n";
  }

  if (!transparent) {
    if (darkPalette) {
      svg += `  <rect x="0" y="0" width="${svgWidth}" height="${svgHeight}" fill="var(--bg)"/>\n`;
    } else {
      const bgColor = palette.name === "github-dark" ? darkBg : lightBg;
      svg += `  <rect x="0" y="0" width="${svgWidth}" height="${svgHeight}" fill="${bgColor}"/>\n`;
    }
  }

  for (let col = 0; col < NUM_COLS; col++) {
    for (let row = 0; row < NUM_ROWS; row++) {
      const cell = grid[col][row];
      if (cell === null) continue;

      const x = col * pitch;
      const y = row * pitch;
      const key = `${col},${row}`;
      const kfs = grouped.get(key);

      if (kfs && kfs.length > 0) {
        const isConstant = kfs.every(
          (kf) => kf.highlightColor === kf.baseColor,
        );
        const isFullDurationText =
          !isConstant &&
          kfs.every((kf) => kf.highlightColor === kfs[0].highlightColor) &&
          kfs.some((kf) => kf.startTime <= 0.0 && kf.endTime >= 1.0);

        if (isConstant) {
          const color = kfs[0].baseColor;
          svg += `  <rect x="${x}" y="${y}" width="${cellSize}" height="${cellSize}" rx="2" ry="2" fill="${color}"/>\n`;
        } else if (isFullDurationText) {
          const color = kfs[0].highlightColor;
          svg += `  <rect x="${x}" y="${y}" width="${cellSize}" height="${cellSize}" rx="2" ry="2" fill="${color}"/>\n`;
        } else {
          const midpointColor = colorAtTime(kfs, 0.5);
          svg += `  <rect x="${x}" y="${y}" width="${cellSize}" height="${cellSize}" rx="2" ry="2" fill="${midpointColor}">\n`;

          const { keyTimes, values } = buildAnimateAttributes(kfs, duration);
          const repeatCount = loop ? "indefinite" : "1";
          const fillAttr = loop ? "" : ' fill="freeze"';
          svg += `    <animate attributeName="fill" dur="${duration}s" repeatCount="${repeatCount}"${fillAttr} keyTimes="${keyTimes}" values="${values}" calcMode="discrete"/>\n`;

          svg += "  </rect>\n";
        }
      } else {
        const level = cell.level;
        if (darkPalette) {
          svg += `  <rect x="${x}" y="${y}" width="${cellSize}" height="${cellSize}" rx="2" ry="2" fill="var(--c${level})"/>\n`;
        } else {
          const color = palette.colors[level];
          svg += `  <rect x="${x}" y="${y}" width="${cellSize}" height="${cellSize}" rx="2" ry="2" fill="${color}"/>\n`;
        }
      }
    }
  }

  svg += "</svg>";
  return svg;
}
