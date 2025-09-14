"""Dagger CI/CD pipeline for RAG System"""

import dagger
import asyncio
from dagger import function, object_type


@object_type
class RagsystemCi:
    """CI/CD pipeline for the RAG System project"""

    @function
    async def build(self, src: dagger.Directory) -> dagger.Container:
        """Build the RAG system"""
        return (
            dag.container()
            .from_("rust:1.75-slim")
            .with_workdir("/app")
            .with_directory("/app", src)
            .with_exec(["cargo", "build", "--release"])
        )

    @function
    async def check(self, src: dagger.Directory) -> dagger.Container:
        """Check compilation without building"""
        return (
            dag.container()
            .from_("rust:1.75-slim")
            .with_workdir("/app")
            .with_directory("/app", src)
            .with_exec(["cargo", "check"])
        )

    @function
    async def test(self, src: dagger.Directory) -> dagger.Container:
        """Run tests"""
        return (
            dag.container()
            .from_("rust:1.75-slim")
            .with_workdir("/app")
            .with_directory("/app", src)
            .with_env_variable("RUST_BACKTRACE", "1")
            .with_exec(["cargo", "test", "--lib"])
        )

    @function
    async def fmt(self, src: dagger.Directory) -> dagger.Container:
        """Check code formatting"""
        return (
            dag.container()
            .from_("rust:1.75-slim")
            .with_workdir("/app")
            .with_directory("/app", src)
            .with_exec(["cargo", "fmt", "--check"])
        )

    @function
    async def clippy(self, src: dagger.Directory) -> dagger.Container:
        """Run Clippy linter"""
        return (
            dag.container()
            .from_("rust:1.75-slim")
            .with_workdir("/app")
            .with_directory("/app", src)
            .with_exec(["cargo", "clippy", "--", "-D", "warnings"])
        )

    @function
    async def audit(self, src: dagger.Directory) -> dagger.Container:
        """Run security audit"""
        return (
            dag.container()
            .from_("rust:1.75-slim")
            .with_workdir("/app")
            .with_directory("/app", src)
            .with_exec(["sh", "-c", "cargo install cargo-audit && cargo audit"])
        )

    @function
    async def docs(self, src: dagger.Directory) -> dagger.Container:
        """Generate documentation"""
        return (
            dag.container()
            .from_("rust:1.75-slim")
            .with_workdir("/app")
            .with_directory("/app", src)
            .with_exec(["cargo", "doc", "--no-deps"])
        )

    @function
    async def full_ci(self, src: dagger.Directory) -> list[dagger.Container]:
        """Run full CI pipeline"""
        results = await asyncio.gather(
            self.check(src),
            self.fmt(src),
            self.clippy(src),
            self.test(src),
            self.audit(src),
        )
        return results

    @function
    async def docker_build(self, src: dagger.Directory) -> dagger.Container:
        """Build Docker image"""
        return (
            dag.container()
            .build(context=src)
        )

    @function
    async def development(self, src: dagger.Directory) -> dagger.Container:
        """Create development environment"""
        return (
            dag.container()
            .from_("rust:1.75-slim")
            .with_workdir("/app")
            .with_directory("/app", src)
            .with_exec(["cargo", "install", "cargo-watch", "cargo-audit"])
            .with_exposed_port(8080)
            .with_service_binding(8080, self.development_service(src))
        )

    @function
    async def development_service(self, src: dagger.Directory) -> dagger.Service:
        """Development service for hot-reloading"""
        return (
            dag.container()
            .from_("rust:1.75-slim")
            .with_workdir("/app")
            .with_directory("/app", src)
            .with_exec(["cargo", "watch", "-x", "run"])
            .with_exposed_port(8080)
            .as_service()
        )


# Entry point for Dagger
@function
async def ci(src: dagger.Directory | None = None) -> list[dagger.Container]:
    """Main CI function"""
    if src is None:
        src = dag.host().directory(".")

    ci_pipeline = RagsystemCi()
    return await ci_pipeline.full_ci(src)


@function
async def dev(src: dagger.Directory | None = None) -> dagger.Service:
    """Development environment"""
    if src is None:
        src = dag.host().directory(".")

    ci_pipeline = RagsystemCi()
    return await ci_pipeline.development_service(src)