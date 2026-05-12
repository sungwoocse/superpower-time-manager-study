import { describe, expect, it } from "vitest";
import { buildUsageEvent } from "./background";

describe("buildUsageEvent", () => {
  it("normalizes Instagram tab data into an active usage event", () => {
    const event = buildUsageEvent({
      url: "https://www.instagram.com/reels/",
      title: "Instagram",
    });

    expect(event.domain).toBe("instagram.com");
    expect(event.browser).toBe("unknown");
    expect(event.eventType).toBe("active");
    expect(event.title).toBe("Instagram");
  });
});
