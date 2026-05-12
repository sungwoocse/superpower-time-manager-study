import type { DomainRule } from "../shared/types";

export function RulesPanel({ rules }: { rules: DomainRule[] }) {
  return (
    <section className="site-section" aria-labelledby="rules-panel-title">
      <div className="section-heading">
        <h2 id="rules-panel-title">Rules</h2>
        <span>{rules.length} domains</span>
      </div>
      <ul className="rules-list">
        {rules.map((rule) => (
          <li key={rule.domain}>
            <span className="rule-domain">{rule.domain}</span>
            <span className="classification-pill" data-classification={rule.classification}>
              {rule.classification}
            </span>
          </li>
        ))}
      </ul>
    </section>
  );
}
