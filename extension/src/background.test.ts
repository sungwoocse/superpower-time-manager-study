import { describe, expect, it, vi } from "vitest";
import { buildUsageEvent, isActiveUsageEligible, sendActiveTab } from "./background";

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

  it("returns null for non-http urls", () => {
    expect(buildUsageEvent({ url: "chrome://extensions", title: "Extensions" })).toBeNull();
    expect(buildUsageEvent({ url: "ftp://www.instagram.com/reels/", title: "FTP" })).toBeNull();
  });

  it("uses the normalized domain when the tab title is blank", () => {
    const event = buildUsageEvent({
      url: "https://WWW.Instagram.com/reels/",
      title: "  ",
    });

    expect(event?.domain).toBe("instagram.com");
    expect(event?.title).toBe("instagram.com");
  });
});

describe("sendActiveTab", () => {
  it("does not fetch when a usage event cannot be built", async () => {
    const fetchUsageEvent = vi.fn();

    await sendActiveTab(
      { url: "chrome://extensions", title: "Extensions", windowId: 1 },
      {
        fetchUsageEvent,
        getWindow: async () => ({ focused: true }),
        queryIdleState: async () => "active",
      },
    );

    expect(fetchUsageEvent).not.toHaveBeenCalled();
  });

  it("does not fetch when the tab window is not focused", async () => {
    const fetchUsageEvent = vi.fn();

    await sendActiveTab(
      { url: "https://www.instagram.com/reels/", title: "Instagram", windowId: 1 },
      {
        fetchUsageEvent,
        getWindow: async () => ({ focused: false }),
        queryIdleState: async () => "active",
      },
    );

    expect(fetchUsageEvent).not.toHaveBeenCalled();
  });

  it("does not fetch when the browser is idle", async () => {
    const fetchUsageEvent = vi.fn();

    await sendActiveTab(
      { url: "https://www.instagram.com/reels/", title: "Instagram", windowId: 1 },
      {
        fetchUsageEvent,
        getWindow: async () => ({ focused: true }),
        queryIdleState: async () => "idle",
      },
    );

    expect(fetchUsageEvent).not.toHaveBeenCalled();
  });
});

describe("isActiveUsageEligible", () => {
  it("requires a focused window and active idle state", async () => {
    await expect(
      isActiveUsageEligible(
        { url: "https://www.instagram.com/reels/", title: "Instagram", windowId: 1 },
        {
          getWindow: async () => ({ focused: true }),
          queryIdleState: async () => "active",
        },
      ),
    ).resolves.toBe(true);
  });
});
