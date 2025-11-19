# Contributing to RusTV

Thank you for your interest in contributing to RusTV! This document provides guidelines and information for contributors.

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Git
- NDI SDK (for testing with real NDI devices)

### Setting Up Development Environment

1. Fork and clone the repository:
```bash
git clone https://github.com/YOUR_USERNAME/RusTV.git
cd RusTV
```

2. Build the project:
```bash
cargo build
```

3. Run tests:
```bash
cargo test
```

## Development Workflow

### Code Style

We use `rustfmt` for code formatting. Before committing, run:

```bash
cargo fmt --all
```

### Linting

We use `clippy` for linting. Ensure your code passes clippy checks:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Testing

- Write tests for new functionality
- Ensure all tests pass before submitting a PR
- Run tests with: `cargo test`

### Commit Messages

Follow conventional commit format:

- `feat:` New features
- `fix:` Bug fixes
- `docs:` Documentation changes
- `test:` Test additions or modifications
- `refactor:` Code refactoring
- `chore:` Maintenance tasks

Example:
```
feat: add support for NDI group filtering
fix: correct PTZ position calculation
docs: update README with new examples
```

## Project Structure

```
RusTV/
├── src/
│   ├── main.rs           # CLI application entry point
│   ├── config.rs         # Configuration management
│   ├── ndi/              # NDI integration
│   │   ├── mod.rs
│   │   ├── discovery.rs  # Source autodiscovery
│   │   ├── receiver.rs   # Stream receiver
│   │   └── source.rs     # Source representation
│   ├── matrix/           # Matrix routing
│   │   ├── mod.rs
│   │   └── router.rs     # Routing logic
│   └── birddog/          # BirdDog camera API
│       ├── mod.rs
│       ├── api.rs        # API client
│       └── ptz.rs        # PTZ control
├── Cargo.toml            # Dependencies
├── README.md             # Main documentation
├── EXAMPLES.md           # Usage examples
└── CONTRIBUTING.md       # This file
```

## Adding New Features

### NDI Features

When adding NDI-related features:
1. Update `src/ndi/` modules
2. Add tests in the same file
3. Update documentation in README.md
4. Add examples in EXAMPLES.md

### Matrix Routing Features

When adding routing features:
1. Update `src/matrix/router.rs`
2. Add corresponding CLI commands in `src/main.rs`
3. Update configuration schema if needed
4. Add tests and examples

### BirdDog API Features

When adding camera control features:
1. Update `src/birddog/` modules
2. Follow BirdDog API specifications
3. Add CLI commands in `src/main.rs`
4. Add tests and examples

## Testing Guidelines

### Unit Tests

Place unit tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Test implementation
    }
}
```

### Integration Tests

For integration tests, create files in `tests/` directory:

```rust
// tests/integration_test.rs
#[test]
fn test_end_to_end_workflow() {
    // Test implementation
}
```

### Running Specific Tests

```bash
# Run a specific test
cargo test test_name

# Run tests for a specific module
cargo test ndi::

# Run with output
cargo test -- --nocapture
```

## Documentation

### Code Documentation

Use Rust doc comments:

```rust
/// Brief description of the function
///
/// # Arguments
///
/// * `param` - Description of parameter
///
/// # Examples
///
/// ```
/// let result = function(param);
/// ```
pub fn function(param: Type) -> Result<()> {
    // Implementation
}
```

### README Updates

When adding features, update:
- Feature list
- Usage examples
- Configuration documentation

## Pull Request Process

1. Create a feature branch:
```bash
git checkout -b feature/your-feature-name
```

2. Make your changes and commit:
```bash
git add .
git commit -m "feat: your feature description"
```

3. Push to your fork:
```bash
git push origin feature/your-feature-name
```

4. Create a Pull Request on GitHub

5. Ensure CI passes:
   - All tests pass
   - Code is formatted
   - No clippy warnings
   - Builds successfully on all platforms

### PR Checklist

- [ ] Code follows project style guidelines
- [ ] Tests added for new functionality
- [ ] All tests pass
- [ ] Documentation updated
- [ ] Examples added if applicable
- [ ] Commit messages follow convention
- [ ] No compiler warnings
- [ ] No clippy warnings

## Issue Reporting

When reporting issues, include:

1. **Description**: Clear description of the issue
2. **Steps to Reproduce**: Detailed steps to reproduce
3. **Expected Behavior**: What should happen
4. **Actual Behavior**: What actually happens
5. **Environment**:
   - OS and version
   - Rust version
   - RusTV version
6. **Logs**: Relevant log output (use `RUST_LOG=debug`)

## Feature Requests

When requesting features:

1. **Use Case**: Describe the use case
2. **Proposed Solution**: Suggest how it could work
3. **Alternatives**: Alternative approaches considered
4. **Examples**: Example usage

## Code Review

All submissions require review. Be open to feedback and willing to make changes.

### Review Criteria

- Code quality and readability
- Test coverage
- Documentation
- Performance considerations
- Security implications
- API design

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Questions?

If you have questions, feel free to:
- Open an issue on GitHub
- Start a discussion in GitHub Discussions

## Resources

- [Rust Documentation](https://doc.rust-lang.org/)
- [NDI SDK Documentation](https://ndi.tv/sdk/)
- [BirdDog API Documentation](https://birddog.tv/)
- [Tokio Documentation](https://tokio.rs/)

Thank you for contributing to RusTV!
