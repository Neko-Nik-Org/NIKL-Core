# 🐾 NIKL-Core

**NIKL** is a lightweight, modern programming language designed for **concurrent**, **network-aware**, and **general-purpose** programming. With native support for threads, asynchronous operations, and networking primitives, NIKL aims to simplify the development of scalable and responsive systems.

> 📝 File extension: `.nk`

> ⚡ Goal: Simple concurrency, native networking, no boilerplate.

> **Note**: `NIKL` is independently developed and **not affiliated with or endorsed by the Rust Foundation**

---

## 🔤 What's in a Name?

The name **NIKL** is a blend of personal and symbolic meaning:

* **Nik** comes from **Neko Nik**, derived from **Nikhil** — the creator's name
* **Neko** (猫) means *cat* in Japanese 🐱 — light, agile, and independent
* **L** stands for **Language**, but also completes the phonetic sound of **Nikhil**
* **Nickel**, a lightweight and resilient metal, reflects the language’s design goals: **lightweight**, **robust**, and **efficient**

---

## 🚀 Features

- ✅ Clean, minimal syntax
- ✅ Lightweight, fast, and efficient
- ✅ Built-in support for **package management**
- ✅ Concurrency with `spawn` and `wait`
- ✅ Async execution using a Rust-based async runtime (`tokio`)
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
// Easy function (type hinting is optional)
fn add_numbers(a: Int, b: Int) -> Int {
    return a + b
}

print(add_numbers(1, 2)) // Prints 3
```

**Note**: There's lot to explore in the language, including for loops, while loops, and infinite loops. The syntax is designed to be simple and intuitive

> Syntax highlighter is available for NIKL in [VSCode](https://marketplace.visualstudio.com/items?itemName=Neko-Nik.nikl-language-support)

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

* 🦀 **Rust** — used as the implementation language for performance and safety
* ⚡ **Tokio** — async runtime for concurrency
* 🧩 Custom **lexer**, **parser**, and **bytecode VM**
* 🧪 Tests and tracing support via `tracing` and `cargo test`

---

## 📌 Roadmap

* [x] Tokenizer / Lexer
* [x] Parser
* [x] Core Interpreter (Eval / Runtime)
* [x] Module System
* [x] Basic I/O
* [x] OS, Regex internal module support
* [ ] `spawn` and `wait` for concurrency
* [ ] File & socket I/O
* [ ] Native async DNS, HTTP, etc.
* [ ] Package system (`nikl-pkg`)
* [ ] Compiler (To be planned, not yet started)
* [ ] Documentation & Language Reference
* [ ] Package Manager website

---

## 📚 Coming Soon

* 📦 **Prebuilt binaries**: Easily download and run `nikl` without building from source.
* 📖 **Documentation & Language Reference**: Comprehensive guides and examples.
* 🌐 **Official Website**: [nikl.nekonik.com](https://nikl.nekonik.com) (Work in Progress).
* 🛠 **Package Manager**: `nikl-pkg` for managing dependencies and modules.
* 🧪 **Standard Library**: Core utilities for common tasks.

---

## 🌐 Official Domains

The following domains and subdomains are officially owned and maintained by the author (Neko Nik):

* [nikl.nekonik.com](https://nikl.nekonik.com) - **Official NIKL website**
* [VS Code Extension](https://marketplace.visualstudio.com/items?itemName=Neko-Nik.nikl-language-support) - **NIKL Language Support for Visual Studio Code**
* [nikl-pkg.nekonik.com](https://nikl-pkg.nekonik.com) - **NIKL Package Manager** (Not yet live/implemented)
* [api.nikl-pkg.nekonik.com](https://api.nikl-pkg.nekonik.com) - **NIKL Package Manager API** (Not yet live/implemented)
* [forums.nekonik.com](https://forums.nekonik.com) - **Neko Nik Forums**


Below are the official links to the author's personal website and social media:

* [nekonik.com](https://www.nekonik.com) - **Author's Website**
* [status.nekonik.com](https://status.nekonik.com) - **Author's Status Page**
* [github.com/Neko-Nik](https://github.com/Neko-Nik) - **Author's GitHub**
* [orcid.org/0009-0008-3435-5674](https://orcid.org/0009-0008-3435-5674) - **Author's ORCID** (Open Researcher and Contributor ID)
* [linkedin.com/in/neko-nik](https://www.linkedin.com/in/neko-nik) - **Author's LinkedIn**


## 📨 Stay Updated

**Join the newsletter** and get instant updates on releases, features, and community announcements:

👉🏻 [https://mailer.nekonik.com/subscription/form](https://mailer.nekonik.com/subscription/form)


## 👤 Author

Made with ❤️ by [Neko Nik](https://www.nekonik.com/impressum)

> Join the discussion on the [Neko Nik Forums](https://forums.nekonik.com)

## 🪪 License

NIKL-Core is licensed under the [MIT License](https://github.com/Neko-Nik/NIKL-Core/blob/main/LICENSE) — use it freely, with attribution. No warranties, no liability.

Perfect — you're on the right track. Here's a clean, simple, and legally sound version of your license section, keeping the tone clear and minimal while hitting all your points:


> ⚠️ **Disclaimer**:

> • All domains under *.nekonik.com, including but not limited to nikl.nekonik.com, are owned and managed by the author, Nikhil ("Neko Nik"). These are the only official sources for NIKL-related materials, documentation, and announcements.

> • You are responsible for any code written or executed using NIKL.

> • Modifications to the interpreter are at your own risk.

> • This is a personal project, not affiliated with or endorsed by the Rust Foundation or any other organization.

> • This is a work in progress — features may change as development continues.

> • Use at your own risk. No warranties or guarantees are provided. By using NIKL, you agree to these terms.

> • For more details, see the [License](https://github.com/Neko-Nik/NIKL-Core/blob/main/LICENSE) file.
