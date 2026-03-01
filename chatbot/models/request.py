from pydantic import BaseModel
from typing import Any


class ChatRequest(BaseModel):
    message:         str
    conversation_id: str | None = None
    # On first message only — pass analysis result directly from the UI
    context:         dict | None = None


class ChatResponse(BaseModel):
    reply:           str
    conversation_id: str