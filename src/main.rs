mod args;
mod money;
mod printer;
mod solver;

use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let (coins, money) = args::parse_args(line?);
        calculate_result(coins, money);
    }
    Ok(())
}

fn calculate_result(coins: u32, money: u32) -> (u32, u32, u32, u32) {
    let (quarters, dimes, nickels, pennies) = solver::calculate_result(coins, money);
    printer::print_contest_result(quarters, dimes, nickels, pennies);
    (quarters, dimes, nickels, pennies)
}

fn check_result(expected_coins: u32, expected_money: u32,
                quarters: u32, dimes: u32, nickels: u32, pennies: u32) {
    let (calculated_coins, coins_correct,
         calculated_money, money_correct) =
        solver::check_result(expected_coins, expected_money,
                             quarters, dimes, nickels, pennies);
    println!();
    printer::print_check(calculated_coins, coins_correct, calculated_money, money_correct);
}
