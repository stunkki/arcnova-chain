# Contributing

Thanks for your interest in contributing!  
This project is an educational blockchain implementation written in Rust, and contributions of all kinds are welcome.

---

## 🧭 Ways You Can Contribute

### Code Contributions
- Add new blockchain features (see ROADMAP in the README)
- Fix bugs or improve existing modules
- Write tests or benchmarks
- Improve documentation or comments
- Refactor or simplify complex logic

### Non-Code Contributions
- Report issues and suggest features
- Improve documentation, diagrams, or examples
- Help review PRs
- Write tutorials or examples for beginners

---

## 📦 Project Structure

src/
├─ block.rs
├─ blockchain.rs
├─ merkle.rs
├─ transaction.rs
├─ wallet.rs
├─ lib.rs
└─ main.rs (optional example)

yaml
Copy code

---

## 🛠 Development Setup

1. Install Rust (nightly not required):  
   https://www.rust-lang.org/tools/install

2. Clone the repository:

   ```bash
   git clone https://github.com/yourname/blockchain-core
   cd blockchain-core
Run the example:

bash
Copy code
cargo run
Run tests (if you add them):

bash
Copy code
cargo test
🌿 Branching Model
main → stable, always working

dev → active development (optional)

feature branches should follow:

cpp
Copy code
feature/<short-name>
fix/<short-name>
docs/<short-name>
Example:

bash
Copy code
feature/utxo-model
docs/p2p-diagram
🔀 Pull Requests
Before submitting a PR, please:

Keep code clear and idiomatic (rustfmt/clippy recommended).

Write comments for complex logic.

Add tests when possible.

Update docs if you introduce new features.

Keep commits focused — small PRs are easier to review.

🐞 Reporting Issues
Please include:

What happened?

Steps to reproduce

Your OS + Rust version

Expected behavior

Screenshots/log output (if useful)

✨ Style Guide
Files are lowercase (architecture.md, changelog.md, contributing.md)

README.md uses uppercase by convention

Rust code follows standard Rust style

Run formatters:

bash
Copy code
cargo fmt
cargo clippy
🤝 Community Standards
Be respectful, constructive, and kind.
We follow the “assume good intent” rule: this is a learning-friendly project.

📜 License

By contributing, you agree that your contributions will be licensed under the MIT License included in this repository.