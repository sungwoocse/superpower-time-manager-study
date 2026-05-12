export const INGEST_URL = "http://127.0.0.1:51247/usage-events";

export interface MinimalTab {
  url?: string;
  title?: string;
}

export interface UsageEvent {
  url: string;
  domain: string;
  title: string;
  browser: "unknown";
  eventType: "active";
  timestamp: string;
}

export function normalizeDomain(url: string): string {
  try {
    const hostname = new URL(url).hostname.toLowerCase();
    return hostname.startsWith("www.") ? hostname.slice(4) : hostname;
  } catch {
    return "";
  }
}

export function buildUsageEvent(tab: MinimalTab): UsageEvent {
  const url = tab.url ?? "";

  return {
    url,
    domain: normalizeDomain(url),
    title: tab.title ?? "",
    browser: "unknown",
    eventType: "active",
    timestamp: new Date().toISOString(),
  };
}

export async function sendActiveTab(tab: MinimalTab): Promise<void> {
  const event = buildUsageEvent(tab);
  if (!event.domain) return;

  try {
    await fetch(INGEST_URL, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify(event),
    });
  } catch {
    // The desktop app may be closed. MVP retries on the next browser event.
  }
}

function registerChromeListeners(): void {
  if (typeof chrome === "undefined" || !chrome.tabs) return;

  chrome.tabs.onActivated.addListener(async ({ tabId }) => {
    const tab = await chrome.tabs.get(tabId);
    await sendActiveTab(tab);
  });

  chrome.tabs.onUpdated.addListener(async (_tabId, changeInfo, tab) => {
    if (changeInfo.status === "complete" && tab.active) {
      await sendActiveTab(tab);
    }
  });
}

registerChromeListeners();
