import "../App.css";
import ProjectSummary from "./ProjectSummary";
import FunctionTable from "./FunctionTable";
import DependencyGraph from "./DependencyGraph";

export default function MainPanel() {
  return (
    <main className="main-content">
      <ProjectSummary />
      <FunctionTable />
      <DependencyGraph />
    </main>
  );
}
