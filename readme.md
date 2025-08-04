# Rust API Starter Pack

![Rust](https://img.shields.io/badge/rust-1.x-orange.svg)
![Axum](https://img.shields.io/badge/Axum-0.7-blue)
![SQLx](https://img.shields.io/badge/SQLx-0.7-green)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-15-blue)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)

A robust and production-ready starter template for building modern web APIs in Rust. This project provides a solid foundation with essential features like authentication, database integration, and security best practices already configured, so you can focus on building your application's logic.

This starter pack is built with a focus on:
*   **Performance:** Leveraging the speed and safety of Rust with the `tokio` runtime.
*   **Developer Experience:** A logical project structure and integrated tooling.
*   **Security:** Common security vulnerabilities are addressed out-of-the-box.

---

## Features

This starter pack comes with a comprehensive set of features pre-configured:

*   **Web Framework:** [**Axum**](https://github.com/tokio-rs/axum) for ergonomic and performant routing.
*   **Asynchronous Runtime:** [**Tokio**](https://tokio.rs/) as the foundation for all async operations.
*   **Database Integration:** [**SQLx**](https://github.com/launchbadge/sqlx) for compile-time verified, asynchronous SQL queries against a **PostgreSQL** database.
*   **Authentication:** JWT-based authentication middleware for securing routes.
*   **Password Hashing:** Secure password handling using [**bcrypt**](https://docs.rs/bcrypt/latest/bcrypt/).
*   **Configuration Management:** Centralized configuration loaded from a `.env` file.
*   **Security Hardening:**
    *   **Rate Limiting:** Protects against DoS attacks using [**Tower Governor**](https://github.com/benwis/tower-governor).
    *   **CORS:** Cross-Origin Resource Sharing middleware configured for frontend integration.
    *   **CSRF:** Cross-site request forgery
    *   **Security Headers:** Essential HTTP security headers (HSTS, X-Frame-Options, etc.) included to protect against common web vulnerabilities.
*   **Error Handling:** A centralized `AppError` type for consistent error responses.
*   **Logging:** Structured logging provided by the `tracing` ecosystem.
*   **Modular Structure:** A clean, organized project layout separating concerns like API handlers, database queries, and services.

## Prerequisites

Before you begin, ensure you have the following installed:
*   [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
*   [PostgreSQL](https://www.postgresql.org/download/) (or Docker)
*   `sqlx-cli` for database migrations and query checking:
    ```bash
    cargo install sqlx-cli
    ```
*   (Optional) `cargo-watch` for live-reloading during development:
    ```bash
    cargo install cargo-watch
    ```

## 🚀 Getting Started

Follow these steps to get your local development environment up and running.

### 1. Clone the Repository

```bash
git clone https://github.com/AlmerKastaraZain/Rust-starter-pack/main
cd <your-project-directory>
```

### 2. Set Up the Database

This project uses PostgreSQL. The easiest way to run it locally is with Docker.

```bash
# Start a PostgreSQL container
docker run --name my-postgres-db -e POSTGRES_PASSWORD=yourpassword -p 5432:5432 -d postgres

# Note: The database credentials should match your .env file.
```

### 3. Configure Your Environment

Configuration is managed via a `.env` file. Copy the example file and customize it.

```bash
# Copy the example .env file
cp .env.example .env
```

Now, open `.env` and fill in the values for your local setup. The default values should work with the Docker command above if you create the user and database.

**Example `.env`:**
```env
# .env 

# General Settings
ENV="development"
DATABASE_URL="postgres://axum_postgres:axumpostgres@127.0.0.1:5432/axum_postgres"
DATABASE_URL="postgres://axum_postgres:axumpostgres@127.0.0.1:5432/axum_postgres"
SERVER_ADDRESS="127.0.0.1:7878"

# Security Settings
# CSRF Settings
CSRF_SECRET="mLirohHZGqUS+Qk1aYTCAIPZ/Q5YpRqOYRLFn9n0RiKhhL5ZcMuaMgyjEMF/s84Pf4Nkh+pfXT+CwrrULp9Olw=="

# CORS Settings (Set to your localhost or url)
FRONTEND_ORIGIN="http://localhost:5173"

# JSON Web Token Settings
JWT_SECRET="gs/wrACujNwM+C5u9up+vYhEUsnruf6OFU+eWk4oT9RrFQdpPTg+KLaYBHeMxbIYPdtDL3CpI/eOJtt3cx3v+A=="
JWT_EXPIRATION_HOURS=1

# Rate Limiting (DDOS Protections)
RATE_LIMIT_REQUESTS=2
RATE_LIMIT_PERIOD_SECONDS=5
```

### 4. Run Database Migrations

With your database running and `.env` configured, apply the database schema. `sqlx-cli` will automatically read your `DATABASE_URL`.

```bash
# Create the database if it doesn't exist
sqlx database create

# Run the migration files located in the /migrations directory
sqlx migrate run
```

### 5. Prepare SQLx Offline Data

This project uses SQLx's offline mode for compile-time query verification. Generate the required data file:

```bash
cargo sqlx prepare
```

### 6. Run the Application

You're all set! Run the server with:

```bash
cargo run
```

You should see output indicating the server is listening, e.g., `🚀 Listening on http://127.0.0.1:7878`.

For development with automatic restarts when you save a file, use `cargo-watch`:

```bash
cargo watch -x run
```

## Project Structure

```
.
├── migrations/         # SQLx database migration files
├── src/
│   ├── api/            # API route handlers (controllers)
│   ├── config/         # Configuration loading module
│   ├── db/             # Database query logic
│   ├── errors/         # Centralized error handling
│   ├── middleware/     # Custom Axum middleware (e.g., auth)
│   ├── models/         # Data structures (e.g., User)
│   ├── utils/          # Utilities 
│   ├── path/           # Axum router definitions
│   ├── service/        # Business logic
│   └── main.rs         # Application entry point
├── .env.example        # Example environment variables
├── .sqlx/              # SQLx offline query data (commit this!)
├── Cargo.toml
└── README.md
```

## Next Steps & Future Enhancements

This starter provides a solid base. Here are some ideas for where to go next:

*   **Input Validation:** Add struct-level validation using the `validator` crate.
*   **Testing:** Write unit tests for your services and integration tests for your API endpoints.
*   **Stronger Hashing:** Consider migrating from `bcrypt` to `Argon2` for password hashing.
*   **Deployment:** Containerize the application with a `Dockerfile` for easy deployment.
*   **CI/CD:** Set up a GitHub Actions workflow to run `cargo check`, `cargo test`, and `cargo audit` on every push.
*   **More Security:** Implement a detailed Content Security Policy (CSP) for your frontend.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
