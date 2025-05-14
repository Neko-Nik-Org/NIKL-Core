# 🐾 NIKL-Core

**Nikl** is a lightweight, modern programming language designed for **concurrent**, **network-aware**, and **general-purpose** programming. With native support for threads, asynchronous operations, and networking primitives, Nikl aims to simplify the development of scalable and responsive systems.

This repository — **NIKL-Core** — contains the **Rust-based interpreter** for Nikl, including its lexer, parser, async runtime integration, and core execution engine.

> 📝 File extension: `.nk`  
> ⚡ Goal: Simple concurrency, native networking, no boilerplate.

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
````

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
* [x] Parser
* [x] Core Interpreter (Eval / Runtime)
* [x] Basic async + `spawn`, `wait`
* [ ] File & socket I/O
* [ ] Native async DNS, HTTP, etc.
* [ ] Package system (`nikl-pkg`)
* [ ] Compiler (WIP)

---

## 📚 Coming Soon

* 📦 Prebuilt binaries (`nikl`)
* 🧰 CLI Tools (`nikl run`, `nikl check`)
* 📖 Documentation & Language Reference
* 🌐 nikl.dev website

---

## 👤 Author

Made with ❤️ by [Neko Nik](https://nekonik.com)

> Follow progress and contribute at [github.com/Neko-Nik/NIKL-Core](https://github.com/Neko-Nik/NIKL-Core)

---

## 📄 License

MIT License — see [LICENSE](./LICENSE)

---

```

Would you like:
- A `CONTRIBUTING.md`?
- GitHub labels + issue templates (`feature`, `bug`, `good first issue`)?
- A `nikl.nk` standard test script?
- Or help writing the docs landing page for `nikl.dev`?
```
