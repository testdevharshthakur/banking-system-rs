# 🏦 Rust Terminal Banking System

A simple, terminal-based banking application built with Rust. This project demonstrates basic Rust programming concepts, modular design (with submodules), and data persistence using JSON.

## ✨ Features

- Create new bank accounts.
- Deposit funds into an account.
- Withdraw funds from an account.
- Transfer funds between accounts.
- Check the balance of a specific account.
- List all registered accounts.
- Data persistence: Account information is saved to a JSON file (`data/accounts.json`) and loaded on startup.

## 📁 Project Structure

```
banking-system-rs/
├── src/
│   ├── main.rs           # Main application loop and menu handling
│   ├── bank/
│   │   ├── mod.rs        # Bank logic and account management
│   │   └── account/
│   │       └── mod.rs    # Account struct and core logic (deposit, withdraw)
│   ├── io/
│   │   └── mod.rs        # Terminal input/output utilities
│   └── menu/
│       └── mod.rs        # Menu handling and user interaction
├── data/
│   └── .gitkeep          # Placeholder; `accounts.json` is generated at runtime
├── .gitignore            # Specifies files and directories to be ignored by Git
├── Cargo.toml            # Rust project manifest and dependencies
├── Cargo.lock            # Cargo lockfile
├── LICENSE               # Project license (MIT)
└── README.md             # Project description and instructions
```

## 🚀 How to Run

1. **Clone the repository:**
   ```bash
   git clone https://github.com/testdevharshthakur/banking-system-rs.git
   cd banking-system-rs
   ```

2. **Build the project:**
   ```bash
   cargo build
   ```

3. **Run the application:**
   ```bash
   cargo run
   ```

## 📝 Usage

Follow the on-screen menu instructions:
- Enter numbers `1-7` for banking operations.
- Enter `7` to exit the application.

Your account data will be automatically saved in the `data/accounts.json` file (created at runtime).

## 📦 Dependencies

- `serde` = "1.0" (with `derive` feature)
- `serde_json` = "1.0"
- `chrono` = "0.4" (with `serde` feature)
- `uuid` = "1.17" (with `v4` and `serde` features)

These are automatically handled by `Cargo.toml` when you build the project.

## 📄 License

This project is open-source and available under the [MIT License](LICENSE).

