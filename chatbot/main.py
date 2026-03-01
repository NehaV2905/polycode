"""
Module 4 — Polycode Chatbot
FastAPI entry point.

Startup:
  cd chatbot
  $env:PYTHONPATH = "."
  uvicorn main:app --reload --port 8080
"""

import os

from groq import Groq
from dotenv import load_dotenv
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from conversation.store import ConversationStore
from grpc_client.analysis_client import AnalysisClient
from routes.chat import router

load_dotenv()

app = FastAPI(
    title="Polycode Chatbot",
    description="Natural language interface over Polycode static analysis.",
    version="1.0.0",
)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
)

@app.on_event("startup")
async def startup():
    api_server_addr = os.getenv("API_SERVER_ADDR", "http://127.0.0.1:3000")
    groq_key        = os.getenv("GROQ_API_KEY")

    if not groq_key:
        raise RuntimeError("GROQ_API_KEY is not set in .env")

    app.state.grpc_client = AnalysisClient(api_server_addr)
    app.state.llm_client  = Groq(api_key=groq_key)
    app.state.store       = ConversationStore()

    print(f"[chatbot] Connected to api_server at {api_server_addr}")
    print(f"[chatbot] Groq client ready")
    print(f"[chatbot] Listening on http://0.0.0.0:8080")


@app.on_event("shutdown")
async def shutdown():
    app.state.grpc_client.close()
    print("[chatbot] Shutdown complete")


app.include_router(router)