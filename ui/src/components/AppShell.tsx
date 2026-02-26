import { useState } from "react";
import MainPanel from "./MainPanel";
import CodeInputPanel from "./CodeInputPanel";

export default function AppShell() {
  const [analyzed, setAnalyzed] = useState(false);
  return (
    <div className="app-shell">
      <CodeInputPanel onAnalyze={() => setAnalyzed(true)} />
      <MainPanel analyzed={analyzed} />
    </div>
  );
}
