//! Simple document processor for MVP

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub file_path: String,
    pub file_type: String,
    pub file_size: usize,
    pub word_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedDocument {
    pub id: String,
    pub content: String,
    pub metadata: DocumentMetadata,
}

pub struct DocumentProcessor;

impl DocumentProcessor {
    pub fn new() -> Self {
        Self
    }

    pub fn process_file(&self, file_path: &Path) -> Result<ProcessedDocument> {
        let content = fs::read_to_string(file_path)?;
        let metadata = file_path.metadata()?;

        let word_count = content.split_whitespace().count();
        let file_type = file_path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("txt")
            .to_string();

        let doc = ProcessedDocument {
            id: uuid::Uuid::new_v4().to_string(),
            content,
            metadata: DocumentMetadata {
                file_path: file_path.to_string_lossy().to_string(),
                file_type,
                file_size: metadata.len() as usize,
                word_count,
            },
        };

        Ok(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_processor_creation() {
        let processor = DocumentProcessor::new();
        assert!(true); // Just test that it doesn't panic
    }

    #[test]
    fn test_file_processing() {
        let processor = DocumentProcessor::new();
        let test_file = "/tmp/test_processor.txt";
        let test_content = "This is a test file for processing.";

        fs::write(test_file, test_content).unwrap();

        let document = processor.process_file(Path::new(test_file)).unwrap();
        assert_eq!(document.content, test_content);
        assert_eq!(document.metadata.word_count, 7);
        assert_eq!(document.metadata.file_type, "txt");

        fs::remove_file(test_file).unwrap();
    }
}