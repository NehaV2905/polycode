import { useState } from "react";
import type { Suggestion, AnalysisStats } from "../types";

interface Props {
  suggestions: Suggestion[];
  stats: AnalysisStats;
}

export default function FixSuggester({ suggestions, stats }: Props) {
  const [expanded, setExpanded] = useState<number | null>(null);

  return (
    <section className="fix-suggester">
      <h2>AI Fix Suggester</h2>

      <div className="fix-meta">
        <span>Source: <strong>{stats.source.replace(/https?:\/\/(github|gitlab)\.com\//, "")}</strong></span>
        <span>Files: <strong>{stats.files_parsed}</strong></span>
        <span>Findings: <strong>{stats.total_findings}</strong></span>
        <span>Showing: <strong>{suggestions.length}</strong></span>
      </div>

      {suggestions.length === 0 ? (
        <p style={{ opacity: 0.6, padding: "1rem 0" }}>No dead code found.</p>
      ) : (
        <div className="fix-list">
          {suggestions.map((s) => (
            <div
              key={s.id}
              className={`fix-card ${expanded === s.id ? "expanded" : ""}`}
              onClick={() => setExpanded(expanded === s.id ? null : s.id)}
            >
              <div className="fix-card-header">
                <div className="fix-card-left">
                  <span className="fix-badge">DEAD CODE</span>
                  <span className="fix-function">{s.function}</span>
                  <span className="fix-line">line {s.line}</span>
                </div>
                <div className="fix-card-right">
                  <span className="fix-file">{s.file.split("/").pop()}</span>
                  <span className="fix-chevron">{expanded === s.id ? "▲" : "▼"}</span>
                </div>
              </div>

              {expanded === s.id && (
                <div className="fix-card-body">
                  <p className="fix-filepath">{s.file}</p>
                  <p className="fix-suggestion">{s.suggestion}</p>
                </div>
              )}
            </div>
          ))}
        </div>
      )}
    </section>
  );
}