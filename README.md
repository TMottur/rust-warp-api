# 🧠 Rust Q&A API

A near-complete Q&A REST API built with **Rust**, **Warp**, **Tokio**, and **Serde**—handling questions, answers, votes, and full CRUD operations.

---

## 🚀 Features

- RESTful endpoints for questions and answers  
- Nested answer routes (`/questions/{id}/answers`)  
- Upvoting/downvoting functionality  
- In-memory database using `Arc<Mutex<_>>`  
- Fully async with `Tokio`  
- Typed JSON parsing via `Serde`

---

## 📦 Getting Started

### Prerequisites

- Rust & Cargo ≥ 1.60

### Clone & Run

````bash
git clone https://github.com/TMottur/rust-warp-api.git
cd rust-warp-api
cargo run
````

Server runs on `http://localhost:3030` by default.  
You can override this using the `PORT` environment variable.

---

## 🧪 API Usage

### Questions

- **Get all questions**
    ````http
    GET /questions
    ````

- **Get a single question**
    ````http
    GET /questions/{id}
    ````

- **Create a question**
    ````http
    POST /questions
    Content-Type: application/json

    {
      "title": "What is Warp?",
      "body": "How does warp's filter system work?"
    }
    ````

- **Update a question**
    ````http
    PUT /questions/{id}
    Content-Type: application/json

    {
      "title": "Updated Title",
      "body": "Updated body content"
    }
    ````

- **Delete a question**
    ````http
    DELETE /questions/{id}
    ````

- **Vote**
    ````http
    POST /questions/{id}/vote/up
    POST /questions/{id}/vote/down
    ````

### Answers

- **List all answers to a question**
    ````http
    GET /questions/{q_id}/answers
    ````

- **Get a single answer**
    ````http
    GET /questions/{q_id}/answers/{a_id}
    ````

- **Create an answer**
    ````http
    POST /questions/{q_id}/answers
    Content-Type: application/json

    {
      "body": "Here's how to use nested filters in Warp..."
    }
    ````

- **Update an answer**
    ````http
    PUT /questions/{q_id}/answers/{a_id}
    Content-Type: application/json

    {
      "body": "Updated answer body..."
    }
    ````

- **Delete an answer**
    ````http
    DELETE /questions/{q_id}/answers/{a_id}
    ````

- **Vote**
    ````http
    POST /questions/{q_id}/answers/{a_id}/vote/up
    POST /questions/{q_id}/answers/{a_id}/vote/down
    ````

---

## 🛠 Implementation Notes

- **In-memory storage**: `type Db = Arc<Mutex<DbInner>>`
- **Handlers** in `handlers.rs`
- **Routes** in `routes.rs`
- **Models** in `models.rs` using Serde
- **Thread-safe state**: shared between handlers using `tokio::sync::Mutex`
