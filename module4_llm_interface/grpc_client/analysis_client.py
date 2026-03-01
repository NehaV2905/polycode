"""
Thin wrapper around the generated Module 3 gRPC stubs.

All Module 4 code talks to Module 3 exclusively through this class —
never directly touching the proto stubs.
"""

import grpc
from proto import analysis_pb2, analysis_pb2_grpc


class AnalysisClient:
    """
    gRPC client for Module 3's AnalysisService.

    Usage:
        client = AnalysisClient("http://127.0.0.1:50052")
        result = client.get_full_analysis()
        result = client.get_dead_code("path/to/file.py")
    """

    def __init__(self, addr: str):
        # Strip http:// prefix — grpc.insecure_channel expects "host:port"
        self._addr = addr.replace("http://", "").replace("https://", "")
        self._channel = grpc.insecure_channel(self._addr)
        self._stub = analysis_pb2_grpc.AnalysisServiceStub(self._channel)

    # ── Health ─────────────────────────────────────────────────────────────

    def health_check(self) -> dict:
        """Check Module 3 is reachable and return graph stats."""
        try:
            resp = self._stub.HealthCheck(analysis_pb2.EmptyRequest())
            return {
                "ok": resp.ok,
                "node_count": resp.node_count,
                "edge_count": resp.edge_count,
                "file_count": resp.file_count,
            }
        except grpc.RpcError:
            return {"ok": False, "node_count": 0, "edge_count": 0, "file_count": 0}

    # ── Codebase-wide ──────────────────────────────────────────────────────

    def get_full_analysis(self) -> dict:
        """
        Fetch all analyses across all tracked files.
        Called once on conversation start when file_path is None.
        Returns a plain dict ready to be JSON-serialised into the system prompt.
        """
        resp = self._stub.GetFullAnalysis(analysis_pb2.EmptyRequest())
        return _full_analysis_to_dict(resp)

    def get_tracked_files(self) -> list[str]:
        """Return list of all file paths tracked in the IR graph."""
        resp = self._stub.GetTrackedFiles(analysis_pb2.EmptyRequest())
        return list(resp.file_paths)

    # ── File-scoped ────────────────────────────────────────────────────────

    def get_dead_code(self, file_path: str) -> dict:
        resp = self._stub.GetDeadCode(
            analysis_pb2.FileRequest(file_path=file_path)
        )
        return {
            "file_path": resp.file_path,
            "unused_functions": list(resp.unused_functions),
        }

    def get_call_graph(self, file_path: str) -> dict:
        resp = self._stub.GetCallGraph(
            analysis_pb2.FileRequest(file_path=file_path)
        )
        return {
            "file_path": resp.file_path,
            "nodes": list(resp.nodes),
            "edges": [{"caller": e.caller, "callee": e.callee} for e in resp.edges],
        }

    def get_dependencies(self, file_path: str) -> dict:
        resp = self._stub.GetDependencies(
            analysis_pb2.FileRequest(file_path=file_path)
        )
        return {
            "file_path": resp.file_path,
            "imports": {
                k: {
                    "module_name": v.module_name,
                    "imported_names": list(v.imported_names),
                    "is_wildcard": v.is_wildcard,
                }
                for k, v in resp.imports.items()
            },
        }

    def get_impact(self, file_path: str, target_symbol: str) -> dict:
        resp = self._stub.GetImpact(
            analysis_pb2.ImpactRequest(
                file_path=file_path,
                target_symbol=target_symbol,
            )
        )
        return {
            "target_symbol": resp.target_symbol,
            "target_file": resp.target_file,
            "direct_impacts": list(resp.direct_impacts),
            "transitive_impacts": list(resp.transitive_impacts),
            "impact_depth_levels": dict(resp.impact_depth_levels),
        }

    def get_file_analysis(self, file_path: str) -> dict:
        """
        Fetch all four file-scoped analyses and combine into one dict.
        Called once on conversation start when file_path is provided.
        """
        return {
            "file_path": file_path,
            "dead_code": self.get_dead_code(file_path),
            "call_graph": self.get_call_graph(file_path),
            "dependencies": self.get_dependencies(file_path),
        }

    def close(self):
        self._channel.close()


# ── Conversion helpers ─────────────────────────────────────────────────────

def _full_analysis_to_dict(resp) -> dict:
    """Convert FullAnalysisResponse proto to a plain Python dict."""
    result = {
        "tracked_files": list(resp.tracked_files),
        "global_call_graph": None,
        "global_dependencies": None,
        "global_dead_code": None,
    }

    if resp.HasField("global_call_graph"):
        cg = resp.global_call_graph
        result["global_call_graph"] = {
            "nodes": list(cg.nodes),
            "edges": [{"caller": e.caller, "callee": e.callee} for e in cg.edges],
        }

    if resp.HasField("global_dependencies"):
        deps = resp.global_dependencies
        result["global_dependencies"] = {
            "imports": {
                k: {
                    "module_name": v.module_name,
                    "imported_names": list(v.imported_names),
                    "is_wildcard": v.is_wildcard,
                }
                for k, v in deps.imports.items()
            }
        }

    if resp.HasField("global_dead_code"):
        dc = resp.global_dead_code
        result["global_dead_code"] = {
            "unused_functions": list(dc.unused_functions),
        }

    return result