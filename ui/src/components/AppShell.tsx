import { useState } from "react";
import MainPanel from "./MainPanel";
import CodeInputPanel from "./CodeInputPanel";
import LandingPage from "./LandingPage";
import ChatBox from "./ChatBox";

export default function AppShell() {
  const [started, setStarted] = useState(false);
  const [analyzed, setAnalyzed] = useState(false);
  if (!started) return <LandingPage onEnter={() => setStarted(true)} />;
  return (
    <div className="app-shell">
      <aside className="sidebar">
        <div className="sidebar-top">
          <CodeInputPanel onAnalyze={() => setAnalyzed(true)} />
        </div>
        <div className="sidebar-bottom">
          <ChatBox />
        </div>
      </aside>
      <MainPanel analyzed={analyzed} />
    </div>
  );
}