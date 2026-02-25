import MainPanel from "./MainPanel";
import StatusBar from "./StatusBar";
import CodeInputPanel from "./CodeInputPanel";

export default function AppShell() {
  return (
    <div className="app-shell">
      <CodeInputPanel />
      <MainPanel />
      <StatusBar />
    </div>
  );
}
