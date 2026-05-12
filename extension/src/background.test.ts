import { beforeEach, describe, expect, it, vi } from "vitest";
import {
  buildUsageEvent,
  CONFIG_URL,
  isActiveUsageEligible,
  resetIngestConfigCacheForTests,
  sendActiveTab,
} from "./background";

describe("buildUsageEvent", () => {
  it("normalizes Instagram tab data into an active usage event", () => {
    const event = buildUsageEvent({
      url: "https://www.instagram.com/reels/",
      title: "Instagram",
    });

    expect(event).not.toBeNull();
    if (!event) throw new Error("expected usage event");

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
  beforeEach(() => {
    resetIngestConfigCacheForTests();
  });

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

  it("fetches the per-install token before sending a usage event", async () => {
    const fetchUsageEvent = vi
      .fn()
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({ ingestToken: "install-token" }),
      })
      .mockResolvedValueOnce({ ok: true });

    await sendActiveTab(
      { url: "https://www.instagram.com/reels/", title: "Instagram", windowId: 1 },
      {
        fetchUsageEvent,
        getWindow: async () => ({ focused: true }),
        queryIdleState: async () => "active",
      },
    );

    expect(fetchUsageEvent).toHaveBeenNthCalledWith(1, CONFIG_URL, {
      method: "GET",
      headers: { accept: "application/json" },
    });
    expect(fetchUsageEvent).toHaveBeenCalledWith(
      expect.any(String),
      expect.objectContaining({
        headers: {
          "content-type": "application/json",
          "x-time-manager-token": "install-token",
        },
      }),
    );
  });

  it("skips sending and retries config later when token fetch fails", async () => {
    const fetchUsageEvent = vi
      .fn()
      .mockResolvedValueOnce({ ok: false })
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({ ingestToken: "install-token" }),
      })
      .mockResolvedValueOnce({ ok: true });
    const dependencies = {
      fetchUsageEvent,
      getWindow: async () => ({ focused: true }),
      queryIdleState: async () => "active" as const,
    };

    await sendActiveTab(
      { url: "https://www.instagram.com/reels/", title: "Instagram", windowId: 1 },
      dependencies,
    );
    await sendActiveTab(
      { url: "https://www.instagram.com/reels/", title: "Instagram", windowId: 1 },
      dependencies,
    );

    expect(fetchUsageEvent).toHaveBeenNthCalledWith(1, CONFIG_URL, {
      method: "GET",
      headers: { accept: "application/json" },
    });
    expect(fetchUsageEvent).toHaveBeenNthCalledWith(2, CONFIG_URL, {
      method: "GET",
      headers: { accept: "application/json" },
    });
    expect(fetchUsageEvent).toHaveBeenCalledTimes(3);
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
