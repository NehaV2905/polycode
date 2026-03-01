"""
Module 4 — Polycode LLM Interface
FastAPI entry point.

Startup:
  cd module4_llm_interface
  uvicorn main:app --reload --port 8080
"""

import os

import anthropic
from dotenv import load_dotenv
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from conversation.store import ConversationStore
from grpc_client.analysis_client import AnalysisClient
from routes.chat import router

load_dotenv()

# ── App ────────────────────────────────────────────────────────────────────

app = FastAPI(
    title="Polycode LLM Interface",
    description="Natural language interface over Polycode static analysis.",
    version="1.0.0",
)

# ── CORS — open for local dev ──────────────────────────────────────────────

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
)

# ── Shared state — initialised once at startup ─────────────────────────────

@app.on_event("startup")
async def startup():
    module3_addr = os.getenv("MODULE3_ADDR", "http://127.0.0.1:50052")
    api_key      = os.getenv("ANTHROPIC_API_KEY")

    if not api_key:
        raise RuntimeError("ANTHROPIC_API_KEY is not set in .env")

    app.state.grpc_client = AnalysisClient(module3_addr)
    app.state.llm_client  = anthropic.Anthropic(api_key=api_key)
    app.state.store       = ConversationStore()

    print(f"[module4] Connected to Module 3 at {module3_addr}")
    print(f"[module4] Anthropic client ready")
    print(f"[module4] Listening on http://0.0.0.0:8080")


@app.on_event("shutdown")
async def shutdown():
    app.state.grpc_client.close()
    print("[module4] Shutdown complete")


# ── Routes ─────────────────────────────────────────────────────────────────

app.include_router(router)