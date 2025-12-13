# Rust Learning Guide for Go Developers

## สารบัญ

1. [Setup Environment](#1-setup-environment)
2. [Rust vs Go Comparison](#2-rust-vs-go-comparison)
3. [Core Concepts](#3-core-concepts)
4. [Project Structure](#4-project-structure)
5. [Common Patterns](#5-common-patterns)
6. [Web Development with Axum](#6-web-development-with-axum)
7. [Database with SQLx](#7-database-with-sqlx)
8. [Error Handling](#8-error-handling)
9. [Async Programming](#9-async-programming)
10. [Development Workflow](#10-development-workflow)

---

## 1. Setup Environment

### Install Rust

```bash
# Install rustup (Rust toolchain manager)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Update to latest
rustup update

# Check version
rustc --version
cargo --version
```

### Useful Tools

```bash
# Cargo watch - auto-recompile on file changes
cargo install cargo-watch

# SQLx CLI - database migrations
cargo install sqlx-cli --features postgres

# Cargo expand - see macro expansions
cargo install cargo-expand
```

### IDE Setup (VS Code)

Extensions แนะนำ:
- **rust-analyzer** - Language server (สำคัญมาก)
- **Even Better TOML** - Cargo.toml syntax
- **crates** - Dependency version hints
- **Error Lens** - Inline error display

---

## 2. Rust vs Go Comparison

### Variables & Types

```go
// Go
var name string = "hello"
name := "hello"
const MAX = 100
```

```rust
// Rust
let name: String = String::from("hello");
let name = "hello"; // &str (string slice)
const MAX: i32 = 100;

// Mutable variable
let mut count = 0;
count += 1;
```

### Functions

```go
// Go
func add(a, b int) int {
    return a + b
}

func divide(a, b int) (int, error) {
    if b == 0 {
        return 0, errors.New("division by zero")
    }
    return a / b, nil
}
```

```rust
// Rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // no semicolon = return value
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("division by zero".to_string());
    }
    Ok(a / b)
}
```

### Structs & Methods

```go
// Go
type User struct {
    ID   int
    Name string
}

func (u *User) Greet() string {
    return "Hello, " + u.Name
}
```

```rust
// Rust
struct User {
    id: i32,
    name: String,
}

impl User {
    // Constructor (convention: new)
    fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }

    // Method with &self (like Go receiver)
    fn greet(&self) -> String {
        format!("Hello, {}", self.name)
    }

    // Method that modifies self
    fn rename(&mut self, new_name: String) {
        self.name = new_name;
    }
}
```

### Error Handling

```go
// Go
result, err := doSomething()
if err != nil {
    return err
}
```

```rust
// Rust - Option 1: Match
let result = do_something();
match result {
    Ok(value) => println!("Got: {}", value),
    Err(e) => return Err(e),
}

// Rust - Option 2: ? operator (preferred)
let value = do_something()?;  // returns early if Err
```

### Collections

```go
// Go
nums := []int{1, 2, 3}
nums = append(nums, 4)

m := map[string]int{"a": 1}
m["b"] = 2
```

```rust
// Rust
let mut nums = vec![1, 2, 3];
nums.push(4);

use std::collections::HashMap;
let mut m = HashMap::new();
m.insert("a", 1);
m.insert("b", 2);
```

### Interfaces vs Traits

```go
// Go Interface
type Reader interface {
    Read(p []byte) (n int, err error)
}
```

```rust
// Rust Trait
trait Reader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error>;
}

// Implement trait for a type
impl Reader for MyFile {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        // implementation
    }
}
```

---

## 3. Core Concepts

### 3.1 Ownership (สำคัญมาก!)

Rust ไม่มี Garbage Collector แต่ใช้ระบบ Ownership แทน

**Rules:**
1. แต่ละ value มี owner เดียว
2. เมื่อ owner ออกจาก scope, value ถูก drop
3. Value สามารถถูก move หรือ borrow ได้

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is MOVED to s2
    // println!("{}", s1);  // ERROR! s1 no longer valid

    let s3 = s2.clone();  // deep copy
    println!("{} {}", s2, s3);  // OK
}
```

### 3.2 Borrowing & References

```rust
fn main() {
    let s = String::from("hello");

    // Immutable borrow (&)
    let len = calculate_length(&s);
    println!("{} has length {}", s, len);  // s still valid

    // Mutable borrow (&mut)
    let mut s2 = String::from("hello");
    change(&mut s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" world");
}
```

**Borrowing Rules:**
- หลาย immutable borrows (&T) พร้อมกันได้
- หรือ หนึ่ง mutable borrow (&mut T) เท่านั้น
- ไม่สามารถมีทั้งสองพร้อมกัน

### 3.3 Lifetimes

Lifetimes บอก compiler ว่า reference มีชีวิตอยู่นานแค่ไหน

```rust
// Explicit lifetime annotation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Struct with references needs lifetime
struct Config<'a> {
    name: &'a str,
}
```

### 3.4 Option & Result

```rust
// Option - value ที่อาจจะไม่มี (แทน null/nil)
fn find_user(id: i32) -> Option<User> {
    if id == 1 {
        Some(User { id: 1, name: "Alice".to_string() })
    } else {
        None
    }
}

// Usage
match find_user(1) {
    Some(user) => println!("Found: {}", user.name),
    None => println!("Not found"),
}

// Or use if let
if let Some(user) = find_user(1) {
    println!("Found: {}", user.name);
}

// Or unwrap (panics if None - use carefully!)
let user = find_user(1).unwrap();
let user = find_user(1).expect("User must exist");

// Or provide default
let user = find_user(1).unwrap_or(default_user);
```

```rust
// Result - operation ที่อาจ fail
fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse()
}

// Usage with ? operator
fn process() -> Result<(), Error> {
    let num = parse_number("42")?;  // returns early if Err
    println!("Got: {}", num);
    Ok(())
}
```

---

## 4. Project Structure

```
backend/
├── Cargo.toml              # Dependencies (like go.mod)
├── Cargo.lock              # Lock file (like go.sum)
├── .env                    # Environment variables
├── migrations/             # SQL migrations
│   ├── 000001_init.up.sql
│   └── 000001_init.down.sql
└── src/
    ├── main.rs             # Entrypoint
    ├── config/
    │   └── mod.rs          # Configuration
    ├── db/
    │   ├── mod.rs          # Module declaration
    │   ├── postgres.rs     # PostgreSQL connection
    │   └── redis.rs        # Redis connection
    ├── error/
    │   └── mod.rs          # Error types
    ├── models/
    │   ├── mod.rs
    │   ├── user.rs
    │   └── village.rs
    ├── repositories/
    │   ├── mod.rs
    │   └── user_repo.rs
    ├── services/
    │   ├── mod.rs
    │   └── auth_service.rs
    ├── handlers/
    │   ├── mod.rs
    │   └── auth.rs
    └── middleware/
        ├── mod.rs
        └── auth.rs
```

### Module System

```rust
// src/main.rs
mod config;      // looks for src/config.rs or src/config/mod.rs
mod db;
mod handlers;

use config::Config;
use db::postgres;
```

```rust
// src/db/mod.rs
pub mod postgres;  // pub = public (export)
pub mod redis;
```

```rust
// src/db/postgres.rs
use crate::config::DatabaseConfig;  // crate = root module

pub async fn create_pool(config: &DatabaseConfig) -> Result<PgPool, Error> {
    // ...
}
```

---

## 5. Common Patterns

### 5.1 Builder Pattern

```rust
#[derive(Default)]
struct ServerBuilder {
    port: Option<u16>,
    host: Option<String>,
}

impl ServerBuilder {
    fn new() -> Self {
        Self::default()
    }

    fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    fn build(self) -> Server {
        Server {
            port: self.port.unwrap_or(8080),
            host: self.host.unwrap_or_else(|| "localhost".to_string()),
        }
    }
}

// Usage
let server = ServerBuilder::new()
    .port(3000)
    .host("0.0.0.0")
    .build();
```

### 5.2 Repository Pattern

```rust
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Error>;
    async fn create(&self, user: CreateUser) -> Result<User, Error>;
    async fn update(&self, id: Uuid, user: UpdateUser) -> Result<User, Error>;
    async fn delete(&self, id: Uuid) -> Result<(), Error>;
}

pub struct PostgresUserRepository {
    pool: PgPool,
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Error> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
            .map_err(Into::into)
    }
    // ... other methods
}
```

### 5.3 From/Into Traits

```rust
// Convert between types
struct CreateUserRequest {
    email: String,
    password: String,
}

struct User {
    id: Uuid,
    email: String,
    password_hash: String,
}

impl From<CreateUserRequest> for User {
    fn from(req: CreateUserRequest) -> Self {
        Self {
            id: Uuid::new_v4(),
            email: req.email,
            password_hash: hash_password(&req.password),
        }
    }
}

// Usage
let user: User = request.into();
// or
let user = User::from(request);
```

---

## 6. Web Development with Axum

### Basic Handler

```rust
use axum::{
    extract::{Path, Query, State, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

// Handler function
async fn hello() -> &'static str {
    "Hello, World!"
}

// With path parameter
async fn get_user(Path(id): Path<Uuid>) -> impl IntoResponse {
    Json(User { id, name: "Alice".to_string() })
}

// With query parameters
#[derive(Deserialize)]
struct Pagination {
    page: Option<u32>,
    limit: Option<u32>,
}

async fn list_users(Query(params): Query<Pagination>) -> impl IntoResponse {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    // ...
}

// With JSON body
#[derive(Deserialize)]
struct CreateUser {
    email: String,
    password: String,
}

async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    (StatusCode::CREATED, Json(user))
}

// With app state
async fn get_users(State(state): State<AppState>) -> impl IntoResponse {
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&state.db)
        .await?;
    Json(users)
}
```

### Router Setup

```rust
fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(hello))
        .route("/users", get(list_users).post(create_user))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
        .nest("/api/v1", api_routes())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
```

### Middleware

```rust
use axum::{
    middleware::{self, Next},
    extract::Request,
    response::Response,
};

async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Get token from header
    let token = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(AppError::Unauthorized)?;

    // Verify token
    let claims = verify_token(token, &state.config.jwt.secret)?;

    // Add user to request extensions
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

// Apply middleware
let protected_routes = Router::new()
    .route("/profile", get(get_profile))
    .layer(middleware::from_fn_with_state(state.clone(), auth_middleware));
```

---

## 7. Database with SQLx

### Setup

```bash
# Install CLI
cargo install sqlx-cli --features postgres

# Create database
sqlx database create

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

### Queries

```rust
use sqlx::{PgPool, FromRow};

#[derive(Debug, FromRow, Serialize)]
struct User {
    id: Uuid,
    email: String,
    created_at: DateTime<Utc>,
}

// Query with compile-time checking (requires DATABASE_URL)
async fn find_user(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"SELECT id, email, created_at FROM users WHERE id = $1"#,
        id
    )
    .fetch_optional(pool)
    .await
}

// Dynamic query
async fn find_users(pool: &PgPool, email: Option<&str>) -> Result<Vec<User>, sqlx::Error> {
    let mut query = sqlx::QueryBuilder::new("SELECT id, email, created_at FROM users");

    if let Some(email) = email {
        query.push(" WHERE email LIKE ");
        query.push_bind(format!("%{}%", email));
    }

    query.build_query_as::<User>()
        .fetch_all(pool)
        .await
}

// Insert
async fn create_user(pool: &PgPool, email: &str, password_hash: &str) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (email, password_hash)
        VALUES ($1, $2)
        RETURNING id, email, created_at
        "#,
        email,
        password_hash
    )
    .fetch_one(pool)
    .await
}

// Transaction
async fn transfer_resources(pool: &PgPool, from: Uuid, to: Uuid, amount: i32) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query!(
        "UPDATE villages SET wood = wood - $1 WHERE id = $2",
        amount, from
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query!(
        "UPDATE villages SET wood = wood + $1 WHERE id = $2",
        amount, to
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}
```

### Migrations

```sql
-- migrations/000001_create_users.up.sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- migrations/000001_create_users.down.sql
DROP TABLE users;
```

---

## 8. Error Handling

### Custom Error Type

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication required")]
    Unauthorized,

    #[error("Access denied")]
    Forbidden,

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    BadRequest(String),

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),

    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, self.to_string()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Internal(_) | AppError::Database(_) => {
                tracing::error!("Internal error: {:?}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };

        let body = Json(serde_json::json!({
            "error": { "message": message }
        }));

        (status, body).into_response()
    }
}

// Usage in handlers
async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, AppError> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_optional(&state.db)
        .await?  // Database error auto-converts
        .ok_or_else(|| AppError::NotFound(format!("User {} not found", id)))?;

    Ok(Json(user))
}
```

---

## 9. Async Programming

### Tokio Runtime

```rust
#[tokio::main]
async fn main() {
    // Async code here
}

// Spawning tasks
tokio::spawn(async {
    // Background task
});

// Running multiple futures concurrently
let (result1, result2) = tokio::join!(
    fetch_data_1(),
    fetch_data_2()
);

// Select first completed
tokio::select! {
    result = future1 => { /* handle result */ }
    result = future2 => { /* handle result */ }
}
```

### Channels

```rust
use tokio::sync::mpsc;

// Create channel
let (tx, mut rx) = mpsc::channel::<String>(100);

// Sender
tokio::spawn(async move {
    tx.send("hello".to_string()).await.unwrap();
});

// Receiver
while let Some(message) = rx.recv().await {
    println!("Got: {}", message);
}
```

---

## 10. Development Workflow

### Daily Commands

```bash
# Start development (auto-reload)
cargo watch -x run

# Check code without building
cargo check

# Build debug
cargo build

# Build release
cargo build --release

# Run tests
cargo test

# Run specific test
cargo test test_name

# Format code
cargo fmt

# Lint
cargo clippy

# Update dependencies
cargo update
```

### Database Commands

```bash
# Create migration
sqlx migrate add create_users

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert

# Check migration status
sqlx migrate info
```

### Environment Setup

```bash
# Copy example env
cp .env.example .env

# Required for sqlx compile-time checking
export DATABASE_URL="postgres://user:pass@localhost/db"

# Or use .env file
```

### Project Scripts (Makefile)

```makefile
.PHONY: dev build test lint migrate

dev:
	cargo watch -x run

build:
	cargo build --release

test:
	cargo test

lint:
	cargo clippy -- -D warnings
	cargo fmt --check

migrate-up:
	sqlx migrate run

migrate-down:
	sqlx migrate revert

docker-up:
	docker-compose up -d

docker-down:
	docker-compose down
```

---

## Quick Reference Card

| Go | Rust | Notes |
|----|------|-------|
| `var x int` | `let x: i32` | Immutable by default |
| `x := 5` | `let x = 5` | Type inference |
| `var x int = 5` | `let mut x = 5` | Mutable |
| `nil` | `None` | Option type |
| `error` | `Result<T, E>` | Error handling |
| `if err != nil` | `?` operator | Early return on error |
| `interface{}` | `dyn Trait` / generics | Dynamic dispatch |
| `go func()` | `tokio::spawn()` | Concurrency |
| `chan T` | `mpsc::channel()` | Channels |
| `defer` | `Drop` trait | Cleanup |
| `struct{}` embedding | `impl Trait for` | Composition |
| `package` | `mod` | Modules |
| `import` | `use` | Imports |

---

## Resources

- [The Rust Book](https://doc.rust-lang.org/book/) - Official tutorial
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by examples
- [Axum Documentation](https://docs.rs/axum/latest/axum/) - Web framework
- [SQLx Documentation](https://docs.rs/sqlx/latest/sqlx/) - Database
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial) - Async runtime
- [This Week in Rust](https://this-week-in-rust.org/) - Weekly news

---

*Document Version: 1.0*
*Last Updated: December 2025*
