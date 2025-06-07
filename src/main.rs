mod bank;
mod io;
mod menu;

use bank::Bank;
use io::clear_screen;
use io::get_input_string;
use io::press_enter_to_continue;
use menu::check_balance_menu;
use menu::create_account_menu;
use menu::deposit_menu;
use menu::list_all_accounts_menu;
use menu::transfer_menu;
use menu::withdraw_menu;

fn main() {
    let mut bank = Bank::default();
    bank.load_accounts();

    loop {
        clear_screen();
        println!("------------------------------------");
        println!("  Rust Terminal Banking System");
        println!("------------------------------------");
        println!("1. Create New Account");
        println!("2. Deposit Funds");
        println!("3. Withdraw Funds");
        println!("4. Transfer Funds");
        println!("5. Check Account Balance");
        println!("6. List All Accounts");
        println!("7. Exit");
        println!("------------------------------------");

        let choice_str = get_input_string("Enter your choice: ");
        let choice: Option<u32> = choice_str.parse().ok();

        match choice {
            Some(1) => create_account_menu(&mut bank),
            Some(2) => deposit_menu(&mut bank),
            Some(3) => withdraw_menu(&mut bank),
            Some(4) => transfer_menu(&mut bank),
            Some(5) => check_balance_menu(&mut bank),
            Some(6) => list_all_accounts_menu(&bank),
            Some(7) => {
                println!("Exiting banking system. Goodbye!");
                break;
            }
            _ => {
                println!("\nInvalid choice. Please enter a number between 1 and 7.");
                press_enter_to_continue();
            }
        }
    }
}
