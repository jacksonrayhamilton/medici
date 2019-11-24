pub fn print_inputs(coins: u32, money: u32) {
    println!(concat!(
        "INPUT\n",
        "Coins: {}\n",
        "Money: {}"
    ), coins, format_money(money));
}

pub fn print_result(quarters: u32, dimes: u32, nickels: u32, pennies: u32) {
    println!(concat!(
        "RESULT\n",
        "Quarters: {}\n",
        "Dimes: {}\n",
        "Nickels: {}\n",
        "Pennies: {}"
    ), quarters, dimes, nickels, pennies);
}

pub fn print_check(calculated_coins: u32, coins_correct: bool,
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
