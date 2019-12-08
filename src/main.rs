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
    printer::print_result(quarters, dimes, nickels, pennies);
    (quarters, dimes, nickels, pennies)
}
