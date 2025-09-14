//! Simple evaluation for MVP

use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::search::SearchResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationMetrics {
    pub relevance: f32,
    pub precision: f32,
    pub recall: f32,
    pub f1_score: f32,
}

pub struct Evaluator;

impl Evaluator {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&self, results: &[SearchResult], expected_doc_ids: &[String]) -> Result<EvaluationMetrics> {
        if results.is_empty() {
            return Ok(EvaluationMetrics {
                relevance: 0.0,
                precision: 0.0,
                recall: 0.0,
                f1_score: 0.0,
            });
        }

        // Calculate relevance based on scores
        let relevance = self.calculate_relevance(results);

        // Calculate precision and recall
        let (precision, recall) = self.calculate_precision_recall(results, expected_doc_ids);

        // Calculate F1 score
        let f1_score = if precision + recall > 0.0 {
            2.0 * (precision * recall) / (precision + recall)
        } else {
            0.0
        };

        Ok(EvaluationMetrics {
            relevance,
            precision,
            recall,
            f1_score,
        })
    }

    fn calculate_relevance(&self, results: &[SearchResult]) -> f32 {
        if results.is_empty() {
            return 0.0;
        }

        // Simple relevance based on search scores
        let total_score: f32 = results.iter().map(|r| r.score).sum();
        let max_possible_score = results.len() as f32;

        if max_possible_score > 0.0 {
            (total_score / max_possible_score).min(1.0)
        } else {
            0.0
        }
    }

    fn calculate_precision_recall(&self, results: &[SearchResult], expected_doc_ids: &[String]) -> (f32, f32) {
        if expected_doc_ids.is_empty() {
            return (1.0, 1.0);
        }

        let mut relevant_retrieved = 0;
        let expected_set: std::collections::HashSet<_> = expected_doc_ids.iter().collect();

        for result in results {
            if expected_set.contains(&result.document_id) {
                relevant_retrieved += 1;
            }
        }

        let precision = if !results.is_empty() {
            relevant_retrieved as f32 / results.len() as f32
        } else {
            0.0
        };

        let recall = relevant_retrieved as f32 / expected_doc_ids.len() as f32;

        (precision, recall)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::search::SearchResult;

    #[test]
    fn test_evaluator_creation() {
        let evaluator = Evaluator::new();
        assert!(true);
    }

    #[test]
    fn test_empty_results() {
        let evaluator = Evaluator::new();
        let results = vec![];
        let expected = vec!["doc1".to_string()];

        let metrics = evaluator.evaluate(&results, &expected).unwrap();

        assert_eq!(metrics.relevance, 0.0);
        assert_eq!(metrics.precision, 0.0);
        assert_eq!(metrics.recall, 0.0);
        assert_eq!(metrics.f1_score, 0.0);
    }

    #[test]
    fn test_perfect_evaluation() {
        let evaluator = Evaluator::new();

        let results = vec![SearchResult {
            chunk_id: "chunk1".to_string(),
            document_id: "doc1".to_string(),
            content: "Perfect match".to_string(),
            score: 1.0,
            rank: 1,
        }];

        let expected = vec!["doc1".to_string()];
        let metrics = evaluator.evaluate(&results, &expected).unwrap();

        assert!(metrics.relevance > 0.5);
        assert_eq!(metrics.precision, 1.0);
        assert_eq!(metrics.recall, 1.0);
        assert_eq!(metrics.f1_score, 1.0);
    }
}