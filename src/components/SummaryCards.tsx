import { formatDuration } from "../shared/format";

interface SummaryCardsProps {
  productiveSeconds: number;
  unproductiveSeconds: number;
  neutralSeconds: number;
}

const items = [
  { key: "productiveSeconds", label: "Productive", tone: "productive" },
  { key: "unproductiveSeconds", label: "Unproductive", tone: "unproductive" },
  { key: "neutralSeconds", label: "Neutral", tone: "neutral" },
] as const;

export function SummaryCards(props: SummaryCardsProps) {
  return (
    <section className="summary-grid" aria-label="Today summary">
      {items.map((item) => (
        <article className="summary-card" data-tone={item.tone} key={item.key}>
          <span>{item.label}</span>
          <strong>{formatDuration(props[item.key])}</strong>
        </article>
      ))}
    </section>
  );
}
