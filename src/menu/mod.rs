use crate::bank::Bank;
use crate::io::clear_screen;
use crate::io::get_input_f64;
use crate::io::get_input_string;
use crate::io::get_input_u32;
use crate::io::press_enter_to_continue;

pub fn create_account_menu(bank: &mut Bank) {
    clear_screen();
    println!("------ Create a New Account -------");
    let owner_name = get_input_string("Enter the name of the owner");
    if owner_name.is_empty() {
        println!("Owner name cannot be empty");
        press_enter_to_continue();
        return;
    }
    bank.create_account(owner_name);
    press_enter_to_continue();
}

pub fn deposit_menu(bank: &mut Bank) {
    clear_screen();
    println!("------ Deposit Funds ------");
    let account_num = match get_input_u32("Enter the account number: ") {
        Some(acc_num) => acc_num,
        None => {
            println!("Account number is invalid");
            press_enter_to_continue();
            return;
        }
    };
    let amount = match get_input_f64("Enter amount: ") {
        Some(amt) => amt,
        None => {
            println!("Invalid amount");
            press_enter_to_continue();
            return;
        }
    };

    match bank.deposit(account_num, amount) {
        Ok(new_balance) => println!(
            "Sucessfully deposited ${:.2} into account {}. New Balance: {:.2}",
            amount, account_num, new_balance
        ),
        Err(err) => eprintln!("Error in deposit operation: {}", err),
    }
    press_enter_to_continue();
}

pub fn transfer_menu(bank: &mut Bank) {
    clear_screen();
    println!("------ Transfer Funds ------");

    let from_account_num = match get_input_u32("Enter the source account number: ") {
        Some(acc_num) => acc_num,
        None => {
            println!("Invalid source account number");
            press_enter_to_continue();
            return;
        }
    };

    let to_accout_num = match get_input_u32("Enter destination account number: ") {
        Some(acc_num) => acc_num,
        None => {
            println!("Invalid destination account number");
            press_enter_to_continue();
            return;
        }
    };

    let amount = match get_input_f64("Enter amount to transfer: ") {
        Some(amt) => amt,
        None => {
            println!("Invalid amount");
            press_enter_to_continue();
            return;
        }
    };

    match bank.transfer(from_account_num, to_accout_num, amount) {
        Ok(_) => println!(
            "Successfully transferred ${:.2} from account {} to account {}",
            amount, from_account_num, to_accout_num
        ),
        Err(err) => eprintln!("Error in transfer operation: {}", err),
    }
    press_enter_to_continue();
}

pub fn check_balance_menu(bank: &mut Bank) {
    clear_screen();
    println!("------ Check Account Balance ------");
    let account_num = match get_input_u32("Enter Account number") {
        Some(acc_num) => acc_num,
        None => {
            println!("Invalid account number");
            press_enter_to_continue();
            return;
        }
    };
    bank.check_balance(account_num);
    press_enter_to_continue();
}

pub fn withdraw_menu(bank: &mut Bank) {
    clear_screen();
    println!("------ Withdraw Funds ------");
    let account_num = match get_input_u32("Enter Account number: ") {
        Some(acc_num) => acc_num,
        None => {
            println!("Invalid account number");
            press_enter_to_continue();
            return;
        }
    };
    let amount = match get_input_f64("Enter amount to withdraw: ") {
        Some(amt) => amt,
        None => {
            println!("Invalid amount");
            press_enter_to_continue();
            return;
        }
    };
    match bank.withdraw(account_num, amount) {
        Ok(new_balance) => println!(
            "Sucessfully withdrawn ${:.2} from account {}. New Balance: ${:.2}",
            amount, account_num, new_balance
        ),
        Err(err) => eprintln!("Error: {}", err),
    }
    press_enter_to_continue();
}

pub fn list_all_accounts_menu(bank: &Bank) {
    clear_screen();
    println!("--- List All Accounts ---");
    bank.list_all_accounts();
    press_enter_to_continue();
}
