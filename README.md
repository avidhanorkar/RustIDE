# RustIDE

An **in-browser IDE** built with **Rust (Axum)** for the backend and **React** for the frontend.  
Supports **real-time code editing, execution, and collaboration**.  
The backend is powered by **Tokio + PostgreSQL**, providing a scalable and fast environment for developers.

---

## ✨ Features

- 🔑 User Authentication (Sign up / Login / Session handling)
- 📂 Project & File Management
- 🖊️ In-Browser Code Editor (with syntax highlighting)
- ⚡ Code Execution via isolated sandboxes (Docker / WASM runtimes)
- 🤝 Real-time Collaboration (WebSockets)
- 📜 Execution History & Error Logs
- 🗄️ PostgreSQL-backed persistence
- 📡 REST + WebSocket APIs

---

## 🛠️ Tech Stack

### Backend
- [Rust](https://www.rust-lang.org/) (Async runtime)
- [Axum](https://github.com/tokio-rs/axum) (Web framework)
- [Tokio](https://tokio.rs/) (Async runtime)
- [Tokio-Postgres](https://docs.rs/tokio-postgres/latest/tokio_postgres/) (Database)
- [Serde](https://serde.rs/) (Serialization / Deserialization)
- [jsonwebtoken](https://github.com/Keats/jsonwebtoken) (JWT Auth)
- [bcrypt](https://docs.rs/bcrypt/) (Password hashing)

### Frontend
- [React](https://react.dev/)
- [Monaco Editor](https://microsoft.github.io/monaco-editor/) (VSCode editor engine)
- [TailwindCSS](https://tailwindcss.com/)

### DevOps
- [Docker](https://www.docker.com/) (Sandboxed code execution)
- [PostgreSQL](https://www.postgresql.org/) (Database)
- [Redis](https://redis.io/) (Optional - for caching sessions)

---

## Project Structure

### Backend Structure
ide-backend/
├─ Cargo.toml # Project dependencies & metadata
├─ .env # Environment variables (DB URL, secrets, etc.)
├─ migrations/ # SQL migrations (init tables, alter, etc.)
│ ├─ 0001_create_users.sql
│ ├─ 0002_create_projects.sql
│ ├─ 0003_create_files.sql
│ └─ 0004_create_runs.sql
└─ src/
├─ main.rs # App entrypoint (Axum server bootstrap)
│
├─ config.rs # Load config (dotenv, env vars, runtime config)
├─ db.rs # Database connection (tokio-postgres client setup)
│
├─ models/ # Domain models (plain structs + from_row mappers)
│ ├─ mod.rs # Re-exports
│ ├─ user.rs # User struct + from_row
│ ├─ project.rs # Project struct + from_row
│ ├─ file.rs # File struct + from_row
│ └─ run.rs # Run struct + from_row (execution metadata)
│
├─ repositories/ # DB access layer (CRUD functions using tokio-postgres)
│ ├─ mod.rs
│ ├─ user_repo.rs # create/get/find user
│ ├─ project_repo.rs # create/list projects
│ ├─ file_repo.rs # file CRUD (by project + path)
│ └─ run_repo.rs # insert/update run records (logs/status)
│
├─ services/ # Business logic (use repos, enforce rules)
│ ├─ mod.rs
│ ├─ auth_service.rs # Signup, login, password hashing, token mgmt
│ ├─ project_service.rs # project creation, access checks
│ ├─ file_service.rs # file save/load, validation, CRDT snapshot hooks
│ └─ run_service.rs # choose runner (WASI vs Docker), enqueue jobs
│
├─ handlers/ # HTTP handlers (parse req -> call services -> respond)
│ ├─ mod.rs
│ ├─ auth_handler.rs # /api/auth/* routes
│ ├─ project_handler.rs # /api/projects/*
│ ├─ file_handler.rs # /api/projects/:id/files/*
│ └─ run_handler.rs # /api/projects/:id/run
│
├─ ws/ # WebSocket handlers & hubs
│ ├─ mod.rs
│ ├─ editor_ws.rs # /ws/editor/:project_id (CRDT sync)
│ └─ terminal_ws.rs # /ws/terminal/:run_id (PTY proxy)
│
├─ lsp/ # LSP proxying & lifecycle manager
│ ├─ mod.rs
│ └─ proxy.rs # spawn language servers, forward JSON-RPC
│
├─ exec/ # Runners & sandboxing
│ ├─ mod.rs
│ ├─ wasi_runner.rs # run WASM/WASI modules (wasmtime)
│ └─ docker_runner.rs # run code in hardened Docker containers
│
├─ routes.rs # Compose Axum Router and mount middleware
├─ errors.rs # AppError types & conversion to HTTP responses
└─ utils/ # Small helpers
├─ jwt.rs # JWT encode/verify helpers
├─ hashing.rs # password hashing (argon2/bcrypt wrappers)
├─ response.rs # standard API response helpers
└─ logging.rs # tracing initialization


---
