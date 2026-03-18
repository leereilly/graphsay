import { describe, it, expect } from "vitest";
import { buildFfmpegArgs } from "../src/ffmpeg.js";

describe("ffmpeg", () => {
  it("builds gif args correctly", () => {
    const args = buildFfmpegArgs("/tmp/frames", "/tmp/out.gif", "gif", 20);
    expect(args).toContain("-y");
    expect(args).toContain("20");
    expect(args).toContain("/tmp/out.gif");
    expect(args.join(" ")).toContain("palettegen");
    expect(args.join(" ")).toContain("paletteuse");
  });

  it("builds webp args correctly", () => {
    const args = buildFfmpegArgs("/tmp/frames", "/tmp/out.webp", "webp", 20);
    expect(args).toContain("libwebp");
    expect(args).toContain("-lossless");
    expect(args).toContain("1");
  });

  it("builds mp4 args correctly", () => {
    const args = buildFfmpegArgs("/tmp/frames", "/tmp/out.mp4", "mp4", 20);
    expect(args).toContain("libx264");
    expect(args).toContain("yuv420p");
    expect(args).toContain("+faststart");
  });

  it("gif args include -loop 0 when loop is true", () => {
    const args = buildFfmpegArgs("/tmp/frames", "/tmp/out.gif", "gif", 20, true);
    const loopIdx = args.indexOf("-loop");
    expect(loopIdx).toBeGreaterThan(-1);
    expect(args[loopIdx + 1]).toBe("0");
  });

  it("gif args include -loop -1 when loop is false", () => {
    const args = buildFfmpegArgs("/tmp/frames", "/tmp/out.gif", "gif", 20, false);
    const loopIdx = args.indexOf("-loop");
    expect(loopIdx).toBeGreaterThan(-1);
    expect(args[loopIdx + 1]).toBe("-1");
  });

  it("webp args always include -loop 0 regardless of loop flag", () => {
    const argsTrue = buildFfmpegArgs("/tmp/frames", "/tmp/out.webp", "webp", 20, true);
    const argsFalse = buildFfmpegArgs("/tmp/frames", "/tmp/out.webp", "webp", 20, false);
    const idxTrue = argsTrue.indexOf("-loop");
    const idxFalse = argsFalse.indexOf("-loop");
    expect(argsTrue[idxTrue + 1]).toBe("0");
    expect(argsFalse[idxFalse + 1]).toBe("0");
  });
});
