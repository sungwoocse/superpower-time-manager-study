export type Classification = "productive" | "unproductive" | "neutral";

export interface UsageEvent {
  url: string;
  domain: string;
  title: string;
  browser: "chrome" | "edge" | "unknown";
  eventType: "focus" | "blur" | "idle" | "active";
  timestamp: string;
}

export interface DomainRule {
  domain: string;
  classification: Classification;
}
