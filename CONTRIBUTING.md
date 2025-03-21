# Contributing to Fitness Progress Tracker

Thank you for considering contributing to the Fitness Progress Tracker backend API! This document outlines the process for contributing to this project.

## Development Setup

1. **Fork the Repository**
   
   Start by forking the repository to your GitHub account.

2. **Clone the Repository**

   ```bash
   git clone https://github.com/YOUR_USERNAME/Fitness-Progress-Tracker.git
   cd Fitness-Progress-Tracker
   ```

3. **Set Up Environment**

   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

4. **Build and Run**

   ```bash
   cargo build
   cargo run
   ```

## Development Workflow

We follow a simple development workflow:

1. **Create a Feature Branch**

   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make Changes**

   Implement your changes with clear, focused commits.

3. **Run Tests**

   ```bash
   cargo test
   ```

4. **Submit a Pull Request**

   When your changes are ready, submit a pull request to the main repository.

## Commit Guidelines

- Use descriptive commit messages
- Begin commit messages with a verb in present tense (e.g., "Add feature" not "Added feature")
- Reference issue numbers when applicable
- Keep commits focused on a single change/feature

## Code Style Guidelines

- Follow Rust style conventions
- Use `cargo fmt` to format your code
- Use `cargo clippy` to check for common mistakes and improvements
- Add comments for complex logic
- Write clear function and variable names

## Pull Request Process

1. Update the README.md or documentation with details of changes if needed
2. Ensure all tests pass
3. Update the version numbers in Cargo.toml if applicable
4. The PR will be merged once it's reviewed and approved

## Learning Resources

If you're new to Rust, here are some helpful resources:

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)

## Project Structure

The project follows a modular structure:

```
src/
├── api/            # API endpoints and request handlers
├── config/         # Configuration management
├── db/             # Database connection and migrations
├── models/         # Data models and schemas
├── services/       # Business logic and service layer
└── utils/          # Utility functions and helpers
```

## Questions?

If you have any questions about contributing, please open an issue or contact the project maintainers.
