"""
HTTP client for api_server (port 3000).

Replaces the gRPC client — api_server is the real analysis backend.
Module 3 is a library embedded inside api_server, not a standalone gRPC server.

api_server endpoints used:
  POST /api/analyze/files   multipart(files) → AnalysisResponse
"""

import httpx
from pathlib import Path


class AnalysisClient:
    """
    HTTP client wrapping api_server's analysis endpoints.

    Usage:
        client = AnalysisClient("http://127.0.0.1:3000")
        result = client.get_file_analysis("path/to/file.py")
    """

    def __init__(self, addr: str):
        self._base = addr.rstrip("/")
        self._http = httpx.Client(timeout=60.0)

    # ── Health ─────────────────────────────────────────────────────────────

    def health_check(self) -> dict:
        """Check if api_server is reachable."""
        try:
            resp = self._http.post(
                f"{self._base}/api/analyze/files",
                files={},
                timeout=5.0,
            )
            return {
                "ok": resp.status_code < 500,
                "node_count": 0,
                "edge_count": 0,
                "file_count": 0,
            }
        except Exception:
            return {"ok": False, "node_count": 0, "edge_count": 0, "file_count": 0}

    # ── File analysis ──────────────────────────────────────────────────────

    def get_file_analysis(self, file_path: str) -> dict:
        """
        Upload a single file to api_server and return its full analysis.
        Called once on conversation start when file_path is provided.
        """
        path = Path(file_path)
        if not path.exists():
            raise FileNotFoundError(f"File not found: {file_path}")

        with open(path, "rb") as f:
            content = f.read()

        resp = self._http.post(
            f"{self._base}/api/analyze/files",
            files={"files": (path.name, content, "text/plain")},
        )
        resp.raise_for_status()
        data = resp.json()

        return {
            "file_path": file_path,
            "ir": data.get("ir", {}),
            "dead_code": {
                "unused_functions": [
                    s["function"] for s in data.get("suggestions", [])
                ]
            },
            "stats": data.get("stats", {}),
        }

    def get_full_analysis(self) -> dict:
        """
        Codebase-wide — api_server requires a file or repo URL.
        Returns a prompt to ask the user to specify a file.
        """
        return {
            "note": (
                "No file specified. Please ask the user which file "
                "they want to analyse, or provide a file path."
            )
        }

    def close(self):
        self._http.close()