import { describe, expect, it } from "vitest";
import { classifyDomain, normalizeDomain } from "./domain";

describe("normalizeDomain", () => {
  it("extracts hostname and removes www prefix", () => {
    expect(normalizeDomain("https://www.youtube.com/watch?v=abc")).toBe("youtube.com");
  });

  it("returns empty string for invalid URLs", () => {
    expect(normalizeDomain("not a url")).toBe("");
  });
});

describe("classifyDomain", () => {
  it("matches exact domains and subdomains", () => {
    const rules = [{ domain: "youtube.com", classification: "unproductive" as const }];

    expect(classifyDomain("youtube.com", rules)).toBe("unproductive");
    expect(classifyDomain("m.youtube.com", rules)).toBe("unproductive");
  });

  it("returns neutral when no rule matches", () => {
    expect(classifyDomain("example.com", [])).toBe("neutral");
  });
});
