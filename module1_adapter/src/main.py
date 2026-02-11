"""
Main Entry Point - LSP Listener & Orchestrator

This module ties everything together:
- Listens to file changes via LSP
- Triggers parser to extract facts
- Sends events via transport layer

Philosophy: Simple orchestration. No cleverness.
"""

import asyncio
import sys
import os
import argparse
from pathlib import Path
from typing import Optional
from urllib.parse import unquote, urlparse

# Try to import LSP support (optional for standalone mode)
LSP_AVAILABLE = False
try:
    from lsprotocol.types import (
        TEXT_DOCUMENT_DID_OPEN,
        TEXT_DOCUMENT_DID_CHANGE,
        TEXT_DOCUMENT_DID_SAVE,
    )
    from lsprotocol.types import (
        DidOpenTextDocumentParams,
        DidChangeTextDocumentParams,
        DidSaveTextDocumentParams,
    )
    from pygls.server import LanguageServer
    LSP_AVAILABLE = True
except ImportError:
    print("[Warning] pygls not available. LSP mode disabled. Standalone mode still works.")
    pass


# Local modules
from parser_logic import extract_facts_from_source, extract_facts_from_file
from transport import IREventPublisher, start_grpc_server


class LanguageAdapter:
    """
    Main adapter class that coordinates LSP listening, parsing, and event emission.
    """
    
    def __init__(self, language: str = "python", grpc_port: int = 50051):
        self.language = language
        self.grpc_port = grpc_port
        self.publisher = IREventPublisher(language=language)
        self.grpc_server = None
        self.grpc_service = None
        
        if LSP_AVAILABLE:
            self.lsp_server = LanguageServer('IR-Adapter', 'v0.1')
            # Register LSP handlers
            self._register_lsp_handlers()
        else:
            self.lsp_server = None

    
    def _register_lsp_handlers(self):
        """Register LSP event handlers."""
        
        @self.lsp_server.feature(TEXT_DOCUMENT_DID_OPEN)
        async def on_open(ls: LanguageServer, params: DidOpenTextDocumentParams):
            """Handle file open event."""
            print(f"[LSP] File opened: {params.text_document.uri}")
            await self._process_document(
                params.text_document.uri,
                params.text_document.text
            )
        
        @self.lsp_server.feature(TEXT_DOCUMENT_DID_CHANGE)
        async def on_change(ls: LanguageServer, params: DidChangeTextDocumentParams):
            """Handle file change event."""
            print(f"[LSP] File changed: {params.text_document.uri}")
            
            # Get the latest content
            if params.content_changes:
                latest_content = params.content_changes[-1].text
                await self._process_document(
                    params.text_document.uri,
                    latest_content
                )
        
        @self.lsp_server.feature(TEXT_DOCUMENT_DID_SAVE)
        async def on_save(ls: LanguageServer, params: DidSaveTextDocumentParams):
            """Handle file save event."""
            print(f"[LSP] File saved: {params.text_document.uri}")
            
            # Re-process the file
            file_path = self._uri_to_path(params.text_document.uri)
            if file_path:
                facts = extract_facts_from_file(file_path)
                self._emit_facts(facts, file_path)
    
    async def _process_document(self, uri: str, content: str):
        """
        Process a document and extract IR facts.
        
        Args:
            uri: Document URI
            content: Document content
        """
        file_path = self._uri_to_path(uri)
        if not file_path:
            return
        
        print(f"[Parser] Processing {file_path}...")
        facts = extract_facts_from_source(content, file_path)
        
        print(f"[Parser] Extracted {len(facts)} facts")
        self._emit_facts(facts, file_path)
    
    def _emit_facts(self, facts, file_path: str):
        """
        Convert facts to IR events and emit them.
        
        Args:
            facts: List of IRFact objects
            file_path: Source file path
        """
        for fact in facts:
            event = self.publisher.publish_fact(fact, file_path)
            
            # Print event for debugging
            event_type = event.WhichOneof('event')
            print(f"[Event] {event_type} at line {fact.line_number}")
            
            # Queue for gRPC streaming
            if self.grpc_service:
                self.grpc_service.publisher.emit_event(event)
    
    def _uri_to_path(self, uri: str) -> Optional[str]:
        """
        Convert LSP URI to file path (cross-platform).
        
        Handles file:// URIs on Windows, macOS, and Linux.
        
        Args:
            uri: LSP URI (e.g., "file:///path/to/file")
        
        Returns:
            Absolute file path as string, or None if invalid
        """
        if not uri.startswith("file://"):
            return None
        
        # Parse the URI
        parsed = urlparse(uri)
        
        # Decode percent-encoded characters
        path = unquote(parsed.path)
        
        # Platform-specific handling
        if sys.platform == "win32":
            # On Windows, file:///C:/path becomes /C:/path
            # We need to remove the leading slash
            if path.startswith("/") and len(path) > 2 and path[2] == ":":
                path = path[1:]  # Remove leading /
        
        # Convert to Path object for normalization
        path_obj = Path(path)
        
        # Return absolute path as string
        return str(path_obj.absolute())
    
    async def start_lsp(self):
        """Start the LSP server."""
        print(f"[LSP] Starting Language Adapter for {self.language}...")
        await self.lsp_server.start_io()
    
    def start_grpc(self):
        """Start the gRPC server."""
        self.grpc_server, self.grpc_service = start_grpc_server(self.grpc_port)
    
    def run(self):
        """Run the adapter."""
        if not LSP_AVAILABLE:
            print("[Error] LSP mode requires pygls. Please install: pip install pygls")
            print("[Info] Use --mode file for standalone file processing")
            return
        
        # Start gRPC server in background
        self.start_grpc()
        
        # Start LSP server (blocks until shutdown)
        print(f"[Main] Language Adapter running...")
        print(f"[Main] - LSP: stdin/stdout")
        print(f"[Main] - gRPC: localhost:{self.grpc_port}")
        
        try:
            asyncio.run(self.start_lsp())
        except KeyboardInterrupt:
            print("\n[Main] Shutting down...")
            if self.grpc_server:
                self.grpc_server.stop(0)


def standalone_file_mode(file_path: str, language: str = "python"):
    """
    Process a single file without LSP (for testing).
    
    Args:
        file_path: Path to the file to process
        language: Programming language
    """
    print(f"[Standalone] Processing {file_path}...")
    
    # Extract facts
    facts = extract_facts_from_file(file_path)
    
    print(f"\n{'='*60}")
    print(f"Extracted {len(facts)} IR facts from {file_path}")
    print(f"{'='*60}\n")
    
    # Convert to events and print
    publisher = IREventPublisher(language=language)
    for fact in facts:
        event = publisher.publish_fact(fact, file_path)
        event_type = event.WhichOneof('event')
        
        # Pretty print the event
        print(f"[{fact.line_number:3d}] {event_type:20s} | {fact.data}")
    
    print(f"\n{'='*60}\n")


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="Module 1 - Language Adapter for IR Event Extraction"
    )
    parser.add_argument(
        "--mode",
        choices=["lsp", "file"],
        default="lsp",
        help="Run mode: 'lsp' for LSP server, 'file' for single file processing"
    )
    parser.add_argument(
        "--file",
        type=str,
        help="File to process (required for 'file' mode)"
    )
    parser.add_argument(
        "--language",
        type=str,
        default="python",
        help="Programming language (default: python)"
    )
    parser.add_argument(
        "--grpc-port",
        type=int,
        default=50051,
        help="gRPC server port (default: 50051)"
    )
    
    args = parser.parse_args()
    
    if args.mode == "file":
        if not args.file:
            print("Error: --file is required for 'file' mode")
            sys.exit(1)
        
        standalone_file_mode(args.file, args.language)
    
    else:  # LSP mode
        adapter = LanguageAdapter(
            language=args.language,
            grpc_port=args.grpc_port
        )
        adapter.run()


if __name__ == "__main__":
    main()