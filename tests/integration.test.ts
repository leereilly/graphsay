import { describe, it, expect } from "vitest";
import { execFileSync } from "node:child_process";
import * as fs from "node:fs";
import * as path from "node:path";
import * as os from "node:os";

describe("integration", () => {
  const cliPath = path.resolve("dist/cli.js");

  it("generates SVG with light theme", () => {
    const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "int-test-"));
    const outputPath = path.join(tmpDir, "test-light.svg");
    try {
      execFileSync("node", [
        cliPath,
        "--message", "TEST",
        "--format", "svg",
        "--theme", "light",
        "-o", outputPath,
      ], { stdio: "pipe" });

      const content = fs.readFileSync(outputPath, "utf-8");
      expect(content.startsWith("<svg")).toBe(true);
      expect(content.endsWith("</svg>")).toBe(true);
    } finally {
      fs.rmSync(tmpDir, { recursive: true, force: true });
    }
  });

  it("generates SVG with both theme and dark mode CSS", () => {
    const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "int-test-"));
    const outputPath = path.join(tmpDir, "test-both.svg");
    try {
      execFileSync("node", [
        cliPath,
        "--message", "HI",
        "--format", "svg",
        "--theme", "both",
        "-o", outputPath,
      ], { stdio: "pipe" });

      const content = fs.readFileSync(outputPath, "utf-8");
      expect(content).toContain("prefers-color-scheme: dark");
      expect(content).toContain("<style>");
    } finally {
      fs.rmSync(tmpDir, { recursive: true, force: true });
    }
  });

  it("generates SVG in static mode", () => {
    const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "int-test-"));
    const outputPath = path.join(tmpDir, "test-static.svg");
    try {
      execFileSync("node", [
        cliPath,
        "--message", "OK",
        "--format", "svg",
        "--mode", "static",
        "-o", outputPath,
      ], { stdio: "pipe" });

      const content = fs.readFileSync(outputPath, "utf-8");
      expect(content).toContain("<svg");
    } finally {
      fs.rmSync(tmpDir, { recursive: true, force: true });
    }
  });

  it("fails without --message", () => {
    expect(() => {
      execFileSync("node", [cliPath], { stdio: "pipe" });
    }).toThrow();
  });

  it("fails with invalid color", () => {
    expect(() => {
      execFileSync("node", [
        cliPath,
        "--message", "X",
        "--color", "notacolor",
      ], { stdio: "pipe" });
    }).toThrow();
  });

  it("SVG uses palette gray not white for empty cells", () => {
    const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "int-test-"));
    const outputPath = path.join(tmpDir, "test-gray.svg");
    try {
      execFileSync("node", [
        cliPath,
        "--message", "A",
        "--format", "svg",
        "--theme", "light",
        "-o", outputPath,
      ], { stdio: "pipe" });

      const content = fs.readFileSync(outputPath, "utf-8");
      expect(content).toContain("#ebedf0");
    } finally {
      fs.rmSync(tmpDir, { recursive: true, force: true });
    }
  });

  it("default SVG uses repeatCount indefinite (loop enabled)", () => {
    const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "int-test-"));
    const outputPath = path.join(tmpDir, "test-loop-default.svg");
    try {
      execFileSync("node", [
        cliPath,
        "--message", "HI",
        "--format", "svg",
        "--theme", "light",
        "-o", outputPath,
      ], { stdio: "pipe" });

      const content = fs.readFileSync(outputPath, "utf-8");
      expect(content).toContain('repeatCount="indefinite"');
      expect(content).not.toContain('fill="freeze"');
    } finally {
      fs.rmSync(tmpDir, { recursive: true, force: true });
    }
  });

  it("--no-loop SVG uses repeatCount 1 and fill freeze", () => {
    const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "int-test-"));
    const outputPath = path.join(tmpDir, "test-no-loop.svg");
    try {
      execFileSync("node", [
        cliPath,
        "--message", "HI",
        "--format", "svg",
        "--theme", "light",
        "--no-loop",
        "-o", outputPath,
      ], { stdio: "pipe" });

      const content = fs.readFileSync(outputPath, "utf-8");
      expect(content).toContain('repeatCount="1"');
      expect(content).toContain('fill="freeze"');
    } finally {
      fs.rmSync(tmpDir, { recursive: true, force: true });
    }
  });
});
