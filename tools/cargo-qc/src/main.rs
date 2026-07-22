use colored::*;
use std::env;
use std::process::Command;

fn main() {
    println!("{}", "🚀 Running Local Quality Control (cargo qc)".bold().cyan());

    // Ensure we are running from a cargo directory
    let current_dir = env::current_dir().expect("Failed to get current directory");
    println!("📍 Directory: {}\n", current_dir.display());

    // 1. Formatting
    println!("{}", "➔ Running cargo fmt...".yellow());
    let fmt_status = Command::new("cargo")
        .arg("fmt")
        .arg("--")
        .arg("--check")
        .status()
        .expect("Failed to execute cargo fmt");

    if !fmt_status.success() {
        eprintln!("{}", "❌ Formatting failed. Run `cargo fmt` to fix it.".red().bold());
        std::process::exit(1);
    }
    println!("{}", "✅ Formatting passed!".green());

    // 2. Clippy
    println!("\n{}", "➔ Running cargo clippy...".yellow());
    let clippy_status = Command::new("cargo")
        .arg("clippy")
        .arg("--")
        .arg("-D")
        .arg("warnings")
        .status()
        .expect("Failed to execute cargo clippy");

    if !clippy_status.success() {
        eprintln!("{}", "❌ Clippy failed. Please fix the warnings above.".red().bold());
        std::process::exit(1);
    }
    println!("{}", "✅ Clippy passed!".green());

    // 3. Build
    println!("\n{}", "➔ Running cargo build...".yellow());
    let build_status =
        Command::new("cargo").arg("build").status().expect("Failed to execute cargo build");

    if !build_status.success() {
        eprintln!("{}", "❌ Build failed. Please fix the compiler errors.".red().bold());
        std::process::exit(1);
    }
    println!("{}", "✅ Build passed!".green());

    println!("\n{}", "🎉 All checks passed successfully! Your code is solid.".bold().green());
}
