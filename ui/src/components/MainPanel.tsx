import CodeInputPanel from "./CodeInputPanel";
import ProjectSummary from "./ProjectSummary";
import FunctionTable from "./FunctionTable";
import DependencyGraph from "./DependencyGraph";

export default function MainPanel() {
  return (
    <main>
      <h2>POLYCODE</h2>
      <CodeInputPanel />
      <ProjectSummary />
      <FunctionTable />
      <DependencyGraph />
    </main>
  );
}
