# Polycode — Universal Code Analysis Platform

Polycode translates source code from any supported language into a single, language-agnostic Intermediate Representation (IR), then runs static analysis, dependency graphing, dead code detection, and AI-powered fix suggestions — all surfaced through a React UI.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  React UI  (ui/)          Vite + TypeScript + Cytoscape.js  │
└───────────────────────────────┬─────────────────────────────┘
                                │ HTTP  /api/*
┌───────────────────────────────▼─────────────────────────────┐
│  api_server  (Rust/axum)      port 3000                     │
│  Orchestrates M2 → M3 → AI pipeline, serves JSON to UI      │
└────────┬──────────────────────────────────────────┬─────────┘
         │ gRPC  :50051                              │ Groq API
┌────────▼────────┐   builds graph   ┌──────────────▼───────┐
│  Module 1       │ ───────────────► │  Module 2 (lib)      │
│  Python/gRPC    │                  │  IR Graph Builder    │
│  Tree-sitter    │                  │  Rust                │
│  6 languages    │                  └──────────────────────┘
└─────────────────┘                           │
                                    ┌─────────▼────────────┐
                                    │  Module 3 (lib)      │
                                    │  Analysis Engine     │
                                    │  Dead code, calls,   │
                                    │  impact analysis     │
                                    └──────────────────────┘
```

**Supported languages:** Python · Java · Go · Rust · Ruby · C

---

## Prerequisites

| Tool | Version |
|------|---------|
| Python | 3.10+ |
| Rust / Cargo | 1.75+ |
| Node.js | 18+ |
| npm | 9+ |

---

## Setup

### 1. Python dependencies (Module 1)

```bash
pip install -r requirements.txt
```

### 2. Rust workspace

```bash
cargo build --release
```

### 3. UI dependencies

```bash
cd ui && npm install
```

---

## Running the System

The system requires **three processes** running concurrently. Open three terminals.

### Terminal 1 — Module 1 (gRPC server)

Module 1 must be started first; all other modules connect to it on port 50051.

```bash
python test_integration_v3.py
```

### Terminal 2 — API server

The API server bridges the UI to the analysis pipeline.

```bash
# Optional: set GROQ_API_KEY for AI fix suggestions
export GROQ_API_KEY=gsk_...

cargo run -p api_server
# Listening on http://0.0.0.0:3000
```

### Terminal 3 — UI dev server

```bash
cd ui && npm run dev
# Open http://localhost:5173
```

Open http://localhost:5173 in your browser, then either:
- **Upload files** — drag one or more source files into the sidebar
- **Enter a repo URL** — paste a GitHub/GitLab HTTPS URL and click Analyse

---

## CLI Usage (without UI)

Each module can also be run standalone from the command line.

### Module 3 — Analysis engine

```bash
# Requires Module 1 running on :50051
cargo run -p module3_analysis -- --file path/to/file.py
cargo run -p module3_analysis -- --repo https://github.com/owner/repo

# Run impact analysis on a specific function
cargo run -p module3_analysis -- --file path/to/file.py --impact-target login
```

### Module 4 — AI fix suggester

```bash
export GROQ_API_KEY=gsk_...

cargo run -p module4_fixer -- --file path/to/file.py
cargo run -p module4_fixer -- --repo https://github.com/owner/repo
cargo run -p module4_fixer -- --file path/to/file.py --max-fixes 25
```

---

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `M1_SERVER` | `http://127.0.0.1:50051` | Module 1 gRPC address |
| `GROQ_API_KEY` | *(unset)* | Groq API key for AI suggestions |
| `PORT` | `3000` | API server listen port |

---

## Project Structure

```
polycode/
├── module1_adapter/        # Python — Tree-sitter parser, gRPC server
├── module2_ir_builder/     # Rust  — IR graph construction (library)
├── module3_analysis/       # Rust  — Static analysis engine (library + binary)
├── module4_fixer/          # Rust  — AI fix suggester (binary, CLI only)
├── api_server/             # Rust  — HTTP API bridging UI and pipeline
├── ui/                     # TypeScript/React — web frontend
├── Cargo.toml              # Rust workspace
└── requirements.txt        # Python dependencies (Module 1)
```
