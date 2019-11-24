mod money;
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
    print_inputs(coins, money);
    let (quarters, dimes, nickels, pennies) = solver::calculate_result(coins, money);
    println!();
    print_result(quarters, dimes, nickels, pennies);
    check_result(coins, money, quarters, dimes, nickels, pennies);
}

fn print_inputs(coins: u32, money: u32) {
    println!(concat!(
        "INPUT\n",
        "Coins: {}\n",
        "Money: {}"
    ), coins, format_money(money));
}

fn print_result(quarters: u32, dimes: u32, nickels: u32, pennies: u32) {
    println!(concat!(
        "RESULT\n",
        "Quarters: {}\n",
        "Dimes: {}\n",
        "Nickels: {}\n",
        "Pennies: {}"
    ), quarters, dimes, nickels, pennies);
}

fn check_result(expected_coins: u32, expected_money: u32,
                quarters: u32, dimes: u32, nickels: u32, pennies: u32) {
    let (calculated_coins, coins_correct,
         calculated_money, money_correct) =
        solver::check_result(expected_coins, expected_money,
                             quarters, dimes, nickels, pennies);
    println!();
    print_check(calculated_coins, coins_correct, calculated_money, money_correct);
}

fn print_check(calculated_coins: u32, coins_correct: bool,
               calculated_money: u32, money_correct: bool) {
    println!(
        concat!(
            "CHECK\n",
            "Coins: {} ({})\n",
            "Money: {} ({})"
        ),
        calculated_coins, format_success(coins_correct),
        format_money(calculated_money), format_success(money_correct)
    );
}

fn format_money(money: u32) -> String {
    let dollars = money / 100;
    let cents = money % 100;
    format!("${}.{:02}", dollars, cents)
}

fn format_success(success: bool) -> String {
    if success { "✓" } else { "✗" }.to_string()
}
