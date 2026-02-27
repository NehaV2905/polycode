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
  <section>
    <h3>Code Input</h3>
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
    <button onClick={onAnalyze}>Analyze</button>
  </section>
);
}