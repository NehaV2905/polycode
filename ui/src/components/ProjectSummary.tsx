import { dummyIR } from "../data/dummyIR";

export default function ProjectSummary() {
  return (
    <>
      <div>
        <h3>Modules</h3>
        <ul>
          {dummyIR.modules.map(m => (
            <li key={m.name}>
              {m.name} ({m.language})
            </li>
          ))}
        </ul>
      </div>
      <section>
        <h2>{dummyIR.projectName}</h2>
        <p>Languages: {dummyIR.languages.join(", ")}</p>
        <p>Modules: {dummyIR.modules.length}</p>
        <p>Functions: {dummyIR.functions.length}</p>
      </section>
    </>
  );
}