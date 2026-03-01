"""
Single Claude Sonnet call per user message.

Receives the full conversation history + system prompt (built once at
conversation creation) and returns Claude's reply as a plain string.
"""

import anthropic

from conversation.store import Conversation
from chat.context import build_system_prompt


def get_answer(client: anthropic.Anthropic, conversation: Conversation, user_message: str) -> str:
    """
    Call Claude Sonnet with:
      - system prompt: analysis JSON context (fixed for this conversation)
      - messages: full conversation history + the new user message

    Returns Claude's reply as a plain string.
    """
    system_prompt = build_system_prompt(conversation.context_json)

    # Build messages: existing history + new user message
    messages = conversation.messages + [{"role": "user", "content": user_message}]

    response = client.messages.create(
        model="claude-sonnet-4-6",
        max_tokens=4096,
        system=system_prompt,
        messages=messages,
    )

    return response.content[0].text