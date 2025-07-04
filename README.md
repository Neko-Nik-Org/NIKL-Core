<div align="center">
    <a href="https://nikl.nekonik.org/">
        <img alt="Nik-Lang" src="https://nikl.nekonik.org/img/logo.svg" width="300px">
        <h1>Nik-Lang</h1>
    </a>
    <strong>Modern, lightweight programming language for concurrent, network-aware, and general-purpose programming</strong>
</div>
<br>
<p align="center">
    <a href="https://github.com/Neko-Nik-Org/NIKL-Core/blob/master/LICENSE">
        <img src="https://img.shields.io/github/license/Neko-Nik/NIKL-Core" alt="License">
    </a>
    <img src="https://img.shields.io/github/languages/code-size/Neko-Nik/NIKL-Core" alt="GitHub code size in bytes">
    <img src="https://img.shields.io/github/commit-activity/w/Neko-Nik/NIKL-Core" alt="GitHub commit activity">
    <img src="https://img.shields.io/docker/pulls/neko7nik/nikl" alt="Docker Pulls">
    <a href="https://github.com/Neko-Nik-Org/NIKL-Core/issues">
        <img src="https://img.shields.io/github/issues/Neko-Nik/NIKL-Core" alt="GitHub issues">
    </a>
    <a href="https://github.com/Neko-Nik-Org/NIKL-Core/pulls">
        <img src="https://img.shields.io/github/issues-pr/Neko-Nik/NIKL-Core" alt="GitHub pull requests">
    </a>
    <!-- <a href="https://github.com/Neko-Nik-Org/NIKL-Core/releases">
        <img src="https://img.shields.io/github/v/release/Neko-Nik/NIKL-Core.svg?style=flat" alt="GitHub Release">
    </a> -->
</p>


**NIKL / Nik-Lang / Nik Programming Language** (pronounced "nickel") is a modern, lightweight programming language designed for **concurrent**, **network-aware**, and **general-purpose** programming. With native support for threads, asynchronous operations, and networking primitives, NIKL aims to simplify the development of scalable and responsive systems.

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

### Prerequisites

* **Docker**: For running the interpreter in a container
* **Rust**: For building the interpreter from source (optional)
* **VSCode**: For editing and syntax highlighting (optional)

### Installation via Docker

If you prefer to use Docker, you can run the NIKL interpreter without installing it on your local machine. This is a great way to quickly test and run Nikl scripts.

1. **Pull the Docker image**:

   ```bash
   docker pull nekonik/nikl:latest
   ```

2. **Run the interpreter**:

   ```bash
    docker run -it --rm nekonik/nikl:latest
    ```
3. **Run a script**:

You can run directly run this command to run the example script provided in the repository:

    ```bash
    docker run --rm -v "$PWD":/examples nekonik/nikl:latest /examples/examples/neko.nk
    ```


### Installation from Source

If you prefer to build from source, follow these steps:

1. **Clone the repository**:

```bash
git clone https://github.com/Neko-Nik-Org/NIKL-Core
cd NIKL-Core
```

2. **Build the project**:

```bash
cargo build --release
```

3. **Run the interpreter**:

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
* [x] Documentation & Language Reference
* [ ] `spawn` and `wait` for concurrency
* [ ] File & socket I/O
* [ ] Native async DNS, HTTP, etc.
* [ ] Package system (`nikl-pkg`)
* [ ] Compiler (To be planned, not yet started)
* [ ] Package Manager website

---

## 📚 Coming Soon

* 📦 **Prebuilt binaries**: Easily download and run `nikl` without building from source.
* 📖 **Documentation & Language Reference**: Comprehensive guides and examples.
* 🌐 **Official Website**: [nikl.nekonik.org](https://nikl.nekonik.org) (Work in Progress).
* 🛠 **Package Manager**: `nikl-pkg` for managing dependencies and modules.
* 🧪 **Standard Library**: Core utilities for common tasks.

---

## 🛡 Contributing

We welcome contributions of all kinds—bug fixes, feature implementations, documentation improvements, and more! If you'd like to get involved:

1. Fork the repo and create a branch for your feature or fix.
2. Follow the [Contributing Guidelines](https://github.com/Neko-Nik-Org/NIKL-Core/blob/main/CONTRIBUTING.md).
3. Submit a Pull Request (PR) for review.

Before contributing, please make sure to read the [Code of Conduct](https://github.com/Neko-Nik-Org/NIKL-Core/blob/main/CODE_OF_CONDUCT.md).

---

## 📜 Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](https://github.com/Neko-Nik-Org/NIKL-Core/blob/main/CODE_OF_CONDUCT.md).
Everyone interacting in the project’s codebase, issue trackers, discussion forums, and other spaces is expected to follow these guidelines to foster a safe, inclusive, and respectful environment.

---

## 📜 Contributing Guidelines

To maintain a clean and collaborative development experience, contributors are encouraged to follow the [Contributing Guidelines](https://github.com/Neko-Nik-Org/NIKL-Core/blob/main/CONTRIBUTING.md), which include:

* Branching and commit conventions
* PR and issue templates
* Code review expectations
* Testing and formatting rules

---

## 🧑‍💻 Code Owners

Code owners are responsible for reviewing and approving changes before they are merged into the main branch. The list of maintainers and reviewers is defined in the [`CODEOWNERS`](https://github.com/Neko-Nik-Org/NIKL-Core/blob/main/CODEOWNERS) file.

---

## 🔐 Security

Found a security issue or vulnerability? Please **do not** open a public issue. Instead, follow our [Security Policy](https://github.com/Neko-Nik-Org/NIKL-Core/blob/main/SECURITY.md) to report it responsibly. We'll work with you to investigate, triage, and address the problem.

---

## 💬 Support

Need help using NIKL or have a general question?

* Check out the [SUPPORT.md](https://github.com/Neko-Nik-Org/NIKL-Core/blob/main/SUPPORT.md) for guidelines.
* Join discussions on the [Neko Nik Forums](https://forums.nekonik.com).
* Contact via [nekonik.com](https://www.nekonik.com/impressum) for inquiries. Or by email at [nikhil@nekonik.com](mailto:nikhil@nekonik.com).
* For urgent issues, please open an issue on GitHub.
* For feature requests, please open a new issue on GitHub.

---

## 🐞 Bug Reporting

If you encounter a bug, please open an issue on the [GitHub Issues page](https://github.com/Neko-Nik-Org/NIKL-Core/issues) and include:

* A clear title and description
* Steps to reproduce
* Your environment (OS, version, etc.)
* Screenshots or logs if available

This helps us fix issues faster and keep the language stable for everyone.

---

## 🙏🏻 Acknowledgments

Special thanks to the open-source community, especially the Rust ecosystem, for inspiration and guidance.
Gratitude to everyone who has contributed code, ideas, feedback, and encouragement to the NIKL project. You make this work possible. ❤️

---

## 🌐 Official Domains

The following domains and subdomains are officially owned and maintained by the author (Neko Nik):

* [nikl.nekonik.org](https://nikl.nekonik.org) - **Official NIKL website**
* [VS Code Extension](https://marketplace.visualstudio.com/items?itemName=Neko-Nik.nikl-language-support) - **NIKL Language Support for Visual Studio Code**
* [nikl-pkg.nekonik.org](https://nikl-pkg.nekonik.org) - **NIKL Package Manager** (Not yet live/implemented)
* [api.nikl-pkg.nekonik.org](https://api.nikl-pkg.nekonik.org) - **NIKL Package Manager API** (Not yet live/implemented)
* [Neko Nik Discord Server](https://discord.gg/PYqHVUGdwv) - **Neko Nik Discord Community**
* [forums.nekonik.com](https://forums.nekonik.com) - **Neko Nik Forums**


Below are the official links to the author's personal website and social media:

* [nekonik.com](https://www.nekonik.com) - **Author's Website**
* [status.nekonik.com](https://status.nekonik.com) - **Author's Status Page**
* [github.com/Neko-Nik-Org](https://github.com/Neko-Nik-Org) - **Author's GitHub**
* [orcid.org/0009-0008-3435-5674](https://orcid.org/0009-0008-3435-5674) - **Author's ORCID** (Open Researcher and Contributor ID)
* [linkedin.com/in/neko-nik](https://www.linkedin.com/in/neko-nik) - **Author's LinkedIn**


## 📨 Stay Updated

**Join the newsletter** and get instant updates on releases, features, and community announcements:

👉🏻 [https://mailer.nekonik.com/subscription/form](https://mailer.nekonik.com/subscription/form)


## 👤 Author

Made with ❤️ by [Neko Nik](https://www.nekonik.com/impressum)

> Join the discussion on the [Neko Nik Forums](https://forums.nekonik.com)

## 🪪 License

NIKL-Core is licensed under the [MIT License](https://github.com/Neko-Nik-Org/NIKL-Core/blob/main/LICENSE) — use it freely, with attribution. No warranties, no liability.

> ⚠️ **Disclaimer**:

> • All domains under *.nekonik.com and *.nekonik.org, including but not limited to nikl.nekonik.org, are owned and managed by the author, Nikhil ("Neko Nik"). These are the only official sources for NIKL-related materials, documentation, and announcements.

> • You are responsible for any code written or executed using NIKL.

> • Modifications to the interpreter are at your own risk.

> • This is a personal project, not affiliated with or endorsed by the Rust Foundation or any other organization.

> • This is a work in progress — features may change as development continues.

> • Use at your own risk. No warranties or guarantees are provided. By using NIKL, you agree to these terms.

> • For more details, see the [License](https://github.com/Neko-Nik-Org/NIKL-Core/blob/main/LICENSE) file.
