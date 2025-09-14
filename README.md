# RAG System MVP

A minimal working RAG (Retrieval-Augmented Generation) system implementation that demonstrates core functionality.

## Features

- **Document Processing**: Processes text files and extracts content with metadata
- **Chunking Strategies**: Fixed-size and paragraph-based chunking
- **Search Functionality**: Simple keyword-based search with scoring
- **Evaluation Framework**: Basic metrics for search quality assessment
- **Storage Management**: In-memory document and chunk storage
- **CLI Interface**: Command-line interface for testing functionality

## Quick Start

1. Clone this repository:
```bash
git clone <repository-url>
cd rig-rag-system
```

2. Build the MVP:
```bash
cd rag_mvp
cargo build
```

3. Process a document:
```bash
./target/debug/rag_mvp process path/to/document.txt
```

4. Search for content:
```bash
./target/debug/rag_mvp search "your query"
```

5. Run the test suite:
```bash
cargo run --bin test_mvp
```

## Project Structure

```
rig-rag-system/
├── rag_mvp/
│   ├── Cargo.toml              # Dependencies and project configuration
│   ├── src/
│   │   ├── lib.rs              # Main library exports and tests
│   │   ├── main.rs             # CLI interface
│   │   ├── chunking.rs        # Document chunking strategies
│   │   ├── processor.rs        # File processing
│   │   ├── search.rs          # Search functionality
│   │   ├── storage.rs          # In-memory storage
│   │   └── evaluation.rs      # Quality evaluation
│   ├── README.md               # MVP documentation
│   └── Cargo.lock              # Dependency lock file
├── test_mvp.rs                 # Integration test
└── README.md                   # This file
```

## CLI Commands

#### Process a Document
```bash
./target/debug/rag_mvp process path/to/document.txt
```

#### Search Documents
```bash
./target/debug/rag_mvp search "your query" --limit 5
```

#### Evaluate Search Quality
```bash
./target/debug/rag_mvp evaluate "your query" --expected "doc1,doc2"
```

#### List Processed Documents
```bash
./target/debug/rag_mvp list
```

#### View Storage Statistics
```bash
./target/debug/rag_mvp stats
```

## Testing

Run the comprehensive test suite:
```bash
cargo run --bin test_mvp
```

Example output:
```
Testing MVP RAG System...
✓ Created test file: /tmp/test_mvp.txt
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

The MVP consists of these core components:

1. **DocumentProcessor**: Handles file reading and content extraction
2. **ChunkingEngine**: Splits documents into searchable chunks
3. **SearchEngine**: Performs keyword-based search with scoring
4. **StorageManager**: Manages in-memory storage of documents and chunks
5. **Evaluator**: Calculates search quality metrics

## Limitations

- **Storage**: Currently uses in-memory storage (no persistence between runs)
- **Search**: Simple keyword matching without semantic understanding
- **Document Types**: Only supports plain text files
- **Scalability**: Designed for demonstration, not production use

## Next Steps

This MVP provides a foundation for building a more comprehensive RAG system with:
- Persistent storage (SQLite, vector databases)
- Advanced search algorithms (semantic search, embeddings)
- Support for multiple document formats (PDF, Markdown, etc.)
- Production-ready error handling and logging
- Performance optimization and caching