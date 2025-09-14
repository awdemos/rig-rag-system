//! Minimal Working RAG System MVP

use std::path::Path;
use std::collections::HashMap;

pub mod chunking;
pub mod processor;
pub mod search;
pub mod storage;
pub mod evaluation;

pub use chunking::*;
pub use processor::*;
pub use search::*;
pub use storage::*;
pub use evaluation::*;

/// Simple RAG system that ties everything together
pub struct SimpleRagSystem {
    chunker: ChunkingEngine,
    searcher: SearchEngine,
    storage: StorageManager,
}

impl SimpleRagSystem {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            chunker: ChunkingEngine::new()?,
            searcher: SearchEngine::new()?,
            storage: StorageManager::new()?,
        })
    }

    pub fn process_document(&mut self, file_path: &Path) -> anyhow::Result<String> {
        // Process the document
        let processor = DocumentProcessor::new();
        let document = processor.process_file(file_path)?;

        // Chunk the document
        let chunks = self.chunker.chunk_document(&document)?;

        // Store the document and chunks
        let doc_id = self.storage.store_document(document)?;
        self.storage.store_chunks(doc_id.clone(), chunks)?;

        Ok(doc_id)
    }

    pub fn search(&self, query: &str, limit: usize) -> anyhow::Result<Vec<SearchResult>> {
        let all_chunks = self.storage.get_all_chunks()?;
        let results = self.searcher.search(query, &all_chunks, limit)?;
        Ok(results)
    }

    pub fn evaluate_search(&self, query: &str, expected_doc_ids: &[String]) -> anyhow::Result<EvaluationMetrics> {
        let results = self.search(query, 5)?;
        let evaluator = Evaluator::new();
        evaluator.evaluate(&results, expected_doc_ids)
    }

    pub fn list_documents(&self) -> anyhow::Result<Vec<String>> {
        self.storage.list_documents()
    }

    pub fn get_document(&self, doc_id: &str) -> anyhow::Result<Option<ProcessedDocument>> {
        self.storage.get_document(doc_id)
    }

    pub fn get_stats(&self) -> anyhow::Result<StorageStats> {
        self.storage.get_stats()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_simple_rag_workflow() {
        // Create test content
        let test_content = r#"
# Machine Learning Basics

Machine learning is a subset of artificial intelligence that enables computers to learn from data without being explicitly programmed.

## Key Concepts

### Supervised Learning
In supervised learning, we train models on labeled data.

### Unsupervised Learning
Unsupervised learning finds patterns in unlabeled data.

Applications include natural language processing, computer vision, and recommendation systems.
"#;

        // Create temporary file
        let test_file = "/tmp/test_rag.md";
        fs::write(test_file, test_content).unwrap();

        // Test the workflow
        let mut rag = SimpleRagSystem::new().unwrap();
        let doc_id = rag.process_document(Path::new(test_file)).unwrap();

        // Test search
        let results = rag.search("machine learning", 3).unwrap();
        assert!(!results.is_empty());

        // Test evaluation
        let metrics = rag.evaluate_search("machine learning", &[doc_id]).unwrap();
        assert!(metrics.relevance >= 0.0);
        assert!(metrics.precision >= 0.0);

        // Clean up
        fs::remove_file(test_file).unwrap();
    }
}