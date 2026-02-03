import { dummyIR } from "../data/dummyIR";

export default function ProjectSummary() {
  return (
    <section>
      <h2>{dummyIR.projectName}</h2>
      <p>Languages: {dummyIR.languages.join(", ")}</p>
      <p>Modules: {dummyIR.modules.length}</p>
      <p>Functions: {dummyIR.functions.length}</p>
    </section>
  );
}
