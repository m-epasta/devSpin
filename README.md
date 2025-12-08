# DevSpin CLI

A development environment manager command-line tool.

## Installation

### From Source
```bash
cargo install --path devspin-cli
```

### From GitHub Releases
Download binaries from the latest [release](https://github.com/m-epasta/devSpin/releases).

## Usage

```bash
# Show help
devspin-cli --help

# Run test command without art
devspin-cli test-cmd

# Run test command with ASCII art
devspin-cli test-cmd --w-msg
```

## Development

### Prerequisites
- Rust 1.80 or later
- Make (optional, for convenience)

### Setup
```bash
# Install pre-commit hooks
pip install pre-commit
pre-commit install

# Development tasks
make help  # List all tasks
make all   # Run all checks
make test  # Run tests
make fmt   # Format code
```

### Contributing
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and checks: `make all`
5. Commit your changes
6. Create a pull request

## CI/CD

This project uses GitHub Actions for:
- **Linting**: Rustfmt, Clippy, Security audit
- **Testing**: Multi-platform (Linux, macOS, Windows)
- **MSRV**: Minimum Supported Rust Version check
- **Releases**: Automated binary builds

## License

MIT License
