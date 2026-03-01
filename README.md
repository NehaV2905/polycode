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

### System tools

| Tool | Version | Purpose |
|------|---------|---------|
| Python | 3.10+ | Module 1 (language adapter) |
| pip | 22+ | Python package manager |
| Rust + Cargo | 1.75+ | Modules 2–4 and API server |
| protoc | 3.x+ | Compile `.proto` files for gRPC (required before `cargo build`) |
| Node.js | 18+ | React UI dev server |
| npm | 9+ | UI package manager |
| git | any | Runtime — used to clone repos for analysis |

### Installing Rust

If Rust is not installed, use [rustup](https://rustup.rs):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Installing protoc

`protoc` is required to compile the gRPC protobuf definitions used by Module 2.

**macOS**
```bash
brew install protobuf
```

**Ubuntu / Debian**
```bash
sudo apt-get install -y protobuf-compiler
```

**Windows**
Download the latest release from https://github.com/protocolbuffers/protobuf/releases
and add the `bin/` directory to your `PATH`.

Verify:
```bash
protoc --version   # should print libprotoc 3.x or higher
```

### Installing Node.js

**macOS**
```bash
brew install node
```

**Ubuntu / Debian**
```bash
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs
```

**Windows / all platforms** — download from https://nodejs.org

---

## Setup

### 1. Python dependencies (Module 1)

```bash
pip install -r requirements.txt
```

### 2. Rust workspace

Requires `protoc` to be installed and on your `PATH` before running this.

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
