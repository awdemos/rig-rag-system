# Quick Start Guide: Enhanced RAG System

## Prerequisites

- Rust 1.75+ installed
- OpenAI API key
- Documents to process (PDF, text, or web URLs)

## Installation

1. **Clone and build the project**:
```bash
git clone <repository-url>
cd rig-rag-system
cargo build --release
```

2. **Set up your OpenAI API key**:
```bash
export OPENAI_API_KEY=your-api-key-here
```

3. **Verify installation**:
```bash
cargo run -- --help
```

## Basic Usage

### 1. Process Documents

**Add PDF documents**:
```bash
# Add single PDF
cargo run -- document add path/to/document.pdf

# Add multiple PDFs
cargo run -- document add documents/*.pdf

# Add with custom chunking
cargo run -- document add --chunk-size 1500 --overlap 200 path/to/document.pdf
```

**Add text documents**:
```bash
# Add text file
cargo run -- document add --type text path/to/document.txt

# Add from directory
cargo run -- document add --type text documents/texts/
```

**Add web content**:
```bash
# Add single URL
cargo run -- document add --type web https://example.com/article

# Add multiple URLs
cargo run -- document add --type web https://site1.com https://site2.com
```

### 2. Configure the System

**View current configuration**:
```bash
cargo run -- config show
```

**Customize chunking**:
```bash
cargo run -- config set chunking.strategy semantic
cargo run -- config set chunking.chunk-size 2000
cargo run -- config set chunking.overlap 300
```

**Configure conversation settings**:
```bash
cargo run -- config set conversation.max-context 10
cargo run -- config set conversation.temperature 0.7
```

**Save configuration**:
```bash
cargo run -- config save my-profile
cargo run -- config load my-profile
```

### 3. Start Conversation

**Basic conversation**:
```bash
cargo run -- chat
```

**Chat with specific documents**:
```bash
cargo run -- chat --documents doc1.pdf doc2.pdf
```

**Continue previous conversation**:
```bash
cargo run -- chat --session session-id
```

### 4. Query Documents

**Single query**:
```bash
cargo run -- query "What are the main themes discussed?"
```

**Query with source documents**:
```bash
cargo run -- query --documents report.pdf "Summarize the findings"
```

**Interactive query mode**:
```bash
cargo run -- query --interactive
```

## Advanced Usage

### Document Management

**List all documents**:
```bash
cargo run -- document list
```

**Check document status**:
```bash
cargo run -- document status document-id
```

**Remove documents**:
```bash
cargo run -- document remove document-id
```

**Reindex documents**:
```bash
cargo run -- document reindex document-id
```

### Conversation Management

**Save conversation**:
```bash
cargo run -- conversation save my-conversation
```

**Load conversation**:
```bash
cargo run -- conversation load my-conversation
```

**List conversations**:
```bash
cargo run -- conversation list
```

**Export conversation**:
```bash
cargo run -- conversation export my-conversation --format json
```

### Configuration Profiles

**Create custom profile**:
```bash
cargo run -- config create research-profile --chunk-size 1000 --max-results 5
```

**Switch between profiles**:
```bash
cargo run -- config use research-profile
```

**Edit configuration file**:
```bash
cargo run -- config edit
```

### Performance Tuning

**Adjust for large documents**:
```bash
cargo run -- config set performance.max-memory 2048
cargo run -- config set processing.max-file-size 52428800  # 50MB
```

**Optimize for speed**:
```bash
cargo run -- config set processing.parallel true
cargo run -- config set performance.cache-embeddings true
```

**Batch processing**:
```bash
cargo run -- document add --batch documents/
```

## Examples

### Example 1: Research Paper Analysis

```bash
# Add research papers
cargo run -- document add papers/*.pdf

# Configure for academic content
cargo run -- config set chunking.strategy paragraph
cargo run -- config set retrieval.max-results 10

# Start research conversation
cargo run -- chat
```

**Sample queries**:
- "What are the main contributions of these papers?"
- "Compare the methodologies used across papers"
- "What are the common themes in the conclusions?"

### Example 2: Web Content Analysis

```bash
# Add news articles
cargo run -- document add --type web \
  https://example.com/news1 \
  https://example.com/news2

# Configure for web content
cargo run -- config set chunking.chunk-size 1500
cargo run -- config set conversation.include-sources true

# Query across articles
cargo run -- query "What are the key developments mentioned?"
```

### Example 3: Meeting Notes Processing

```bash
# Add meeting notes (text files)
cargo run -- document add --type text meetings/

# Configure for conversation analysis
cargo run -- config set conversation.max-context 15
cargo run -- config set retrieval.boost-recent true

# Analyze meeting content
cargo run -- chat --documents meeting-notes.txt
```

## Troubleshooting

### Common Issues

**"API key not found"**:
```bash
export OPENAI_API_KEY=your-key
# Or add to ~/.bashrc or ~/.zshrc
```

**"Document processing failed"**:
```bash
# Check file is accessible and not corrupted
cargo run -- document validate path/to/file.pdf

# Increase file size limit if needed
cargo run -- config set processing.max-file-size 100000000
```

**"Memory usage too high"**:
```bash
# Reduce memory limits
cargo run -- config set performance.max-memory 1024

# Enable streaming processing
cargo run -- config set processing.streaming true
```

**"Slow response times"**:
```bash
# Reduce chunk size for faster processing
cargo run -- config set chunking.chunk-size 1000

# Enable embedding caching
cargo run -- config set performance.cache-embeddings true
```

### Debug Mode

Enable detailed logging:
```bash
cargo run -- --verbose chat
cargo run -- log-level debug query "test question"
```

### Performance Monitoring

Check system performance:
```bash
cargo run -- stats
```

Monitor memory usage:
```bash
cargo run -- config show performance
```

## Configuration Reference

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `OPENAI_API_KEY` | OpenAI API key | Required |
| `RAG_CONFIG_PATH` | Configuration file path | `~/.rag_config.json` |
| `RAG_DATA_DIR` | Data storage directory | `~/.rag_data` |
| `RAG_LOG_LEVEL` | Logging level | `info` |
| `RAG_MAX_MEMORY` | Max memory usage (MB) | `1024` |

### Configuration File Structure

```json
{
  "document_processing": {
    "max_file_size": 52428800,
    "web_timeout": "30s",
    "max_concurrent_downloads": 5
  },
  "chunking": {
    "default_strategy": "semantic",
    "chunk_size": 2000,
    "overlap_size": 200,
    "respect_sentence_boundaries": true
  },
  "retrieval": {
    "max_results": 5,
    "similarity_threshold": 0.7,
    "cross_document_retrieval": true
  },
  "conversation": {
    "max_context_messages": 10,
    "max_session_duration": "1h",
    "include_sources": true,
    "temperature": 0.7
  },
  "performance": {
    "max_memory_usage": 1024,
    "embedding_batch_size": 100,
    "cache_embeddings": true,
    "parallel_processing": true
  }
}
```

## Next Steps

1. **Explore advanced features**: Try different chunking strategies and retrieval configurations
2. **Integrate with your workflow**: Use the CLI in scripts or automation
3. **Customize for your use case**: Create configuration profiles for different types of documents
4. **Monitor performance**: Use the stats command to optimize for your specific needs

For detailed API documentation and advanced customization options, see the full documentation.