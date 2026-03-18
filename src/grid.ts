export const NUM_COLS = 53;
export const NUM_ROWS = 7;

export interface Cell {
  col: number;
  row: number;
  level: number;
}

export type Grid = (Cell | null)[][];

export function generateGrid(rng: () => number = Math.random): Grid {
  const grid: Grid = [];
  for (let col = 0; col < NUM_COLS; col++) {
    const column: (Cell | null)[] = [];
    for (let row = 0; row < NUM_ROWS; row++) {
      const r = rng();
      let level: number;
      if (r < 0.5) {
        level = 0;
      } else if (r < 0.7) {
        level = 1;
      } else if (r < 0.85) {
        level = 2;
      } else if (r < 0.95) {
        level = 3;
      } else {
        level = 4;
      }
      column.push({ col, row, level });
    }
    grid.push(column);
  }
  return grid;
}
