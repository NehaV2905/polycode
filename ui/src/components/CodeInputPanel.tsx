import { useState } from "react";

export default function CodeInputPanel() {
  const [language, setLanguage] = useState("Python");
  const [code, setCode] = useState("");

  return (
    <section style={{ marginBottom: "1rem" }}>
      <h3>Code Input</h3>

      <label>
        Language:&nbsp;
        <select
          value={language}
          onChange={(e) => setLanguage(e.target.value)}
        >
          <option>Python</option>
          <option>Java</option>
          <option>Go</option>
        </select>
      </label>

      <br /><br />

      <textarea
        placeholder={`Paste ${language} code here...`}
        value={code}
        onChange={(e) => setCode(e.target.value)}
        rows={8}
        style={{ width: "100%", fontFamily: "monospace" }}
      />

      <br /><br />

      <button disabled>
        Analyze (Phase 2)
      </button>
    </section>
  );
}
