import json
import os
from grpc_client.analysis_client import AnalysisClient

# Path is relative to this file's directory
_DIR = os.path.dirname(os.path.abspath(__file__))

CACHED_SUMMARIES = {
    "https://github.com/NehaV2905/polycode-test-repo": os.path.join(_DIR, "test_repo.txt")
}

def fetch_context(
    client: AnalysisClient,
    file_path: str | None,
    file_content: str | None = None,
    file_name: str | None = None,
) -> dict:
    if file_content is not None and file_name is not None:
        return client.get_analysis_from_content(file_content, file_name)
    elif file_path is not None:
        return client.get_file_analysis(file_path)
    else:
        return client.get_full_analysis()


def _extract_ir_summary(ir: dict) -> dict:
    nodes = ir.get("nodes", [])
    edges = ir.get("edges", [])
    id_to_node = {n["id"]: n for n in nodes}

    files: dict[str, dict] = {}

    for node in nodes:
        nt   = node.get("node_type", {})
        meta = node.get("metadata", {})
        file_key = os.path.basename(meta.get("file_path", "unknown")) or "unknown"
        line = meta.get("line_number", 0)

        if file_key not in files:
            files[file_key] = {"functions": [], "classes": []}

        if "Function" in nt:
            fn = nt["Function"]
            files[file_key]["functions"].append({
                "name":   fn.get("name"),
                "params": fn.get("param_count", 0),
                "async":  fn.get("is_async", False),
                "parent": fn.get("parent_scope"),
                "line":   line,
            })
        elif "Class" in nt:
            cls = nt["Class"]
            files[file_key]["classes"].append({
                "name":  cls.get("name"),
                "bases": cls.get("base_classes", []),
                "line":  line,
            })

    def _name(node: dict) -> str | None:
        nt = node.get("node_type", {})
        if "Function" in nt: return nt["Function"].get("name")
        if "Class"    in nt: return nt["Class"].get("name")
        return None

    call_graph:    dict[str, list[str]] = {}
    reverse_graph: dict[str, list[str]] = {}
    edge_list: list[dict] = []

    for edge in edges:
        caller_node = id_to_node.get(edge.get("from", ""))
        callee_node = id_to_node.get(edge.get("to",   ""))

        if not caller_node or not callee_node:
            continue

        caller = _name(caller_node)
        callee = _name(callee_node)
        if not caller or not callee or caller == callee:
            continue

        call_graph.setdefault(caller, [])
        if callee not in call_graph[caller]:
            call_graph[caller].append(callee)

        reverse_graph.setdefault(callee, [])
        if caller not in reverse_graph[callee]:
            reverse_graph[callee].append(caller)

        edge_list.append({"caller": caller, "callee": callee})

    all_functions = {
        fn["name"]
        for f in files.values()
        for fn in f["functions"]
    }
    dead_code_candidates = [
        fn for fn in all_functions
        if fn not in reverse_graph or len(reverse_graph[fn]) == 0
    ]

    return {
        "files":              files,
        "call_graph":         call_graph,
        "reverse_call_graph": reverse_graph,
        "all_edges":          edge_list,
        "dead_code_candidates": dead_code_candidates,
    }


SYSTEM_PROMPT_HEADER = """You are a code analysis assistant for Polycode.
Keep answers short and conversational — 2 to 4 sentences max unless the question genuinely requires more.
Use plain text only. No bullet points, no numbered lists, no bold, no markdown formatting.
Never mention internal field names like "ir_summary", "call_graph", "reverse_call_graph", or "all_edges".
Do not say "according to the call_graph" or similar — just state the facts directly.

For impact questions: trace which functions call the target, then who calls those, recursively.
For diagrams only: produce a Mermaid diagram in a fenced ```mermaid block.

## Codebase Analysis

"""


def build_system_prompt(context_json: dict) -> str:
    source = context_json.get("stats", {}).get("source", "")

    # ── Use cached pre-computed summary for known large repos ────────────
    if source in CACHED_SUMMARIES:
        cache_path = CACHED_SUMMARIES[source]
        try:
            with open(cache_path) as f:
                summary = f.read()
            print(f"[context] using cached summary for {source} ({len(summary)} chars, ~{len(summary)//4} tokens)")
            return SYSTEM_PROMPT_HEADER + summary
        except FileNotFoundError:
            print(f"[context] WARNING: cache file not found at {cache_path}, falling back to IR extraction")

    # ── Normal IR extraction for smaller repos ────────────────────────────
    ir          = context_json.get("ir", {})
    stats       = context_json.get("stats", {})
    suggestions = context_json.get("suggestions", [])

    ir_summary = _extract_ir_summary(ir) if ir and ir.get("nodes") else {}

    lean_context = {
        "source":       stats.get("source", "unknown"),
        "files_parsed": stats.get("files_parsed", 0),
        "ir_summary":   ir_summary,
        "dead_code_suggestions": [
            {
                "function":   s.get("function"),
                "file":       s.get("file"),
                "line":       s.get("line"),
                "suggestion": s.get("suggestion"),
            }
            for s in suggestions
        ],
    }

    analysis_str = json.dumps(lean_context, indent=2)
    print(f"[context] prompt size: {len(analysis_str)} chars (~{len(analysis_str)//4} tokens)")

    return SYSTEM_PROMPT_HEADER + analysis_str