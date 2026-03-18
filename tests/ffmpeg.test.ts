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
});
