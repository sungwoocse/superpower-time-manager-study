import { formatDuration } from "../shared/format";
import type { Classification } from "../shared/types";

export interface SiteUsageRow {
  domain: string;
  classification: Classification;
  seconds: number;
}

export function SiteTable({ rows }: { rows: SiteUsageRow[] }) {
  return (
    <section className="site-section" aria-labelledby="site-table-title">
      <div className="section-heading">
        <h2 id="site-table-title">Sites</h2>
        <span>{rows.length} domains</span>
      </div>
      <div className="table-scroll">
        <table className="site-table">
          <thead>
            <tr>
              <th scope="col">Domain</th>
              <th scope="col">Class</th>
              <th scope="col">Time</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row) => (
              <tr key={row.domain}>
                <td>{row.domain}</td>
                <td>
                  <span className="classification-pill" data-classification={row.classification}>
                    {row.classification}
                  </span>
                </td>
                <td>{formatDuration(row.seconds)}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </section>
  );
}
