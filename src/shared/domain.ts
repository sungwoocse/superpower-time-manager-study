import type { Classification, DomainRule } from "./types";

export function normalizeDomain(url: string): string {
  try {
    const hostname = new URL(url).hostname.toLowerCase();
    return hostname.startsWith("www.") ? hostname.slice(4) : hostname;
  } catch {
    return "";
  }
}

export function classifyDomain(domain: string, rules: DomainRule[]): Classification {
  const normalized = domain.toLowerCase();
  const matched = rules.find((rule) => {
    const ruleDomain = rule.domain.toLowerCase();
    return normalized === ruleDomain || normalized.endsWith(`.${ruleDomain}`);
  });

  return matched?.classification ?? "neutral";
}
