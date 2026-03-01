import { useState } from "react";
import type { ChangeEvent } from "react";

type Mode = "file" | "repo";

export default function CodeInputPanel({ onAnalyze }: { onAnalyze: () => void }) {
  const [mode, setMode] = useState<Mode>("file");
  const [files, setFiles] = useState<File[]>([]);
  const [repoUrl, setRepoUrl] = useState("");

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

  const handleAnalyze = () => {
    if (mode === "file") {
      // TODO: call file upload backend
      // e.g. POST /api/analyze/files with FormData
      console.log("Calling file analysis backend with", files);
    } else {
      // TODO: call repo backend
      // e.g. POST /api/analyze/repo with { url: repoUrl }
      console.log("Calling repo analysis backend with", repoUrl);
    }
    onAnalyze();
  };

  return (
    <section>
      <h3>Code Input</h3>

      {/* ── Radio Toggle ── */}
      <div className="input-mode-toggle">
        <label className={`mode-option ${mode === "file" ? "active" : ""}`}>
          <input
            type="radio"
            name="inputMode"
            value="file"
            checked={mode === "file"}
            onChange={() => setMode("file")}
          />
          Upload Files
        </label>
        <label className={`mode-option ${mode === "repo" ? "active" : ""}`}>
          <input
            type="radio"
            name="inputMode"
            value="repo"
            checked={mode === "repo"}
            onChange={() => setMode("repo")}
          />
          GitHub Repo
        </label>
      </div>

      {/* ── File Upload ── */}
      {mode === "file" && (
        <>
          <input type="file" multiple onChange={handleFileChange} />
          {files.length > 0 && (
            <div>
              <strong>Uploaded Files:</strong>
              <ul>
                {files.map((file, index) => (
                  <li key={index}>
                    <button onClick={() => downloadFile(file)}>{file.name}</button>
                    <button onClick={() => removeFile(index)}>Remove</button>
                  </li>
                ))}
              </ul>
            </div>
          )}
        </>
      )}

      {/* ── Repo URL ── */}
      {mode === "repo" && (
        <div className="repo-input-wrapper">
          <input
            className="repo-input"
            type="text"
            placeholder="https://github.com/user/repo"
            value={repoUrl}
            onChange={e => setRepoUrl(e.target.value)}
          />
        </div>
      )}

      <button onClick={handleAnalyze}>Analyze</button>
    </section>
  );
}