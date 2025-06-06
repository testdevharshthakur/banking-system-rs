use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub account_number: u32,
    pub owner_name: String,
    pub balance: f64,
}

impl Account {
    pub fn new(account_number: u32, owner_name: String) -> Self {
        Account {
            account_number: account_number,
            owner_name: owner_name,
            balance: 0.0,
        }
    }

    pub fn deposit(&mut self, amount: f64) -> Result<f64, String> {
        if amount < 0.0 {
            return Err("Deposit amount must be positive".to_string());
        }
        self.balance += amount;
        Ok(self.balance)
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<f64, String> {
        if amount < 0.0 {
            return Err("Withdrawel amount has to be positve".to_string());
        }
        self.balance -= amount;
        Ok(self.balance)
    }

    pub fn get_balance(&mut self) -> f64 {
        self.balance
    }

    pub fn display_info(&self) {
        println!("  Account Number: {}", self.account_number);
        println!("  Owner Name: {}", self.owner_name);
        println!("  Balance: {}", self.balance);
    }
}
