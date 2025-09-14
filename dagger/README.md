# Dagger CI/CD for RAG System

This directory contains Dagger CI/CD pipeline configuration for the RAG System.

## Prerequisites

1. Install Dagger:
   ```bash
   curl -sSfL https://dl.dagger.io/dagger/install.sh | sh
   ```

2. Install Python SDK (optional, for local development):
   ```bash
   pip install "dagger-io>=0.12.0"
   ```

## Usage

### Run full CI pipeline:
```bash
dagger call ci --src=.
```

### Run individual checks:
```bash
# Check compilation
dagger call check --src=.

# Run tests
dagger call test --src=.

# Check formatting
dagger call fmt --src=.

# Run clippy
dagger call clippy --src=.

# Security audit
dagger call audit --src=.

# Generate docs
dagger call docs --src=.
```

### Development mode:
```bash
# Start development environment with hot-reloading
dagger call dev --src=. --up

# Or run once
dagger call dev --src=.
```

### Build artifacts:
```bash
# Build release binary
dagger call build --src=. --up

# Build Docker image
dagger call docker-build --src=.
```

## Available Functions

- `ci()` - Run full CI pipeline
- `check()` - Check compilation
- `test()` - Run tests
- `fmt()` - Check code formatting
- `clippy()` - Run Clippy linter
- `audit()` - Security audit
- `docs()` - Generate documentation
- `build()` - Build release binary
- `docker_build()` - Build Docker image
- `dev()` - Development environment
- `full_ci()` - Run all CI checks

## Local Development

For local development with the Python SDK:

```python
import asyncio
import dagger
from dagger import function, object_type

async def main():
    async with dagger.Connection() as client:
        # Run checks
        src = client.host().directory(".")

        # Check compilation
        check = await (
            client.container()
            .from_("rust:1.75-slim")
            .with_workdir("/app")
            .with_directory("/app", src)
            .with_exec(["cargo", "check"])
            .stdout()
        )
        print("Check output:", check)

if __name__ == "__main__":
    asyncio.run(main())
```