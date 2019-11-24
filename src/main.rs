const QUARTER_VALUE: u32 = 25;
const DIME_VALUE: u32 = 10;
const NICKEL_VALUE: u32 = 5;
const PENNY_VALUE: u32 = 1;

fn main() {
    let coins = 1692;
    let money = 10054;
    print_inputs(coins, money);
    let quarters = 0;
    let dimes = 0;
    let nickels = 0;
    let pennies = 0;
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
    let calculated_coins = quarters + dimes + nickels + pennies;
    let calculated_money = sum_of_denominations(quarters, dimes, nickels, pennies);
    println!();
    print_check(expected_coins, expected_money, calculated_coins, calculated_money);
}

fn print_check(expected_coins: u32, expected_money: u32,
               calculated_coins: u32, calculated_money: u32) {
    println!(
        concat!(
            "CHECK\n",
            "Coins: {} ({})\n",
            "Money: {} ({})"
        ),
        calculated_coins, format_success(expected_coins == calculated_coins),
        format_money(calculated_money), format_success(expected_money == calculated_money)
    );
}

fn sum_of_denominations(quarters: u32, dimes: u32, nickels: u32, pennies: u32) -> u32 {
    quarters * QUARTER_VALUE +
        dimes * DIME_VALUE +
        nickels * NICKEL_VALUE +
        pennies * PENNY_VALUE
}

fn format_money(money: u32) -> String {
    let dollars = money / 100;
    let cents = money % 100;
    format!("${}.{:02}", dollars, cents)
}

fn format_success(success: bool) -> String {
    if success { "✓" } else { "✗" }.to_string()
}
