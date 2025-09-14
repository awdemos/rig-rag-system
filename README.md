# RAG System

A working RAG (Retrieval-Augmented Generation) system implementation that demonstrates core functionality.

## Features

- **Document Processing**: Processes text files and extracts content with metadata
- **Chunking Strategies**: Fixed-size and paragraph-based chunking
- **Search Functionality**: Simple keyword-based search with scoring
- **Evaluation Framework**: Basic metrics for search quality assessment
- **Storage Management**: In-memory document and chunk storage
- **CLI Interface**: Command-line interface for testing functionality

## Supported Document Types

The system currently supports **plain text files** with `.txt` extension. This includes:

- `.txt` files - Standard text documents
- `.md` files - Markdown documents (treated as plain text)
- Any text-based file with content that can be read as UTF-8

## Quick Start

1. Clone this repository:
```bash
git clone <repository-url>
cd rig-rag-system
```

2. Build the system:
```bash
cargo build
```

3. Process a text document:
```bash
./target/debug/rag-system process path/to/document.txt
```

4. Search for content:
```bash
./target/debug/rag-system search "your query"
```

5. Run the test suite:
```bash
cargo run --bin test_rag
```

## Text File Processing

The system handles text files by:
- Reading the entire file content as UTF-8
- Extracting metadata including file path, size, and word count
- Supporting any text-based format that can be read as plain text
- Preserving original formatting and structure during chunking

### Example Text Files

```
# Supported file types:
document.txt          # Standard text file
notes.md             # Markdown file (treated as text)
readme.txt           # Documentation
research.txt        # Research papers
article.txt          # Articles and essays
```

## Project Structure

```
rig-rag-system/
├── Cargo.toml                    # Dependencies and project configuration
├── Cargo.lock                    # Dependency lock file
├── src/
│   ├── lib.rs                    # Main library exports and tests
│   ├── main.rs                   # CLI interface
│   ├── chunking.rs              # Document chunking strategies
│   ├── processor.rs              # File processing
│   ├── search.rs                # Search functionality
│   ├── storage.rs                # In-memory storage
│   └── evaluation.rs            # Quality evaluation
├── test_mvp.rs                   # Integration test
└── README.md                     # This file
```

## CLI Commands

#### Process a Document
```bash
./target/debug/rag-system process path/to/document.txt
```

#### Search Documents
```bash
./target/debug/rag-system search "your query" --limit 5
```

#### Evaluate Search Quality
```bash
./target/debug/rag-system evaluate "your query" --expected "doc1,doc2"
```

#### List Processed Documents
```bash
./target/debug/rag-system list
```

#### View Storage Statistics
```bash
./target/debug/rag-system stats
```

## Testing

Run the comprehensive test suite:
```bash
cargo run --bin test_rag
```

Example output:
```
Testing RAG System...
✓ Created test file: /tmp/test_rag.txt
✓ Created RAG system
✓ Processed document with ID: 626a6491-6794-4829-a8fc-7d00df52ea5c

Testing search functionality:
Found 1 results for 'machine learning':
  1. [Score: 1.000] # Machine Learning Basics...

Testing evaluation:
Evaluation metrics:
  Relevance: 1.000
  Precision: 1.000
  Recall: 1.000
  F1 Score: 1.000

Storage statistics:
  Total Documents: 1
  Total Chunks: 1
  Total Size: 563 bytes

✓ Test completed successfully!
```

## Architecture

The system consists of these core components:

1. **DocumentProcessor**: Handles text file reading and content extraction
2. **ChunkingEngine**: Splits documents into searchable chunks
3. **SearchEngine**: Performs keyword-based search with scoring
4. **StorageManager**: Manages in-memory storage of documents and chunks
5. **Evaluator**: Calculates search quality metrics

## Limitations

- **Storage**: Currently uses in-memory storage (no persistence between runs)
- **Search**: Simple keyword matching without semantic understanding
- **Document Types**: Only supports plain text files (.txt, .md, etc.)
- **Scalability**: Designed for demonstration, not production use

## Next Steps

This system provides a foundation for building a more comprehensive RAG system with:
- Persistent storage (SQLite, vector databases)
- Advanced search algorithms (semantic search, embeddings)
- Support for multiple document formats (PDF, structured files)
- Production-ready error handling and logging
- Performance optimization and caching