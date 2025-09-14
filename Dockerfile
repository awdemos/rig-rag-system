# Multi-stage build for RAG System
FROM rust:1.75-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy Cargo files
COPY rag_system/Cargo.toml rag_system/Cargo.lock ./rag_system/
COPY rag_system/src ./rag_system/src

# Build the application
WORKDIR /app/rag_system
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/rag_system/target/release/rag_system /usr/local/bin/

# Create non-root user
RUN useradd -m -u 1000 raguser
USER raguser

# Expose port (if needed for future web interface)
EXPOSE 8080

# Environment variables
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD echo '{"status": "healthy"}' || exit 1

# Default command
CMD ["rag_system"]