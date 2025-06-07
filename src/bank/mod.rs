use crate::bank::account::Account;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

mod account;

const DATA_FILE: &str = "data/accounts.json";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Bank {
    accounts: HashMap<u32, Account>,
    next_account_num: u32,
}

impl Bank {
    fn generate_account_num(&mut self) -> u32 {
        let mut num = self.next_account_num;
        // If num > u32::Max then reset
        while self.accounts.contains_key(&num) || num == 0 {
            num += 1;
            if num == u32::MAX {
                eprintln!("Warning: Account number range exhausted. Resetting.");
                num = 1;
            }
        }
        self.next_account_num = num + 1; // setting the next account number
        num
    }
    /// Create a new account with a given owner name.
    pub fn create_account(&mut self, owner_name: String) -> Option<u32> {
        let account_number = self.generate_account_num();
        let account = Account::new(account_number, owner_name.clone());
        self.accounts.insert(account_number, account);
        println!(
            "Successfully created an account for {}. Account number: {}",
            owner_name, account_number
        );
        self.save_accounts();
        Some(account_number)
    }
    /// Loads the account data from data file
    pub fn load_accounts(&mut self) {
        let path = Path::new(DATA_FILE);

        if !path.exists() {
            eprintln!("Data file not found. Starting with empty bank");
            return;
        }

        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(err) => {
                eprintln!("Failed to read data from the file: {}", err);
                return;
            }
        };

        match serde_json::from_str(&content) {
            Ok(bank_data) => {
                let loaded_bank: Bank = bank_data;
                self.accounts = loaded_bank.accounts;
                self.next_account_num =
                    self.accounts.keys().max().map_or(1, |&max_num| max_num + 1);
                println!("Accounts Loaded successfully from {}", DATA_FILE);
            }
            Err(err) => {
                eprintln!("Failed to parse account data from JSON:{}", err);
            }
        }
    }
    /// Saves current account data to the DATA_FILE
    pub fn save_accounts(&self) {
        let path = Path::new(DATA_FILE);
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                if let Err(err) = fs::create_dir_all(parent) {
                    eprintln!("Failed to create data directory: {}", err);
                    return;
                }
            }
        }

        match serde_json::to_string_pretty(&self) {
            Ok(json_string) => {
                if let Err(err) = fs::write(path, json_string) {
                    eprintln!("Failed to write data to file {}", err);
                } else {
                    println!("Account Saved Successfully to {}", DATA_FILE);
                }
            }
            Err(err) => {
                eprintln!("Failed to serialize accounts to JSON {}", err);
            }
        }
    }
    /// Checks and displays the current balance of a particular account
    pub fn check_balance(&self, account_num: u32) {
        match self.accounts.get(&account_num) {
            Some(account) => {
                let current_balance = account.balance;
                println!("Current Balance of {}: {}", account_num, current_balance);
            }
            None => {
                eprintln!("Error: Account {} not found", account_num);
            }
        }
    }
    /// Checks and display detials of all the accounts currently in the bank
    pub fn list_all_accounts(&self) {
        if self.accounts.is_empty() {
            println!("No Accounts registerd yet.")
        } else {
            println!("\n--- All Accounts ---");
            for account in self.accounts.values() {
                account.display_info();
                println!("------------------")
            }
        }
    }
    /// Withdarws money from a specefic account.
    pub fn withdraw(&mut self, account_num: u32, amount: f64) -> Result<f64, String> {
        match self.accounts.get_mut(&account_num) {
            Some(account) => {
                let result = account.withdraw(amount);
                if result.is_ok() {
                    self.save_accounts();
                }
                result
            }
            None => Err(format!("Account {} not found.", account_num)),
        }
    }
    /// Deposits a specific amount of money to a particular account
    pub fn deposit(&mut self, account_num: u32, amount: f64) -> Result<f64, String> {
        match self.accounts.get_mut(&account_num) {
            Some(account) => {
                let result = account.deposit(amount);
                if result.is_ok() {
                    self.save_accounts();
                }
                result
            }
            None => Err(format!("Account {} not found.", account_num)),
        }
    }
    /// Checks and then transfers a specefic amount from one account to another
    pub fn transfer(
        &mut self,
        from_account_num: u32,
        to_account_num: u32,
        amount: f64,
    ) -> Result<(), String> {
        if from_account_num == to_account_num {
            return Err("Cannot transfer funds to the same account".to_string());
        }

        if amount <= 0.0 {
            return Err("Transfer amount must be positive".to_string());
        }

        let from_account = match self.accounts.get_mut(&to_account_num) {
            Some(account) => account,
            None => return Err(format!("Account {} not found", from_account_num)),
        };
        from_account.balance -= amount;

        let to_account = match self.accounts.get_mut(&to_account_num) {
            Some(account) => account,
            None => return Err(format!("Account {} not found", to_account_num)),
        };
        to_account.balance += amount;

        self.save_accounts();
        Ok(())
    }
}
