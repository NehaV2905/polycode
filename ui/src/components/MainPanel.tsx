import "../App.css";
import AnalysisOutput from "./AnalysisOutput";
import DependencyGraph from "./DependencyGraph";
import FixSuggester from "./FixSuggester";
import type { AnalysisResult } from "../types";

interface Props {
  result: AnalysisResult | null;
  loading: boolean;
  error: string | null;
}

export default function MainPanel({ result, loading, error }: Props) {
  if (loading) {
    return (
      <main className="main-content">
        <p style={{ padding: "2rem", opacity: 0.7 }}>Analyzing… this may take a moment.</p>
      </main>
    );
  }

  if (error) {
    return (
      <main className="main-content">
        <p style={{ padding: "2rem", color: "#e07070" }}>Error: {error}</p>
      </main>
    );
  }

  if (!result) {
    return (
      <main className="main-content">
        <p style={{ padding: "2rem", opacity: 0.4 }}>Upload a file or enter a repo URL to start.</p>
      </main>
    );
  }

  return (
    <main className="main-content">
      <AnalysisOutput ir={result.ir} />
      <DependencyGraph ir={result.ir} />
      <FixSuggester suggestions={result.suggestions} stats={result.stats} />
    </main>
  );
}