import { Grid, NUM_COLS, NUM_ROWS } from "./grid.js";
import { Palette } from "./palette.js";
import { TextSprite } from "./text.js";

export interface AnimationKeyframe {
  cellX: number;
  cellY: number;
  startTime: number;
  endTime: number;
  baseColor: string;
  highlightColor: string;
  beginOffset: number;
}

export interface AnimationResult {
  keyframes: AnimationKeyframe[];
  duration: number;
}

export function computeScrollAnimation(
  grid: Grid,
  sprite: TextSprite,
  palette: Palette,
  textColor: string,
  speed: number,
): AnimationResult {
  const grayColor = palette.colors[0];
  const totalSteps = NUM_COLS + sprite.width + 1;
  const duration = totalSteps * speed;
  const keyframes: AnimationKeyframe[] = [];

  for (let step = 0; step < totalSteps; step++) {
    const spriteLeft = NUM_COLS - step;
    const startFrac = step / totalSteps;
    const endFrac = (step + 1) / totalSteps;

    for (const pixel of sprite.pixels) {
      const gx = pixel.x + spriteLeft;
      const gy = pixel.y;

      if (gx < 0 || gx >= NUM_COLS) continue;
      if (gy >= NUM_ROWS) continue;

      const col = gx;
      if (grid[col][gy] === null) continue;

      keyframes.push({
        cellX: col,
        cellY: gy,
        startTime: startFrac,
        endTime: endFrac,
        baseColor: grayColor,
        highlightColor: textColor,
        beginOffset: step * speed,
      });
    }
  }

  // Track which cells got text keyframes
  const animatedCells = new Set<string>();
  for (const kf of keyframes) {
    animatedCells.add(`${kf.cellX},${kf.cellY}`);
  }

  // Force all remaining non-null, non-level-0 cells to gray
  for (let col = 0; col < NUM_COLS; col++) {
    for (let row = 0; row < NUM_ROWS; row++) {
      const cell = grid[col][row];
      if (cell === null) continue;
      if (cell.level === 0) continue;
      if (animatedCells.has(`${col},${row}`)) continue;

      keyframes.push({
        cellX: col,
        cellY: row,
        startTime: 0.0,
        endTime: 1.0,
        baseColor: grayColor,
        highlightColor: grayColor,
        beginOffset: 0.0,
      });
    }
  }

  return { keyframes, duration };
}

export function computeStaticAnimation(
  grid: Grid,
  sprite: TextSprite,
  palette: Palette,
  textColor: string,
): AnimationResult {
  const grayColor = palette.colors[0];
  const keyframes: AnimationKeyframe[] = [];
  const duration = 3.0;

  const offsetX =
    sprite.width < NUM_COLS
      ? Math.floor((NUM_COLS - sprite.width) / 2)
      : 0;

  for (const pixel of sprite.pixels) {
    const gx = pixel.x + offsetX;
    const gy = pixel.y;

    if (gx >= NUM_COLS || gy >= NUM_ROWS) continue;
    if (grid[gx][gy] === null) continue;

    keyframes.push({
      cellX: gx,
      cellY: gy,
      startTime: 0.0,
      endTime: 1.0,
      baseColor: grayColor,
      highlightColor: textColor,
      beginOffset: 0.0,
    });
  }

  // Track which cells got text keyframes
  const animatedCells = new Set<string>();
  for (const kf of keyframes) {
    animatedCells.add(`${kf.cellX},${kf.cellY}`);
  }

  // Force all remaining non-null, non-level-0 cells to gray
  for (let col = 0; col < NUM_COLS; col++) {
    for (let row = 0; row < NUM_ROWS; row++) {
      const cell = grid[col][row];
      if (cell === null) continue;
      if (cell.level === 0) continue;
      if (animatedCells.has(`${col},${row}`)) continue;

      keyframes.push({
        cellX: col,
        cellY: row,
        startTime: 0.0,
        endTime: 1.0,
        baseColor: grayColor,
        highlightColor: grayColor,
        beginOffset: 0.0,
      });
    }
  }

  return { keyframes, duration };
}
