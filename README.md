# Fitness Progress Tracker - Backend API

A secure, efficient, and reliable backend API for the Fitness Progress Tracker mobile app, built with Rust.

## Project Overview

This API provides the following core functionalities:
- User authentication (signup, login, profile management)
- Workout tracking and management
- Progress analytics
- Goal setting and tracking
- Secure data storage

## Getting Started

### Prerequisites

- Rust (1.70+)
- PostgreSQL (15.0+)
- Docker (optional, for containerized development)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/ghodeaniket/Fitness-Progress-Tracker.git
cd Fitness-Progress-Tracker
```

2. Set up the environment variables:
```bash
cp .env.example .env
# Edit .env with your configuration
```

3. Build the project:
```bash
cargo build
```

4. Run the server:
```bash
cargo run
```

The API will be available at `http://localhost:8080`.

### Using Docker Compose

For a quick setup with Docker:

```bash
docker-compose up -d
```

This will start both the API server and a PostgreSQL database with the correct schema.

## API Documentation with Swagger UI

The API comes with built-in Swagger UI documentation, making it easy to explore and test endpoints.

- Access Swagger UI: `http://localhost:8080/swagger-ui/`
- OpenAPI JSON: `http://localhost:8080/api-docs/openapi.json`

You can use Swagger UI to:
- Explore all available endpoints
- Test API calls directly from the browser
- See request/response formats
- Authenticate with JWT tokens

## Project Structure

```
src/
├── api/            # API endpoints and request handlers
├── config/         # Configuration management
├── db/             # Database connection and migrations
├── models/         # Data models and schemas
├── services/       # Business logic and service layer
└── utils/          # Utility functions and helpers
```

## Learning Rust

This project is designed to be beginner-friendly for developers learning Rust. Here are some key Rust concepts used in this project:

- **Ownership and Borrowing**: Understanding Rust's memory management model
- **Error Handling**: Using Result and Option types
- **Async Programming**: Working with async/await for efficient I/O operations
- **Traits**: Implementing behavior for types
- **Macros**: Using Rust macros for code generation

For newcomers to Rust, we recommend checking out:
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- Code comments throughout this project that explain Rust-specific concepts

## Development Roadmap

1. **MVP** - Basic API with authentication and data storage
2. **Phase 2** - Advanced workout tracking and analytics
3. **Phase 3** - Social features and gamification
4. **Phase 4** - Machine learning for workout recommendations

## Testing

### Automated Tests

Run the test suite with:
```bash
cargo test
```

### Local Testing with Swagger UI

The most convenient way to test the API locally is through the Swagger UI interface at `http://localhost:8080/swagger-ui/`.

1. First register a user through the `/api/v1/auth/register` endpoint
2. Log in with the user credentials at `/api/v1/auth/login` to get a JWT token
3. Click the "Authorize" button at the top of the Swagger UI and enter your token (`Bearer your_token_here`)
4. Now you can access protected endpoints like creating workouts or viewing user profiles

## Continuous Integration

This project uses GitHub Actions for CI, which automatically:
- Builds the project
- Runs all tests
- Checks code formatting and linting
- Performs security audits

## Contributing

Contributions are welcome! Please check the [CONTRIBUTING.md](CONTRIBUTING.md) file for guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
