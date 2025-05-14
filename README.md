# 🐾 NIKL-Core

**Nikl** is a lightweight, modern programming language designed for **concurrent**, **network-aware**, and **general-purpose** programming. With native support for threads, asynchronous operations, and networking primitives, Nikl aims to simplify the development of scalable and responsive systems.

This repository — **NIKL-Core** — contains the **Rust-based interpreter** for Nikl, including its lexer, parser, async runtime integration, and core execution engine.

> 📝 File extension: `.nk`  
> ⚡ Goal: Simple concurrency, native networking, no boilerplate.

> **NOTE:** This project is **not yet complete** and is currently **in progress**.
> There are **no guarantees** regarding stability, functionality, or completeness at this stage.  
Use at your own risk.

---

## 🚀 Features

- ✅ Clean, Python-like syntax
- ✅ Concurrency with `spawn` and `wait`
- ✅ Async execution using real Rust async runtime (`tokio`)
- ✅ Built-in networking and automation primitives
- ✅ Lightweight interpreter written in Rust
- ✅ Minimal dependencies and fast startup
- ✅ Ideal for scripting, orchestration, and lightweight services

---

## 📦 Getting Started

### 1. Clone the Repository

```bash
git clone https://github.com/Neko-Nik/NIKL-Core
cd NIKL-Core
```

### 2. Build the Interpreter

```bash
cargo build --release
```

### 3. Run a Nikl Script

```bash
cargo run -- path/to/script.nk
```

> 💡 For debugging, enable logging:
> `export RUST_LOG=nikl=trace`

---

## 📄 Example Nikl Script

```nk
let data = spawn fetch_data("https://example.com")
print(1 + 1)

sleep(1000)

print(wait data)
```

This script:

* Spawns an async network request
* Prints `2`
* Waits for 1 second
* Awaits and prints the fetched result

---

## 🧠 Core Language Concepts

| Concept       | Example                      |
| ------------- | ---------------------------- |
| Variable      | `let x = 42`                 |
| Function Call | `fetch_data("url")`          |
| Thread Spawn  | `let t = spawn do_task()`    |
| Awaiting      | `let res = wait t`           |
| Output        | `print("Hello World")`       |
| Sleep         | `sleep(1000)` (milliseconds) |

---

## 🛠 Technology Stack

* 🦀 **Rust** — safe and fast systems programming
* ⚡ **Tokio** — async runtime for concurrency
* 🧩 Custom **lexer**, **parser**, and **bytecode VM**
* 🧪 Tests and tracing support via `tracing` and `cargo test`

---

## 📌 Roadmap

* [x] Tokenizer / Lexer
* [ ] Parser
* [ ] Core Interpreter (Eval / Runtime)
* [ ] `spawn` and `wait` for concurrency
* [ ] File & socket I/O
* [ ] Native async DNS, HTTP, etc.
* [ ] Package system (`nikl-pkg`)
* [ ] Compiler (WIP)

---

## 📚 Coming Soon

* 📦 **Prebuilt binaries**: Easily download and run `nikl` without building from source.
* 📖 **Documentation & Language Reference**: Comprehensive guides and examples.
* 🌐 **Official Website**: [nikl.nekonik.com](https://nikl.nekonik.com) (Work in Progress).
* 🛠 **Package Manager**: `nikl-pkg` for managing dependencies and modules.
* 🧪 **Standard Library**: Core utilities for common tasks.

---

## 👤 Author

Made with ❤️ by [Neko Nik](https://nekonik.com)

> Join the discussion on the [Neko Nik Forums](https://forums.nekonik.com)
