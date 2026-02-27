import { useState } from "react";
import MainPanel from "./MainPanel";
import CodeInputPanel from "./CodeInputPanel";
import LandingPage from "./LandingPage";

export default function AppShell() {
  const [started, setStarted] = useState(false);
  const [analyzed, setAnalyzed] = useState(false);
  if (!started) return <LandingPage onEnter={() => setStarted(true)} />;
  return (
    <div className="app-shell">
      <aside className="sidebar">
        <CodeInputPanel onAnalyze={() => setAnalyzed(true)} />
      </aside>
      <MainPanel analyzed={analyzed} />
    </div>
  );
}