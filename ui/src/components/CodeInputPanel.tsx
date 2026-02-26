import { useState } from "react";
import type { ChangeEvent } from "react";

export default function CodeInputPanel({ onAnalyze }: { onAnalyze: () => void }) {
  const [files, setFiles] = useState<File[]>([]);

  const handleFileChange = (e: ChangeEvent<HTMLInputElement>) => {
    if (!e.target.files) return;
    const selectedFiles = Array.from(e.target.files);
    setFiles((prev) => [...prev, ...selectedFiles]);
    e.target.value = "";
  };

  const downloadFile = (file: File) => {
    const url = URL.createObjectURL(file);
    const a = document.createElement("a");
    a.href = url;
    a.download = file.name;
    a.click();
    URL.revokeObjectURL(url);
  };

  const removeFile = (indexToRemove: number) => {
    setFiles((prev) => prev.filter((_, i) => i !== indexToRemove));
  };

  return (
    <section style={{ marginBottom: "1rem" }}>
      <h3>Code Input</h3>
      <input type="file" multiple onChange={handleFileChange} />
      <br /><br />
      {files.length > 0 && (
        <div>
          <strong>Uploaded Files:</strong>
          <ul>
            {files.map((file, index) => (
              <li key={index} style={{ marginBottom: "6px" }}>
                <button
                  onClick={() => downloadFile(file)}
                  style={{
                    background: "none",
                    border: "none",
                    color: "blue",
                    cursor: "pointer",
                    textDecoration: "underline",
                    marginRight: "10px"
                  }}
                >
                  {file.name}
                </button>
                <button
                  onClick={() => removeFile(index)}
                  style={{
                    color: "white",
                    background: "crimson",
                    border: "none",
                    borderRadius: "4px",
                    padding: "2px 8px",
                    cursor: "pointer"
                  }}
                >
                  Remove
                </button>
              </li>
            ))}
          </ul>
        </div>
      )}
      <button onClick={onAnalyze}>Analyze</button>
    </section>
  );
}