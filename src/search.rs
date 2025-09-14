//! Simple search for MVP

use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::chunking::DocumentChunk;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub chunk_id: String,
    pub document_id: String,
    pub content: String,
    pub score: f32,
    pub rank: usize,
}

pub struct SearchEngine {
    keyword_weight: f32,
}

impl SearchEngine {
    pub fn new() -> Result<Self> {
        Ok(Self {
            keyword_weight: 0.7,
        })
    }

    pub fn search(&self, query: &str, chunks: &[DocumentChunk], limit: usize) -> Result<Vec<SearchResult>> {
        let mut results: Vec<SearchResult> = chunks
            .iter()
            .enumerate()
            .map(|(i, chunk)| {
                let score = self.calculate_similarity(query, &chunk.content);
                SearchResult {
                    chunk_id: chunk.id.clone(),
                    document_id: chunk.document_id.clone(),
                    content: chunk.content.clone(),
                    score,
                    rank: i,
                }
            })
            .collect();

        // Sort by score (descending) and take top results
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit);

        // Update ranks
        for (i, result) in results.iter_mut().enumerate() {
            result.rank = i + 1;
        }

        Ok(results)
    }

    fn calculate_similarity(&self, query: &str, content: &str) -> f32 {
        let query_lower = query.to_lowercase();
        let content_lower = content.to_lowercase();

        // Simple keyword matching score
        let query_words: Vec<&str> = query_lower.split_whitespace().collect();
        let content_words: Vec<&str> = content_lower.split_whitespace().collect();

        if query_words.is_empty() || content_words.is_empty() {
            return 0.0;
        }

        let mut matches = 0;
        for query_word in &query_words {
            for content_word in &content_words {
                if content_word.contains(query_word) || query_word.contains(content_word) {
                    matches += 1;
                    break;
                }
            }
        }

        let keyword_score = matches as f32 / query_words.len() as f32;

        // Simple length penalty (prefer chunks of reasonable length)
        let length_penalty = if content_words.len() < 10 {
            content_words.len() as f32 / 10.0
        } else if content_words.len() > 200 {
            200.0 / content_words.len() as f32
        } else {
            1.0
        };

        keyword_score * length_penalty
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunking::DocumentChunk;

    #[test]
    fn test_search_engine_creation() {
        let engine = SearchEngine::new();
        assert!(engine.is_ok());
    }

    #[test]
    fn test_simple_search() {
        let engine = SearchEngine::new().unwrap();
        let chunks = vec![
            DocumentChunk {
                id: "chunk1".to_string(),
                content: "Machine learning is a subset of artificial intelligence".to_string(),
                start_pos: 0,
                end_pos: 10,
                word_count: 10,
                document_id: "doc1".to_string(),
            },
            DocumentChunk {
                id: "chunk2".to_string(),
                content: "Natural language processing deals with text data".to_string(),
                start_pos: 0,
                end_pos: 8,
                word_count: 8,
                document_id: "doc2".to_string(),
            },
        ];

        let results = engine.search("machine learning", &chunks, 5).unwrap();
        assert_eq!(results.len(), 2);
        assert!(results[0].score > results[1].score); // First result should be more relevant
    }
}