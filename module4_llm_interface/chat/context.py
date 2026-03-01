"""
Fetch analysis context from Module 3 and assemble the Claude system prompt.

Called once per conversation at creation time — never again for that session.
"""

import json

from grpc_client import AnalysisClient


def fetch_context(client: AnalysisClient, file_path: str | None) -> dict:
    """
    Fetch the appropriate analysis from Module 3.

    - file_path is None  → GetFullAnalysis (codebase-wide)
    - file_path is set   → all four file-scoped RPCs combined
    """
    if file_path is None:
        return client.get_full_analysis()
    else:
        return client.get_file_analysis(file_path)


def build_system_prompt(context_json: dict) -> str:
    """
    Assemble the Claude system prompt from the analysis context JSON.
    This string is fixed for the lifetime of the conversation.
    """
    analysis_str = json.dumps(context_json, indent=2)

    return f"""You are a code analysis assistant for the Polycode system. \
You have been given structured analysis data about a codebase in JSON format below. \
Use this data to answer the user's questions accurately. \
Do not make up information that is not present in the data.

If the user asks for a flowchart or diagram, produce a Mermaid diagram in a fenced code block.
If the user asks a follow-up question, use the conversation history for context.

## Analysis Data

{analysis_str}"""