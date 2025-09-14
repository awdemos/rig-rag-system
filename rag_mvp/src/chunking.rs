//! Simple chunking for MVP

use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::processor::ProcessedDocument;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentChunk {
    pub id: String,
    pub content: String,
    pub start_pos: usize,
    pub end_pos: usize,
    pub word_count: usize,
    pub document_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChunkingStrategy {
    FixedSize { size: usize },
    Paragraph,
}

pub struct ChunkingEngine {
    strategy: ChunkingStrategy,
}

impl ChunkingEngine {
    pub fn new() -> Result<Self> {
        Ok(Self {
            strategy: ChunkingStrategy::FixedSize { size: 500 },
        })
    }

    pub fn chunk_document(&self, document: &ProcessedDocument) -> Result<Vec<DocumentChunk>> {
        match &self.strategy {
            ChunkingStrategy::FixedSize { size } => self.fixed_size_chunking(document, *size),
            ChunkingStrategy::Paragraph => self.paragraph_chunking(document),
        }
    }

    fn fixed_size_chunking(&self, document: &ProcessedDocument, chunk_size: usize) -> Result<Vec<DocumentChunk>> {
        let words: Vec<&str> = document.content.split_whitespace().collect();
        let mut chunks = Vec::new();
        let mut start = 0;

        while start < words.len() {
            let end = (start + chunk_size).min(words.len());
            let chunk_words = &words[start..end];
            let chunk_content = chunk_words.join(" ");

            let chunk = DocumentChunk {
                id: format!("{}_{}", document.id, chunks.len()),
                content: chunk_content,
                start_pos: start,
                end_pos: end,
                word_count: chunk_words.len(),
                document_id: document.id.clone(),
            };

            chunks.push(chunk);
            start = end;
        }

        Ok(chunks)
    }

    fn paragraph_chunking(&self, document: &ProcessedDocument) -> Result<Vec<DocumentChunk>> {
        let paragraphs: Vec<&str> = document.content.split("\n\n").filter(|p| !p.trim().is_empty()).collect();
        let mut chunks = Vec::new();
        let mut word_pos = 0;

        for (i, paragraph) in paragraphs.iter().enumerate() {
            let word_count = paragraph.split_whitespace().count();

            let chunk = DocumentChunk {
                id: format!("{}_{}", document.id, i),
                content: paragraph.to_string(),
                start_pos: word_pos,
                end_pos: word_pos + word_count,
                word_count,
                document_id: document.id.clone(),
            };

            chunks.push(chunk);
            word_pos += word_count;
        }

        Ok(chunks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunking_engine_creation() {
        let engine = ChunkingEngine::new();
        assert!(engine.is_ok());
    }

    #[test]
    fn test_fixed_size_chunking() {
        let engine = ChunkingEngine::new().unwrap();
        let document = ProcessedDocument {
            id: "test".to_string(),
            content: "This is a test document with multiple words that should be chunked properly.".to_string(),
            metadata: crate::processor::DocumentMetadata {
                file_path: "/test.txt".to_string(),
                file_type: "txt".to_string(),
                file_size: 100,
                word_count: 15,
            },
        };

        let chunks = engine.chunk_document(&document).unwrap();
        assert!(!chunks.is_empty());
        assert!(chunks[0].word_count <= 500);
    }
}