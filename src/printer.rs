#[cfg(test)]
pub fn format_all(expected_coins: u32, expected_money: u32,
                  quarters: u32, dimes: u32, nickels: u32, pennies: u32,
                  calculated_coins: u32, coins_correct: bool,
                  calculated_money: u32, money_correct: bool) -> String {
    format!(
        concat!(
            "{}\n\n",
            "{}\n\n",
            "{}"
        ),
        format_inputs(expected_coins, expected_money),
        format_result(quarters, dimes, nickels, pennies),
        format_check(calculated_coins, coins_correct,
                     calculated_money, money_correct)
    )
}

pub fn print_inputs(coins: u32, money: u32) {
    println!("{}", format_inputs(coins, money));
}

fn format_inputs(coins: u32, money: u32) -> String {
    format!(concat!(
        "INPUT\n",
        "Coins: {}\n",
        "Money: {}"
    ), coins, format_money(money))
}

pub fn print_result(quarters: u32, dimes: u32, nickels: u32, pennies: u32) {
    println!("{}", format_result(quarters, dimes, nickels, pennies));
}

fn format_result(quarters: u32, dimes: u32, nickels: u32, pennies: u32) -> String {
    format!(concat!(
        "RESULT\n",
        "Quarters: {}\n",
        "Dimes: {}\n",
        "Nickels: {}\n",
        "Pennies: {}"
    ), quarters, dimes, nickels, pennies)
}

pub fn print_contest_result(quarters: u32, dimes: u32, nickels: u32, pennies: u32) {
    println!("\0{{\"quarters\":{},\"dimes\":{},\"nickels\":{},\"pennies\":{}}}",
             quarters, dimes, nickels, pennies);
}

pub fn print_check(calculated_coins: u32, coins_correct: bool,
                   calculated_money: u32, money_correct: bool) {
    println!("{}", format_check(calculated_coins, coins_correct,
                                calculated_money, money_correct));
}

fn format_check(calculated_coins: u32, coins_correct: bool,
                calculated_money: u32, money_correct: bool) -> String {
    format!(
        concat!(
            "CHECK\n",
            "Coins: {} ({})\n",
            "Money: {} ({})"
        ),
        calculated_coins, format_success(coins_correct),
        format_money(calculated_money), format_success(money_correct)
    )
}

fn format_money(money: u32) -> String {
    let dollars = money / 100;
    let cents = money % 100;
    format!("${}.{:02}", dollars, cents)
}

fn format_success(success: bool) -> String {
    if success { "✓" } else { "✗" }.to_string()
}
