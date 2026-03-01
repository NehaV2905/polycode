"""
REST endpoints for Module 4.

POST   /chat                      — main chat endpoint
GET    /health                    — liveness + Module 3 connectivity check
DELETE /conversation/{id}         — clear a session
"""

from fastapi import APIRouter, HTTPException, Request

from models.request import ChatRequest, ChatResponse
from chat.context import fetch_context
from chat.answerer import get_answer

router = APIRouter()


# ── POST /chat ─────────────────────────────────────────────────────────────

@router.post("/chat", response_model=ChatResponse)
async def chat(body: ChatRequest, request: Request):
    store  = request.app.state.store
    client = request.app.state.grpc_client
    llm    = request.app.state.llm_client

    # ── Resolve or create conversation ─────────────────────────────────────
    if body.conversation_id is None:
        # First message — fetch context once and lock the scope
        try:
            context_json = fetch_context(client, body.file_path)
        except Exception as e:
            raise HTTPException(
                status_code=502,
                detail=f"Failed to fetch analysis from Module 3: {e}",
            )
        conversation = store.create(
            file_path=body.file_path,
            context_json=context_json,
        )
    else:
        conversation = store.get(body.conversation_id)
        if conversation is None:
            raise HTTPException(
                status_code=404,
                detail=f"Conversation '{body.conversation_id}' not found.",
            )

    # ── Call Claude ────────────────────────────────────────────────────────
    try:
        reply = get_answer(llm, conversation, body.message)
    except Exception as e:
        raise HTTPException(
            status_code=502,
            detail=f"Anthropic API error: {e}",
        )

    # ── Persist turn to history ────────────────────────────────────────────
    conversation.append("user", body.message)
    conversation.append("assistant", reply)

    return ChatResponse(reply=reply, conversation_id=conversation.id)


# ── GET /health ────────────────────────────────────────────────────────────

@router.get("/health")
async def health(request: Request):
    client = request.app.state.grpc_client
    stats  = client.health_check()
    return {
        "status": "ok",
        "module3_connected": stats["ok"],
        "module3_graph_stats": {
            "node_count": stats["node_count"],
            "edge_count": stats["edge_count"],
            "file_count": stats["file_count"],
        },
    }


# ── DELETE /conversation/{id} ──────────────────────────────────────────────

@router.delete("/conversation/{conversation_id}")
async def delete_conversation(conversation_id: str, request: Request):
    store = request.app.state.store
    deleted = store.delete(conversation_id)
    if not deleted:
        raise HTTPException(
            status_code=404,
            detail=f"Conversation '{conversation_id}' not found.",
        )
    return {"deleted": True, "conversation_id": conversation_id}