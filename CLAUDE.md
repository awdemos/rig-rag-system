# Enhanced RAG System Development Guidelines

Auto-generated from feature plans. Last updated: 2025-09-13

## Active Technologies
- **Language**: Rust 1.75+
- **Primary Dependencies**:
  - Rig framework (core RAG functionality)
  - Tokio (async runtime)
  - Serde (JSON serialization)
  - Reqwest (HTTP client for web content)
  - Scraper (HTML parsing)
  - Thiserror (structured error handling)
  - Tracing (structured logging)
  - Clap (CLI argument parsing)
- **Testing**: cargo test, mockall for mocking
- **Storage**: JSON configuration files (~/.rag_config.json) + in-memory vector store
- **Project Type**: Single project (enhanced CLI application)

## Project Structure
```
specs/001-enhanced-rag-system/
├── plan.md              # Implementation plan
├── research.md          # Research decisions
├── data-model.md        # Data models and schemas
├── quickstart.md        # Quick start guide
└── contracts/
    └── api.yaml         # OpenAPI contract

rag_system/
├── Cargo.toml           # Dependencies
├── src/
│   ├── main.rs          # CLI entry point
│   ├── lib.rs           # Library exports
│   ├── config.rs        # Configuration management
│   ├── document.rs      # Document processing (enhanced)
│   ├── chunking.rs      # Chunking strategies (new)
│   ├── rag_engine.rs    # RAG processing (enhanced)
│   ├── conversation.rs  # Conversation management (new)
│   └── error.rs         # Error handling (enhanced)
└── tests/
    ├── contract/
    ├── integration/
    └── unit/
```

## Commands
```bash
# Build and test
cargo build
cargo test
cargo run -- --help

# Format and lint
cargo fmt
cargo clippy

# Run enhanced CLI
cargo run -- document add path/to/document.pdf
cargo run -- config show
cargo run -- chat
cargo run -- query "your question"

# Document management
cargo run -- document list
cargo run -- document status <id>
cargo run -- document remove <id>

# Configuration
cargo run -- config save <profile-name>
cargo run -- config load <profile-name>
```

## Code Style
- **Rust**: Follow Rust API Guidelines (RAG)
- **Error Handling**: Use thiserror for custom error types, anyhow for general errors
- **Async**: Use tokio for async operations
- **Logging**: Use tracing structured logging
- **Configuration**: Use serde for JSON serialization
- **Testing**: TDD approach with mockall for mocking

## Recent Changes
- **001-enhanced-rag-system**: Adding multi-document support, web content processing, advanced chunking strategies, conversation management, and configuration profiles

<!-- MANUAL ADDITIONS START -->

## Development Notes
- Support for PDF, text, and web document types
- Multiple chunking strategies: fixed_size, paragraph, semantic, heading
- Conversation context management with configurable history
- Configuration profiles with environment variable overrides
- Streaming document processing for memory efficiency
- Structured error handling with comprehensive logging
- Hot-reloadable configuration for development

## Key Components
- **Document Processing**: Trait-based design for different document types with factory pattern
- **Chunking Strategies**: Configurable chunking with overlap and boundary respect
- **RAG Engine**: Enhanced retrieval with cross-document search and relevance scoring
- **Conversation Manager**: Session-based conversation with context window management
- **Configuration System**: Layered configuration (defaults → file → env vars)

<!-- MANUAL ADDITIONS END -->