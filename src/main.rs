//! Simple CLI for the RAG System

use clap::{Parser, Subcommand};
use std::path::Path;

use rag_system::SimpleRagSystem;

#[derive(Parser)]
#[command(name = "rag-system")]
#[command(about = "Simple RAG System")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Process a document
    Process {
        /// Path to the document file
        file: String,
    },
    /// Search for documents
    Search {
        /// Search query
        query: String,
        /// Maximum number of results
        #[arg(short, long, default_value = "5")]
        limit: usize,
    },
    /// Evaluate search quality
    Evaluate {
        /// Search query
        query: String,
        /// Expected document IDs (comma-separated)
        expected: String,
    },
    /// List all processed documents
    List,
    /// Show storage statistics
    Stats,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut rag = SimpleRagSystem::new()?;

    match cli.command {
        Commands::Process { file } => {
            println!("Processing document: {}", file);
            let path = Path::new(&file);

            if !path.exists() {
                eprintln!("Error: File '{}' does not exist", file);
                return Ok(());
            }

            match rag.process_document(path) {
                Ok(doc_id) => {
                    println!("✓ Document processed successfully");
                    println!("  Document ID: {}", doc_id);
                }
                Err(e) => {
                    eprintln!("Error processing document: {}", e);
                }
            }
        }
        Commands::Search { query, limit } => {
            println!("Searching for: {}", query);
            match rag.search(&query, limit) {
                Ok(results) => {
                    println!("Found {} results:", results.len());
                    for (i, result) in results.iter().enumerate() {
                        println!("  {}. [Score: {:.3}] {}", i + 1, result.score, result.content);
                    }
                }
                Err(e) => {
                    eprintln!("Error searching: {}", e);
                }
            }
        }
        Commands::Evaluate { query, expected } => {
            let expected_docs: Vec<String> = expected.split(',').map(|s| s.trim().to_string()).collect();
            println!("Evaluating search for: {}", query);
            println!("Expected documents: {:?}", expected_docs);

            match rag.evaluate_search(&query, &expected_docs) {
                Ok(metrics) => {
                    println!("Evaluation Results:");
                    println!("  Relevance: {:.3}", metrics.relevance);
                    println!("  Precision: {:.3}", metrics.precision);
                    println!("  Recall: {:.3}", metrics.recall);
                    println!("  F1 Score: {:.3}", metrics.f1_score);
                }
                Err(e) => {
                    eprintln!("Error evaluating: {}", e);
                }
            }
        }
        Commands::List => {
            let docs = rag.list_documents()?;
            println!("Processed Documents ({}):", docs.len());
            for doc_id in docs {
                if let Some(doc) = rag.get_document(&doc_id)? {
                    println!("  - {} ({}, {} words)", doc_id, doc.metadata.file_type, doc.metadata.word_count);
                }
            }
        }
        Commands::Stats => {
            let stats = rag.get_stats()?;
            println!("Storage Statistics:");
            println!("  Total Documents: {}", stats.total_documents);
            println!("  Total Chunks: {}", stats.total_chunks);
            println!("  Total Size: {} bytes", stats.total_size_bytes);
        }
    }

    Ok(())
}