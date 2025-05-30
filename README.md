# Rust Warp API

A simple RESTful web API built with [Warp](https://github.com/seanmonstar/warp) and Rust.

This project implements basic routing, in-memory data storage, and query-based filtering. It is intended as a learning tool and foundation for building more advanced web backends in Rust.

## Features

- Warp-based HTTP server
- RESTful routing
- In-memory question store
- Query parameter support
- Health check endpoint (`/healthz`)
- Error recovery

## Getting Started

### Prerequisites

- [Rust](https://rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/)
- Git (SSH access configured)

### Running the Server

```bash
cargo run
```

Server will be available at `http://localhost:3030`.

### Example Endpoints

- `GET /questions?start=0&end=10` — Fetch paginated questions
- `GET /healthz` — Returns `200 OK` for health checks

## License

MIT
