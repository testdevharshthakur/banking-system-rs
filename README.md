# 🏦 Rust Terminal Banking System

A simple, terminal-based banking application built with Rust. This project demonstrates basic Rust programming concepts, modular design, and data persistence using JSON.

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
banking_app/
├── src/
│   ├── main.rs         # Main application loop and menu handling
│   ├── account.rs      # Defines the Account struct and its core logic (deposit, withdraw)
│   ├── bank.rs         # Manages a collection of accounts and handles bank-level operations (create, transfer, save/load)
│   └── io_utils.rs     # Utility functions for terminal input/output (clear screen, read input)
├── data/               # Directory for persistent data
│   └── accounts.json   # (Generated) Stores account data in JSON format
├── .gitignore          # Specifies files and directories to be ignored by Git
├── Cargo.toml          # Rust project manifest and dependencies
└── README.md           # Project description and instructions
```

## 🚀 How to Run

1. **Clone the repository (or create the project as described):**
   ```bash
   git clone https://github.com/your-username/banking_app.git
   cd banking_app
   ```
   (If you created it manually, just navigate to the `banking_app` directory.)

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
- Enter numbers `1-6` for banking operations.
- Enter `7` to exit the application.

Your account data will be automatically saved in the `data/accounts.json` file.

## 📦 Dependencies

- `serde` = "1.0" (with `derive` feature)
- `serde_json` = "1.0"

These are automatically handled by `Cargo.toml` when you build the project.

## 📄 License

This project is open-source and available under the [MIT License](LICENSE).

