//! Test the RAG system functionality in one session

use rag_system::SimpleRagSystem;
use std::fs;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    println!("Testing RAG System...");

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

## Deep Learning

Deep learning uses neural networks with multiple layers to learn hierarchical representations of data.
"#;

    // Create temporary file
    let test_file = "/tmp/test_rag.txt";
    fs::write(test_file, test_content)?;

    println!("✓ Created test file: {}", test_file);

    // Create RAG system
    let mut rag = SimpleRagSystem::new()?;
    println!("✓ Created RAG system");

    // Process document
    let doc_id = rag.process_document(Path::new(test_file))?;
    println!("✓ Processed document with ID: {}", doc_id);

    // Test search
    println!("\nTesting search functionality:");
    let results = rag.search("machine learning", 3)?;
    println!("Found {} results for 'machine learning':", results.len());
    for (i, result) in results.iter().enumerate() {
        println!("  {}. [Score: {:.3}] {}", i + 1, result.score, result.content);
    }

    // Test another search
    println!("\nTesting search for 'neural networks':");
    let results = rag.search("neural networks", 3)?;
    println!("Found {} results for 'neural networks':", results.len());
    for (i, result) in results.iter().enumerate() {
        println!("  {}. [Score: {:.3}] {}", i + 1, result.score, result.content);
    }

    // Test evaluation
    println!("\nTesting evaluation:");
    let metrics = rag.evaluate_search("machine learning", &[doc_id])?;
    println!("Evaluation metrics:");
    println!("  Relevance: {:.3}", metrics.relevance);
    println!("  Precision: {:.3}", metrics.precision);
    println!("  Recall: {:.3}", metrics.recall);
    println!("  F1 Score: {:.3}", metrics.f1_score);

    // Test stats
    let stats = rag.get_stats()?;
    println!("\nStorage statistics:");
    println!("  Total Documents: {}", stats.total_documents);
    println!("  Total Chunks: {}", stats.total_chunks);
    println!("  Total Size: {} bytes", stats.total_size_bytes);

    // Clean up
    fs::remove_file(test_file)?;
    println!("\n✓ Test completed successfully!");

    Ok(())
}