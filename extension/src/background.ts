export const INGEST_URL = "http://127.0.0.1:51247/usage-events";
// MVP-only shared secret. Replace this with a per-install generated token.
export const INGEST_TOKEN = "superpower-time-manager-dev-token";

export interface MinimalTab {
  url?: string;
  title?: string;
  windowId?: number;
}

export interface UsageEvent {
  url: string;
  domain: string;
  title: string;
  browser: "unknown";
  eventType: "active";
  timestamp: string;
}

export interface MinimalWindow {
  focused?: boolean;
}

export type IdleState = "active" | "idle" | "locked";

export interface ActiveUsageDependencies {
  getWindow?: (windowId: number) => Promise<MinimalWindow>;
  queryIdleState?: () => Promise<IdleState>;
}

export interface SendActiveTabDependencies extends ActiveUsageDependencies {
  fetchUsageEvent?: typeof fetch;
}

export function normalizeDomain(url: string): string {
  try {
    const parsedUrl = new URL(url);
    if (parsedUrl.protocol !== "http:" && parsedUrl.protocol !== "https:") return "";

    const hostname = parsedUrl.hostname.toLowerCase();
    return hostname.startsWith("www.") ? hostname.slice(4) : hostname;
  } catch {
    return "";
  }
}

export function buildUsageEvent(tab: MinimalTab): UsageEvent | null {
  const url = tab.url ?? "";
  const domain = normalizeDomain(url);
  if (!domain) return null;

  return {
    url,
    domain,
    title: tab.title?.trim() || domain,
    browser: "unknown",
    eventType: "active",
    timestamp: new Date().toISOString(),
  };
}

export async function isActiveUsageEligible(
  tab: MinimalTab,
  dependencies: ActiveUsageDependencies = getDefaultDependencies(),
): Promise<boolean> {
  if (typeof tab.windowId !== "number") return false;
  if (!dependencies.getWindow || !dependencies.queryIdleState) return false;

  try {
    const [activeWindow, idleState] = await Promise.all([
      dependencies.getWindow(tab.windowId),
      dependencies.queryIdleState(),
    ]);

    return activeWindow.focused === true && idleState === "active";
  } catch {
    return false;
  }
}

export async function sendActiveTab(
  tab: MinimalTab,
  dependencies: SendActiveTabDependencies = getDefaultDependencies(),
): Promise<void> {
  const event = buildUsageEvent(tab);
  if (!event) return;

  const eligible = await isActiveUsageEligible(tab, dependencies);
  if (!eligible) return;

  try {
    const fetchUsageEvent = dependencies.fetchUsageEvent ?? fetch;
    const response = await fetchUsageEvent(INGEST_URL, {
      method: "POST",
      headers: {
        "content-type": "application/json",
        "x-time-manager-token": INGEST_TOKEN,
      },
      body: JSON.stringify(event),
    });
    if (!response.ok) return;
  } catch {
    // The desktop app may be closed. MVP retries on the next browser event.
  }
}

function getDefaultDependencies(): SendActiveTabDependencies {
  if (typeof chrome === "undefined") return {};

  return {
    fetchUsageEvent: fetch,
    getWindow:
      chrome.windows?.get ?
        (windowId) =>
          new Promise((resolve, reject) => {
            chrome.windows.get(windowId, (activeWindow) => {
              const error = chrome.runtime?.lastError;
              if (error) {
                reject(new Error(error.message));
                return;
              }

              resolve(activeWindow);
            });
          })
      : undefined,
    queryIdleState:
      chrome.idle?.queryState ?
        () =>
          new Promise((resolve) => {
            chrome.idle.queryState(60, resolve);
          })
      : undefined,
  };
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
