//! Simple in-memory storage for MVP

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::chunking::DocumentChunk;
use crate::processor::ProcessedDocument;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStats {
    pub total_documents: usize,
    pub total_chunks: usize,
    pub total_size_bytes: usize,
}

pub struct StorageManager {
    documents: Arc<Mutex<HashMap<String, ProcessedDocument>>>,
    chunks: Arc<Mutex<HashMap<String, DocumentChunk>>>,
}

impl StorageManager {
    pub fn new() -> Result<Self> {
        Ok(Self {
            documents: Arc::new(Mutex::new(HashMap::new())),
            chunks: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub fn store_document(&mut self, document: ProcessedDocument) -> Result<String> {
        let doc_id = document.id.clone();
        let mut docs = self.documents.lock().unwrap();
        docs.insert(doc_id.clone(), document);
        Ok(doc_id)
    }

    pub fn store_chunks(&mut self, _doc_id: String, chunks: Vec<DocumentChunk>) -> Result<()> {
        let mut chunk_map = self.chunks.lock().unwrap();
        for chunk in chunks {
            chunk_map.insert(chunk.id.clone(), chunk);
        }
        Ok(())
    }

    pub fn get_document(&self, doc_id: &str) -> Result<Option<ProcessedDocument>> {
        let docs = self.documents.lock().unwrap();
        Ok(docs.get(doc_id).cloned())
    }

    pub fn get_all_chunks(&self) -> Result<Vec<DocumentChunk>> {
        let chunks = self.chunks.lock().unwrap();
        Ok(chunks.values().cloned().collect())
    }

    pub fn list_documents(&self) -> Result<Vec<String>> {
        let docs = self.documents.lock().unwrap();
        Ok(docs.keys().cloned().collect())
    }

    pub fn get_stats(&self) -> Result<StorageStats> {
        let docs = self.documents.lock().unwrap();
        let chunks = self.chunks.lock().unwrap();

        let total_size_bytes = docs.values()
            .map(|doc| doc.content.len())
            .sum::<usize>();

        Ok(StorageStats {
            total_documents: docs.len(),
            total_chunks: chunks.len(),
            total_size_bytes,
        })
    }

    pub fn clear(&mut self) -> Result<()> {
        let mut docs = self.documents.lock().unwrap();
        let mut chunks = self.chunks.lock().unwrap();
        docs.clear();
        chunks.clear();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::processor::DocumentProcessor;

    #[test]
    fn test_storage_creation() {
        let storage = StorageManager::new();
        assert!(storage.is_ok());
    }

    #[test]
    fn test_document_storage() {
        let mut storage = StorageManager::new().unwrap();

        let document = ProcessedDocument {
            id: "test_doc".to_string(),
            content: "Test content".to_string(),
            metadata: crate::processor::DocumentMetadata {
                file_path: "/test/path".to_string(),
                file_type: "txt".to_string(),
                file_size: 12,
                word_count: 2,
            },
        };

        let doc_id = storage.store_document(document).unwrap();
        assert_eq!(doc_id, "test_doc");

        let retrieved = storage.get_document("test_doc").unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().content, "Test content");
    }
}