from dataclasses import dataclass, field
from datetime import datetime
from typing import Optional
import uuid


@dataclass
class Conversation:
    id:           str
    file_path:    Optional[str]        # locked at creation; None = codebase-wide
    messages:     list                 # [{"role": "user"|"assistant", "content": str}]
    context_json: dict                 # Module 3 analysis — fetched once, reused every turn
    created_at:   datetime = field(default_factory=datetime.utcnow)
    last_active:  datetime = field(default_factory=datetime.utcnow)


class ConversationStore:
    def __init__(self):
        self._store: dict[str, Conversation] = {}

    def create(self, file_path: Optional[str], context_json: dict) -> Conversation:
        conv = Conversation(
            id=str(uuid.uuid4()),
            file_path=file_path,
            messages=[],
            context_json=context_json,
        )
        self._store[conv.id] = conv
        return conv

    def get(self, conversation_id: str) -> Optional[Conversation]:
        return self._store.get(conversation_id)

    def append_message(self, conversation_id: str, role: str, content: str) -> None:
        conv = self._store.get(conversation_id)
        if conv is None:
            raise KeyError(f"Conversation {conversation_id} not found")
        conv.messages.append({"role": role, "content": content})
        conv.last_active = datetime.utcnow()

    def delete(self, conversation_id: str) -> bool:
        if conversation_id in self._store:
            del self._store[conversation_id]
            return True
        return False

    def exists(self, conversation_id: str) -> bool:
        return conversation_id in self._store