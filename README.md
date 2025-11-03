# nof1-tracker-rust

This is the Rust implementation of the [Nof1 AI Agent follow-trading CLI](https://github.com/terryso/nof1-tracker).

## 🚀 Quick Start

### 1. Build the workspace

```bash
cd nof1-tracker-rust
cargo build --release
```

### 2. Run the CLI

```bash
cargo run --bin nof1_cli -- [COMMAND] [OPTIONS]
```

Example:

```bash
# List available agents
cargo run --bin nof1_cli -- agents

# Follow an agent (stub)
cargo run --bin nof1_cli -- follow gpt-5 --interval 30 --price-tolerance 0.5 --total-margin 10

# Show profit report (stub)
cargo run --bin nof1_cli -- profit

# Show system status (stub)
cargo run --bin nof1_cli -- status
```

### 3. Run tests

```bash
cargo test --all
```

### 4. Formatting and linting

```bash
cargo fmt --all
cargo clippy --all --all-targets
```
### 5. Run cargo audit
```bash
# Install cargo-audit
cargo install cargo-audit

# Optional: Update the advisory database 
cargo audit update

# Run the audit in your Rust project folder
cargo audit
```


## 🐳 Run in Docker

Build and run the CLI in a container:

```bash
docker build -f .devcontainer/Dockerfile -t nof1-cli .
docker run --rm nof1-cli agents
```

Replace `agents` with any CLI command as needed.

## 🧑‍💻 VS Code Devcontainer

To develop in a fully configured environment:

1. Open the project folder in VS Code.
2. If prompted, "Reopen in Container" (or use the Command Palette: "Dev Containers: Reopen in Container").
3. The devcontainer will build and set up Rust tools automatically.
4. You can run, build, and test as usual inside the container.


## 📁 Workspace Structure

- `cli/` - Command-line interface crate
- `core/` - Core domain types and logic
- `services/` - API clients and business logic
- `storage/` - Persistence layer (file-based, atomic writes)
- `tests/` - Integration and unit tests (to be added)

## 📝 Notes

- Example commands above use stub implementations; full business logic will be added in later epics.
- Configuration is loaded from environment variables and/or TOML files (to be implemented).
- Do not commit API keys or secrets to the repository.

---
