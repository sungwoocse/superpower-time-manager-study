import { describe, expect, it } from "vitest";
import { formatDuration } from "./format";

describe("formatDuration", () => {
  it("formats minutes under one hour", () => {
    expect(formatDuration(25 * 60)).toBe("25m");
  });

  it("formats hours and minutes", () => {
    expect(formatDuration(90 * 60)).toBe("1h 30m");
  });

  it("formats exact hours without trailing minutes", () => {
    expect(formatDuration(2 * 60 * 60)).toBe("2h");
  });

  it("rounds down to whole minutes and clamps negative values", () => {
    expect(formatDuration(59)).toBe("0m");
    expect(formatDuration(-60)).toBe("0m");
  });
});
