# 🐾 NIKL-Core

**Nikl** is a lightweight, modern programming language designed for **concurrent**, **network-aware**, and **general-purpose** programming. With native support for threads, asynchronous operations, and networking primitives, Nikl aims to simplify the development of scalable and responsive systems.

> 📝 File extension: `.nk`

> ⚡ Goal: Simple concurrency, native networking, no boilerplate.

---

## 🔤 What's in a Name?

The name **NIKL** is a blend of personal and symbolic meaning:

* **Nik** comes from **Neko Nik**, derived from **Nikhil** — the creator's name
* **Neko** (猫) means *cat* in Japanese 🐱 — light, agile, and independent
* **L** stands for **Language**, but also completes the phonetic sound of **Nikhil**
* **Nickel**, a lightweight and resilient metal, reflects the language’s design goals: **lightweight**, **robust**, and **efficient**

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

## 🛠 Tech Stack

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
