import { useState } from "react";
import type { ChangeEvent } from "react";
import type { AnalysisResult } from "../types";

type Mode = "file" | "repo";

interface Props {
  onResult: (result: AnalysisResult) => void;
  onLoading: (loading: boolean) => void;
  onError: (error: string | null) => void;
}

export default function CodeInputPanel({ onResult, onLoading, onError }: Props) {
  const [mode, setMode] = useState<Mode>("file");
  const [files, setFiles] = useState<File[]>([]);
  const [repoUrl, setRepoUrl] = useState("");

  const handleFileChange = (e: ChangeEvent<HTMLInputElement>) => {
    if (!e.target.files) return;
    setFiles((prev) => [...prev, ...Array.from(e.target.files!)]);
    e.target.value = "";
  };

  const removeFile = (i: number) =>
    setFiles((prev) => prev.filter((_, idx) => idx !== i));

  const handleAnalyze = async () => {
    onError(null);
    onLoading(true);
    try {
      let response: Response;

      if (mode === "file") {
        if (files.length === 0) {
          onError("Please upload at least one file.");
          return;
        }
        const form = new FormData();
        files.forEach((f) => form.append("file", f, f.name));
        response = await fetch("/api/analyze/files", { method: "POST", body: form });
      } else {
        if (!repoUrl.trim()) {
          onError("Please enter a repository URL.");
          return;
        }
        response = await fetch("/api/analyze/repo", {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ url: repoUrl.trim(), max_fixes: 10 }),
        });
      }

      if (!response.ok) {
        const body = await response.json().catch(() => ({ error: response.statusText }));
        throw new Error(body.error ?? `Server error ${response.status}`);
      }

      onResult(await response.json());
    } catch (err) {
      onError(err instanceof Error ? err.message : String(err));
    } finally {
      onLoading(false);
    }
  };

  return (
    <section>
      <h3>Code Input</h3>

      <div className="input-mode-toggle">
        <label className={`mode-option ${mode === "file" ? "active" : ""}`}>
          <input type="radio" name="inputMode" value="file" checked={mode === "file"} onChange={() => setMode("file")} />
          Upload Files
        </label>
        <label className={`mode-option ${mode === "repo" ? "active" : ""}`}>
          <input type="radio" name="inputMode" value="repo" checked={mode === "repo"} onChange={() => setMode("repo")} />
          GitHub Repo
        </label>
      </div>

      {mode === "file" && (
        <>
          <input type="file" multiple onChange={handleFileChange} />
          {files.length > 0 && (
            <div>
              <strong>Uploaded Files:</strong>
              <ul>
                {files.map((file, i) => (
                  <li key={i}>
                    <span>{file.name}</span>
                    <button onClick={() => removeFile(i)}>Remove</button>
                  </li>
                ))}
              </ul>
            </div>
          )}
        </>
      )}

      {mode === "repo" && (
        <div className="repo-input-wrapper">
          <input
            className="repo-input"
            type="text"
            placeholder="https://github.com/user/repo"
            value={repoUrl}
            onChange={(e) => setRepoUrl(e.target.value)}
          />
        </div>
      )}

      <button onClick={handleAnalyze}>Analyze</button>
    </section>
  );
}