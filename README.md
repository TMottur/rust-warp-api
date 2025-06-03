# Rust Warp API

A learning project inspired by Bastian Gruber’s *Rust Web Programming*, this API is built using the Warp framework. It serves as a practical exploration into building RESTful services in Rust, focusing on clean architecture, modular design, and asynchronous programming.

---

## 🚀 Features

- **CRUD Operations**: Manage resources with standard Create, Read, Update, and Delete endpoints.
- **Asynchronous Handling**: Utilizes Tokio for efficient, non-blocking request processing.
- **Modular Architecture**: Separation of concerns with distinct modules for routes, handlers, and models.
- **JSON Serialization**: Employs Serde for seamless JSON data handling.
- **In-Memory Data Store**: Uses a thread-safe `RwLock` wrapped in an `Arc` for shared state management.
- **Comprehensive Error Handling**: Custom error types and rejection handling for robust API responses.

---

## 🛠 Project Structure

\```
├── src/
│   ├── main.rs          # Entry point: initializes the server and routes
│   ├── routes.rs        # Defines API routes and filters
│   ├── handlers.rs      # Contains request handlers for each endpoint
│   ├── models.rs        # Data models and types
│   └── store.rs         # In-memory data store implementation
├── Cargo.toml           # Project metadata and dependencies
└── README.md            # Project overview and instructions
\```

---

## 📦 Dependencies

- [warp](https://crates.io/crates/warp): Web framework for building APIs.
- [tokio](https://crates.io/crates/tokio): Asynchronous runtime for Rust.
- [serde](https://crates.io/crates/serde) & [serde_json](https://crates.io/crates/serde_json): Serialization and deserialization of JSON data.
- [uuid](https://crates.io/crates/uuid): Generation of unique identifiers.
- [parking_lot](https://crates.io/crates/parking_lot): Provides efficient synchronization primitives.

---

## 🧪 Running the Project

1. **Clone the repository**:

\```bash
git clone https://github.com/TMottur/rust-warp-api.git
cd rust-warp-api
\```

2. **Build and run the server**:

\```bash
cargo run
\```

The server will start on `http://localhost:3030`.

---

## 📬 API Endpoints

- **GET /items**: Retrieve a list of all items.
- **GET /items/{id}**: Retrieve a specific item by ID.
- **POST /items**: Create a new item.
- **PUT /items/{id}**: Update an existing item.
- **DELETE /items/{id}**: Delete an item by ID.

Each endpoint expects and returns JSON-formatted data.

---

## 🧰 Example Usage

**Create a new item**:

\```bash
curl -X POST http://localhost:3030/items \
     -H "Content-Type: application/json" \
     -d '{"name": "Sample Item", "description": "This is a test item."}'
\```

**Retrieve all items**:

\```bash
curl http://localhost:3030/items
\```

**Update an item**:

\```bash
curl -X PUT http://localhost:3030/items/{id} \
     -H "Content-Type: application/json" \
     -d '{"name": "Updated Item", "description": "Updated description."}'
\```

**Delete an item**:

\```bash
curl -X DELETE http://localhost:3030/items/{id}
\```

---

## 🧭 Future Enhancements

- **Persistent Storage**: Integrate a database (e.g., PostgreSQL) for data persistence.
- **Authentication**: Implement JWT-based authentication for secure endpoints.
- **Validation**: Add request data validation to ensure data integrity.
- **Testing**: Write unit and integration tests for handlers and routes.
- **Logging**: Incorporate structured logging for better observability.

---

## 📚 References

- *Rust Web Programming* by Bastian Gruber
- [Warp Documentation](https://docs.rs/warp)
- [Tokio Documentation](https://docs.rs/tokio)

---

Feel free to contribute or raise issues. Your feedback is valuable!
