# 🐾 Nik-Lang

**Nik-Lang** is a lightweight, simple, and concurrent scripting language inspired by Python and Rust — designed for scripting, automation, and networking. It uses a clean syntax with native support for `spawn` and `await`, enabling powerful concurrency with minimal complexity.

> ✨ File extension: `.nk`

---

## 🚀 Features

- ✅ Clean and readable syntax
- ✅ Concurrency with `spawn` and `wait`
- ✅ Async task handling with real Rust async runtime (via `tokio`)
- ✅ Networking and automation-focused primitives
- ✅ Lightweight interpreter written in Rust
- ✅ Simple variable binding and expression evaluation
- ✅ Ideal for scripting, API automation, and task orchestration

---

## 📦 Getting Started

### 1. Clone the Repo

```bash
git clone https://github.com/Neko-Nik/Nik-Lang
cd Nik-Lang
````

### 2. Build the Interpreter

```bash
cargo build --release
```

### 3. Run a `.nk` File

```bash
cargo run -- path/to/script.nk
```
> **Tip:** By setting `export RUST_LOG=nik_lang=trace`, you can enable detailed logging for debugging

---

## 📄 Example `.nk` Script

```nk
let data = spawn fetch_data("https://example.com")
print(1 + 1)

sleep(1000)

print(wait data)
```

This script:

* Spawns an asynchronous API fetch
* Prints a calculation
* Waits 1 second
* Blocks until the response is ready, then prints it

---

## 🧠 Language Concepts

| Feature       | Syntax Example               |
| ------------- | ---------------------------- |
| Variable      | `let x = 10`                 |
| Function Call | `fetch_data("url")`          |
| Concurrency   | `let t = spawn do_task()`    |
| Awaiting      | `let res = wait t`           |
| Print         | `print("Hello")`             |
| Sleep         | `sleep(1000)` (milliseconds) |

---

## 🛠 Tech Stack

* 🦀 Rust (core interpreter)
* ⚙️ `tokio` for async runtime
* 🧪 Custom lexer, parser, and runtime
