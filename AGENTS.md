# Agent Guidelines

## Build/Test Commands
- `cargo build` - Build the project
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run specific test
- `cargo clippy` - Lint code
- `cargo fmt` - Format code

## Code Style
- Use `snake_case` for variables/functions, `PascalCase` for types
- Import style: `use crate::{module::{Type, func}}` with trailing commas
- Error handling: Use `Result<T, Error>` with custom `Error` enum
- Use `pub(crate)` for module-internal visibility
- Format with rustfmt nightly
- Add doc comments for public APIs
- Prefer explicit types over inference for public interfaces

## Commit Guidelines

- Use conventional commit format: `<type>: <description>`
- Types: `feat`, `fix`, `refactor`, `test`, `docs`, `style`, `perf`, `chore`
- Examples: `feat: add ZSTD support`, `fix: correct assertion bug`, `refactor: rename constants`
- Always run `cargo test && cargo fmt && cargo check` before committing
- Use `git commit --amend` to fix commit messages (never amend pushed commits)
- Workflow: make changes → run tests → stage → commit → amend if needed
- Verify with `git log --oneline -1`
