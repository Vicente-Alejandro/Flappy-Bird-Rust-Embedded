# 🛠️ cargo-qc (Quality Control)

A fast, standalone Rust CLI tool that acts as a local Continuous Integration (CI) microservice. 
`cargo-qc` ensures your Rust projects maintain the highest code quality standards by automatically running formatting checks, linters, and compilation tests.

## 🚀 Features

When executed, `cargo-qc` sequentially runs:
1. **Formatter:** `cargo fmt --check` (Ensures standard code styling)
2. **Linter:** `cargo clippy -- -D warnings` (Catches bugs, unidiomatic code, and performance issues)
3. **Compiler:** `cargo build` (Ensures the codebase compiles without errors)

If any step fails, the tool halts and provides immediate feedback. If all steps pass, you get a clean bill of health.

## 📦 Installation

Since this tool is built as a Cargo subcommand, you can install it globally on your machine.

From the root of this repository, run:
```bash
cargo install --path tools/cargo-qc
```

## 🎮 Usage

Once installed, you can use `cargo-qc` in **any** Rust project on your computer!

Simply navigate to your Rust project directory and run:
```bash
cargo qc
```

Cargo automatically intercepts the `qc` subcommand and routes it to this microservice.

### Example Output

```text
🚀 Running Local Quality Control (cargo qc)
📍 Directory: D:\proyectos\RUST\my-awesome-project

➔ Running cargo fmt...
✅ Formatting passed!

➔ Running cargo clippy...
✅ Clippy passed!

➔ Running cargo build...
✅ Build passed!

🎉 All checks passed successfully! Your code is solid.
```

## 🤝 Motivation

This tool was created to provide immediate, local CI feedback without relying on cloud-based providers (like GitHub Actions), saving billing minutes and accelerating the development loop for Rust developers.
