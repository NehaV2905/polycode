from pydantic import BaseModel


class ChatRequest(BaseModel):
    file_path: str | None = None        # null = codebase-wide
    message: str
    conversation_id: str | None = None  # null on first message → server creates UUID


class ChatResponse(BaseModel):
    reply: str
    conversation_id: str