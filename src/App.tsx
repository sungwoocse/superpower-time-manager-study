import { RulesPanel } from "./components/RulesPanel";
import { SiteTable, type SiteUsageRow } from "./components/SiteTable";
import { SummaryCards } from "./components/SummaryCards";
import type { DomainRule } from "./shared/types";
import "./styles.css";

const rows: SiteUsageRow[] = [
  { domain: "chatgpt.com", classification: "productive", seconds: 25 * 60 },
  { domain: "youtube.com", classification: "unproductive", seconds: 90 * 60 },
  { domain: "example.com", classification: "neutral", seconds: 12 * 60 },
];

const rules: DomainRule[] = [
  { domain: "chatgpt.com", classification: "productive" },
  { domain: "youtube.com", classification: "unproductive" },
];

export function App() {
  const productiveSeconds = rows
    .filter((row) => row.classification === "productive")
    .reduce((total, row) => total + row.seconds, 0);
  const unproductiveSeconds = rows
    .filter((row) => row.classification === "unproductive")
    .reduce((total, row) => total + row.seconds, 0);
  const neutralSeconds = rows
    .filter((row) => row.classification === "neutral")
    .reduce((total, row) => total + row.seconds, 0);

  return (
    <main className="app-shell">
      <header className="app-header">
        <div>
          <p className="eyebrow">Windows MVP</p>
          <h1>Superpower Time Manager</h1>
        </div>
      </header>
      <SummaryCards
        productiveSeconds={productiveSeconds}
        unproductiveSeconds={unproductiveSeconds}
        neutralSeconds={neutralSeconds}
      />
      <div className="dashboard-grid">
        <SiteTable rows={rows} />
        <RulesPanel rules={rules} />
      </div>
    </main>
  );
}

export default App;
