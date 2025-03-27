# Odor API Development Guidelines

## Build & Test Commands
- **Build**: `cargo build`
- **Run**: `cargo run`
- **Check**: `cargo check`
- **Test all**: `cargo test`
- **Test single**: `cargo test test_name`
- **Test specific module**: `cargo test --package rust_hex --lib your_module_path`
- **Format code**: `cargo fmt`
- **Lint**: `cargo clippy -- -D warnings`

## Code Style Guidelines
- **Imports**: Group by crate/standard library first, then local modules
- **Naming**: snake_case for variables/functions, CamelCase for types/traits, I-prefix for traits (IUserRepository)
- **Error Handling**: Use anyhow::Result with detailed error messages and context
- **Architecture**: Hexagonal architecture with entity, repository, and usecase layers
- **Interface Pattern**: Define traits (ports) with "I" prefix, implement with "adap_" prefix
- **Dependencies**: Use Arc for shared ownership of connection pools and config
- **Async/Await**: Use async_trait for implementing async trait methods
- **Database**: Use sqlx with PostgreSQL, store SQL in r#" "# raw strings
- **Logging**: Use tracing with appropriate log levels (info, error, debug)
- **Testing**: Use mockall for mocking dependencies in tests

## Project Structure
- `src/config`: Application configuration
- `src/services`: Business logic separated by domain
- `src/utils`: Shared utilities (database, server, etc.)
- `src/services/*/entity`: Domain models
- `src/services/*/repository`: Data access layer
- `src/services/*/usecase`: Business logic layer

## Task Notes
- File:src/route/route.rs Fix this for using data from UserHandler Layer