import "../App.css";
import DependencyGraph from "./DependencyGraph";
import AnalysisOutput from "./AnalysisOutput";


export default function MainPanel({ analyzed }: { analyzed: boolean }) {
  return (
    <main className="main-content">
      {analyzed && <AnalysisOutput />}
      {analyzed && <DependencyGraph />}
    </main>
  );
}
