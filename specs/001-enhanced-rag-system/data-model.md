# Data Model: Full-Featured RAG System with Comprehensive File Type Support

## Core Entities

### Document
Represents any source document that can be processed by the system.

```rust
struct Document {
    id: String,                    // Unique identifier
    source_type: DocumentType,      // PDF, Text, Web
    source_path: String,           // File path or URL
    title: Option<String>,         // Extracted or user-provided title
    metadata: DocumentMetadata,     // Type-specific metadata
    processing_status: ProcessingStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    file_size: u64,                // Bytes
    chunk_count: usize,
}

enum DocumentType {
    // Document formats
    Pdf,
    Text,
    Markdown,
    Rtf,
    Html,
    Xml,

    // Data formats
    Json,
    Csv,
    Yaml,
    Toml,

    // Source code
    SourceCode(SourceCodeLanguage),

    // Configuration files
    ConfigFile(ConfigFormat),

    // Other formats
    LogFile,
    EBook(EBookFormat),
    AudioTranscript(AudioTranscriptFormat),
    Web,
}

enum SourceCodeLanguage {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Java,
    Cpp,
    C,
    Go,
    Ruby,
    PHP,
    Swift,
    Kotlin,
    Shell,
    Sql,
    // Add more as needed
}

enum ConfigFormat {
    Ini,
    Properties,
    Conf,
    // Add more as needed
}

enum EBookFormat {
    Epub,
    Mobi,
    // Add more as needed
}

enum AudioTranscriptFormat {
    Srt,
    Vtt,
    Plain,
    // Add more as needed
}

struct DocumentMetadata {
    // Document-specific
    page_count: Option<u32>,
    author: Option<String>,
    title: Option<String>,

    // Web-specific
    domain: Option<String>,
    crawl_date: Option<DateTime<Utc>>,

    // Source code-specific
    language: Option<SourceCodeLanguage>,
    function_count: Option<u32>,
    class_count: Option<u32>,
    line_count: Option<u32>,

    // Data file-specific
    record_count: Option<u32>,
    field_count: Option<u32>,
    data_structure: Option<DataStructure>,

    // EBook-specific
    chapter_count: Option<u32>,
    isbn: Option<String>,
    author_list: Option<Vec<String>>,

    // Common
    language: Option<String>,
    encoding: Option<String>,
    file_hash: Option<String>, // For change detection
    extracted_metadata: Option<serde_json::Value>,
}

enum DataStructure {
    Tabular,      // CSV, structured data
    Hierarchical, // JSON, XML, YAML
    Keyed,        // TOML, INI
    Unstructured, // Plain text
}

enum ProcessingStatus {
    Pending,
    Processing,
    Completed,
    Failed(String),
    Indexed,
}
```

### DocumentChunk
A logical segment of a document optimized for embedding and retrieval.

```rust
struct DocumentChunk {
    id: String,                     // Unique identifier
    document_id: String,            // Parent document
    content: String,                // Text content
    chunk_type: ChunkType,          // How this chunk was created
    position: usize,                // Position in document
    word_count: usize,
    embedding: Option<Vec<f32>>,    // Generated embedding
    metadata: ChunkMetadata,
    created_at: DateTime<Utc>,
}

enum ChunkType {
    FixedSize,      // Fixed character count
    Paragraph,      // Natural paragraph boundaries
    Semantic,       // Semantic boundaries
    Heading,        // Document section boundaries
}

struct ChunkMetadata {
    page_number: Option<u32>,       // PDF pages
    section: Option<String>,        // Document section
    heading_level: Option<u8>,      // H1, H2, etc.
    url_fragment: Option<String>,  // Web anchor
    relevance_score: Option<f32>,   // Query relevance
}
```

### ConversationSession
Tracks user interaction history and context.

```rust
struct ConversationSession {
    id: String,                     // Session identifier
    user_id: Option<String>,        // User identifier (future)
    created_at: DateTime<Utc>,
    last_activity: DateTime<Utc>,
    messages: Vec<ConversationMessage>,
    context_window: usize,          // Max messages to keep in context
    document_filter: Option<Vec<String>>, // Document IDs to search
    metadata: SessionMetadata,
}

struct ConversationMessage {
    id: String,
    role: MessageRole,              // User, Assistant, System
    content: String,
    timestamp: DateTime<Utc>,
    references: Vec<ChunkReference>, // Source chunks used
}

enum MessageRole {
    User,
    Assistant,
    System,
}

struct ChunkReference {
    chunk_id: String,
    document_id: String,
    relevance_score: f32,
    excerpt: String,               // Relevant text excerpt
}

struct SessionMetadata {
    total_messages: usize,
    total_documents_queried: usize,
    average_response_time: f64,     // Seconds
    session_duration: Duration,
}
```

### ConfigurationProfile
User-defined settings for system behavior.

```rust
struct ConfigurationProfile {
    id: String,
    name: String,
    created_at: DateTime<Utc>,
    is_default: bool,

    // Processing settings
    document_processing: DocumentProcessingConfig,
    chunking: ChunkingConfig,
    retrieval: RetrievalConfig,
    conversation: ConversationConfig,

    // API settings
    openai: OpenAIConfig,

    // System settings
    logging: LoggingConfig,
    performance: PerformanceConfig,
}

struct DocumentProcessingConfig {
    max_file_size: u64,             // Bytes
    supported_formats: Vec<DocumentType>,
    web_timeout: Duration,
    max_concurrent_ownloads: usize,
    user_agent: String,
}

struct ChunkingConfig {
    default_strategy: ChunkType,
    chunk_size: usize,              // Characters
    overlap_size: usize,            // Characters
    respect_sentence_boundaries: bool,
    respect_paragraph_boundaries: bool,
    max_chunks_per_document: usize,
}

struct RetrievalConfig {
    max_results: usize,
    similarity_threshold: f32,
    boost_recent_documents: bool,
    cross_document_retrieval: bool,
}

struct ConversationConfig {
    max_context_messages: usize,
    max_session_duration: Duration,
    auto_save: bool,
    include_sources: bool,
    temperature: f32,              // LLM temperature
}

struct OpenAIConfig {
    api_key: String,                // From environment
    model: String,                  // gpt-4, gpt-3.5-turbo
    embedding_model: String,        // text-embedding-ada-002
    max_tokens: usize,
    request_timeout: Duration,
}

struct LoggingConfig {
    level: LogLevel,
    format: LogFormat,
    file_path: Option<String>,
    max_file_size: u64,
}

struct PerformanceConfig {
    max_memory_usage: u64,          // MB
    embedding_batch_size: usize,
    cache_embeddings: bool,
    parallel_processing: bool,
}
```

### SearchResult
Represents a search result with source attribution.

```rust
struct SearchResult {
    query: String,
    chunks: Vec<ScoredChunk>,
    total_found: usize,
    search_time: Duration,
    document_sources: HashSet<String>, // Unique document IDs
}

struct ScoredChunk {
    chunk: DocumentChunk,
    score: f32,                      // Similarity score
    rank: usize,
    highlights: Vec<String>,        // Matching text segments
}
```

### FileTypeProcessor
Specialized handler for processing specific file formats.

```rust
struct FileTypeProcessor {
    id: String,
    name: String,
    supported_types: Vec<DocumentType>,
    capabilities: ProcessorCapabilities,
    performance_profile: PerformanceProfile,
    config: ProcessorConfig,
    is_enabled: bool,
}

struct ProcessorCapabilities {
    streaming_support: bool,
    metadata_extraction: bool,
    structure_preservation: bool,
    language_detection: bool,
    batch_processing: bool,
    error_recovery: bool,
}

struct PerformanceProfile {
    avg_processing_time_ms: u32,
    memory_multiplier: f32, // Memory usage relative to file size
    max_concurrent: usize,
    timeout_seconds: u32,
}

struct ProcessorConfig {
    chunking_strategy: ChunkType,
    extraction_options: serde_json::Value,
    custom_parameters: serde_json::Value,
}
```

### ContentExtractor
Component for extracting text content from different file types.

```rust
trait ContentExtractor {
    fn can_handle(&self, file_type: &DocumentType) -> bool;
    fn extract_content(&self, path: &Path, config: &ProcessorConfig) -> Result<ExtractedContent>;
    fn extract_metadata(&self, path: &Path) -> Result<DocumentMetadata>;
    fn validate_file(&self, path: &Path) -> Result<()>;
}

struct ExtractedContent {
    text: String,
    structure: Option<ContentStructure>,
    language: Option<String>,
    encoding: Option<String>,
    extraction_time: Duration,
    warnings: Vec<String>,
}

enum ContentStructure {
    Tabular { headers: Vec<String>, rows: Vec<Vec<String>> },
    Hierarchical { nodes: Vec<StructureNode> },
    Code { language: SourceCodeLanguage, functions: Vec<CodeFunction> },
    Document { sections: Vec<DocumentSection> },
    Unstructured,
}

struct StructureNode {
    path: String,
    value: serde_json::Value,
    node_type: String,
    children: Vec<StructureNode>,
}

struct CodeFunction {
    name: String,
    signature: String,
    documentation: Option<String>,
    line_start: usize,
    line_end: usize,
}

struct DocumentSection {
    title: Option<String>,
    level: Option<u8>,
    content: String,
    page_number: Option<u32>,
}
```

### ProcessingBatch
Group of documents processed together with progress tracking.

```rust
struct ProcessingBatch {
    id: String,
    name: String,
    documents: Vec<BatchDocument>,
    status: BatchStatus,
    progress: BatchProgress,
    created_at: DateTime<Utc>,
    started_at: Option<DateTime<Utc>>,
    completed_at: Option<DateTime<Utc>>,
    config: BatchConfig,
    errors: Vec<ProcessingError>,
}

struct BatchDocument {
    document_id: String,
    file_path: String,
    status: DocumentProcessingStatus,
    progress: f32, // 0.0 to 1.0
    error: Option<ProcessingError>,
    processed_at: Option<DateTime<Utc>>,
}

enum BatchStatus {
    Pending,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

struct BatchProgress {
    total_documents: usize,
    completed_documents: usize,
    failed_documents: usize,
    processing_documents: usize,
    bytes_processed: u64,
    total_bytes: u64,
    estimated_time_remaining: Option<Duration>,
}

struct BatchConfig {
    max_concurrent_documents: usize,
    chunk_size: usize,
    enable_caching: bool,
    resume_on_failure: bool,
    error_handling: ErrorHandlingStrategy,
}

enum ErrorHandlingStrategy {
    ContinueOnError,
    StopOnError,
    RetryFailed,
}

struct ProcessingError {
    document_id: String,
    error_type: String,
    message: String,
    occurred_at: DateTime<Utc>,
    stack_trace: Option<String>,
    retry_count: u32,
}

enum DocumentProcessingStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Skipped,
}
```

## Relationships and Constraints

### Document Processing Flow
1. Document → DocumentChunks (1:N)
2. DocumentChunks → Embeddings (1:1, optional)
3. DocumentChunks → Vector Store (automatic)

### Conversation Flow
1. ConversationSession → ConversationMessages (1:N)
2. ConversationMessage → ChunkReferences (1:N)
3. ChunkReference → DocumentChunk (N:1)

### Configuration Hierarchy
1. ConfigurationProfile → Default System Config (1:1)
2. Document processing uses active ConfigurationProfile

### Validation Rules

#### Document Validation
- File size ≤ max_file_size (default: 100MB)
- Supported file formats only
- URL must be accessible and return valid content
- File must be readable and contain text content

#### Chunk Validation
- Content must not be empty
- Word count must be > 0
- Chunk size must be within configured bounds
- Embedding vector must have correct dimensions

#### Conversation Validation
- Session duration ≤ max_session_duration
- Message count ≤ max_context_messages
- Content length must be reasonable (≤ 10,000 chars)
- References must point to valid chunks

#### Configuration Validation
- API keys must be valid format
- Numeric values must be within reasonable bounds
- File paths must be writable
- Timeouts must be > 0

## State Transitions

### Document Processing States
```
Pending → Processing → Completed → Indexed
                ↓
               Failed → Retry → Processing
```

### Conversation Session States
```
Active → Idle → Expired
  ↓
Closed
```

### Configuration Profile States
```
Draft → Active → Archived
```

## Data Storage and Persistence

### In-Memory Storage
- Document chunks and embeddings (current session)
- Active conversation sessions
- Configuration cache

### Persistent Storage
- Configuration profiles (JSON files)
- Document metadata (JSON files)
- Conversation history (optional, JSON files)
- Embedding cache (optional, binary files)

### Temporary Storage
- Downloaded web content
- Processing artifacts
- Temporary chunks during processing

## Security and Privacy

### Data Protection
- API keys stored in environment variables only
- User conversations not persisted by default
- Temporary files cleaned up automatically
- No telemetry or analytics collection

### Access Control
- Configuration files respect system file permissions
- No network access except for approved APIs
- Web requests include user-agent identification
- Rate limiting for external API calls

## Performance Considerations

### Memory Management
- Streaming document processing to avoid loading entire files
- Lazy loading of document chunks
- Configurable memory limits
- Automatic cleanup of unused resources

### Caching Strategy
- Embedding cache to avoid recomputation
- Document metadata cache
- Configuration profile cache
- Web content cache with TTL

### Concurrency
- Parallel document processing
- Async web requests
- Non-blocking I/O operations
- Configurable concurrency limits