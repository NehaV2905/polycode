"""
Fetch analysis context from api_server and assemble the Claude system prompt.
Called once per conversation at creation time.
"""

import json

from grpc_client.analysis_client import AnalysisClient


def fetch_context(client: AnalysisClient, file_path: str | None) -> dict:
    """
    Fetch analysis from api_server.

    - file_path provided → upload that file, get full analysis
    - file_path is None  → return placeholder asking user to specify a file
    """
    if file_path is None:
        return client.get_full_analysis()
    else:
        return client.get_file_analysis(file_path)


def build_system_prompt(context_json: dict) -> str:
    """
    Assemble the Claude system prompt from the analysis context JSON.
    Strips the raw IR graph (UUID nodes/edges) — only semantic summaries
    like suggestions, stats, call graph, dead code are sent to the LLM.
    Fixed for the lifetime of the conversation.
    """
    # Raw IR is internal plumbing — UUIDs and edge metadata add thousands of
    # tokens but carry no meaning the LLM can reason over.
    lean_context = {k: v for k, v in context_json.items() if k != "ir"}
    analysis_str = json.dumps(lean_context, separators=(',', ':'))

    print(f"[debug] prompt size: {len(analysis_str)} chars (~{len(analysis_str)//4} tokens)")

    return f"""You are a code analysis assistant for the Polycode system. \
You have been given structured analysis data about a codebase in JSON format below. \
Use this data to answer the user's questions accurately. \
Do not make up information that is not present in the data.

If the user asks for a flowchart or diagram, produce a Mermaid diagram in a fenced code block.
If the user asks a follow-up question, use the conversation history for context.

## Analysis Data

{analysis_str}"""