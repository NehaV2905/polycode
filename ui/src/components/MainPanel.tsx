import "../App.css";
import AnalysisOutput from "./AnalysisOutput";
import DependencyGraph from "./DependencyGraph";
import FixSuggester from "./FixSuggester";

export default function MainPanel({ analyzed }: { analyzed: boolean }) {
  return (
    <main className="main-content">
      {analyzed && <AnalysisOutput />}
      {analyzed && <DependencyGraph />}
      {analyzed && <FixSuggester />}
    </main>
  );
}