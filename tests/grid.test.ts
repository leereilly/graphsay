import { describe, it, expect } from "vitest";
import { generateGrid, NUM_COLS, NUM_ROWS } from "../src/grid.js";

describe("grid", () => {
  it("has correct dimensions", () => {
    const grid = generateGrid();
    expect(grid).toHaveLength(NUM_COLS);
    for (const col of grid) {
      expect(col).toHaveLength(NUM_ROWS);
    }
  });

  it("all cells are non-null with valid levels", () => {
    const grid = generateGrid();
    for (let col = 0; col < NUM_COLS; col++) {
      for (let row = 0; row < NUM_ROWS; row++) {
        const cell = grid[col][row];
        expect(cell).not.toBeNull();
        expect(cell!.col).toBe(col);
        expect(cell!.row).toBe(row);
        expect(cell!.level).toBeGreaterThanOrEqual(0);
        expect(cell!.level).toBeLessThanOrEqual(4);
      }
    }
  });

  it("respects level distribution with deterministic rng", () => {
    let callCount = 0;
    const rng = () => {
      // Cycle through values that hit each level bucket
      const vals = [0.0, 0.55, 0.75, 0.90, 0.99];
      return vals[callCount++ % vals.length];
    };
    const grid = generateGrid(rng);
    // First col: 0, 1, 2, 3, 4, 0, 1
    expect(grid[0][0]!.level).toBe(0);
    expect(grid[0][1]!.level).toBe(1);
    expect(grid[0][2]!.level).toBe(2);
    expect(grid[0][3]!.level).toBe(3);
    expect(grid[0][4]!.level).toBe(4);
    expect(grid[0][5]!.level).toBe(0);
    expect(grid[0][6]!.level).toBe(1);
  });

  it("statistical distribution is reasonable", () => {
    const counts = [0, 0, 0, 0, 0];
    for (let i = 0; i < 100; i++) {
      const grid = generateGrid();
      for (const col of grid) {
        for (const cell of col) {
          if (cell) counts[cell.level]++;
        }
      }
    }
    const total = counts.reduce((a, b) => a + b, 0);
    const pcts = counts.map((c) => c / total);
    expect(pcts[0]).toBeGreaterThan(0.35);
    expect(pcts[4]).toBeLessThan(0.15);
    expect(pcts[0]).toBeGreaterThan(pcts[4]);
  });
});
