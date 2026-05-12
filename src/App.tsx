import "./styles.css";

export function App() {
  return (
    <main className="app-shell">
      <header className="app-header">
        <div>
          <p className="eyebrow">Windows MVP</p>
          <h1>Superpower Time Manager</h1>
        </div>
      </header>
      <section className="summary-grid" aria-label="Today summary">
        <article>
          <span>Productive</span>
          <strong>0m</strong>
        </article>
        <article>
          <span>Unproductive</span>
          <strong>0m</strong>
        </article>
        <article>
          <span>Neutral</span>
          <strong>0m</strong>
        </article>
      </section>
    </main>
  );
}

export default App;
