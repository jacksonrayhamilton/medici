mod money;
mod printer;
mod solver;

fn main() {
    // let coins = 1692;
    // let money = 10054;
    // let coins = 440;
    // let money = 4510;
    // let coins = 513;
    // let money = 1712;
    let coins = 100;
    let money = 1200;
    printer::print_inputs(coins, money);
    let (quarters, dimes, nickels, pennies) = solver::calculate_result(coins, money);
    println!();
    printer::print_result(quarters, dimes, nickels, pennies);
    check_result(coins, money, quarters, dimes, nickels, pennies);
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
