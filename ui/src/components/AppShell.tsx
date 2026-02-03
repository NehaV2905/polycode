import Sidebar from "./Sidebar";
import MainPanel from "./MainPanel";
import StatusBar from "./StatusBar";

export default function AppShell() {
  return (
    <div className="app-shell">
      <Sidebar />
      <MainPanel />
      <StatusBar />
    </div>
  );
}
