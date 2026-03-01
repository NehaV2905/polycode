from fastapi import APIRouter, HTTPException, Request
from models.request import ChatRequest, ChatResponse
from chat.answerer import get_answer
from chat.context import build_system_prompt

router = APIRouter()


@router.post("/chat", response_model=ChatResponse)
async def chat(body: ChatRequest, request: Request):
    store = request.app.state.store
    llm   = request.app.state.llm_client

    if body.conversation_id is None:
        # First message — context must be provided by the UI
        if not body.context:
            raise HTTPException(
                status_code=400,
                detail="context is required on the first message.",
            )
        conversation = store.create(file_path=None, context_json=body.context)
    else:
        conversation = store.get(body.conversation_id)
        if conversation is None:
            raise HTTPException(
                status_code=404,
                detail=f"Conversation '{body.conversation_id}' not found.",
            )

    try:
        reply = get_answer(llm, conversation, body.message)
    except Exception as e:
        raise HTTPException(status_code=502, detail=f"LLM API error: {e}")

    store.append_message(conversation.id, "user", body.message)
    store.append_message(conversation.id, "assistant", reply)

    return ChatResponse(reply=reply, conversation_id=conversation.id)


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