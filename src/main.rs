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
    print_result(quarters, dimes, nickels, pennies);
}

fn print_inputs(coins: u32, money: u32) {
    println!(concat!(
        "INPUT\n",
        "Coins: {}\n",
        "Money: {}"
    ), coins, format_money(money));
}

fn print_result(quarters: u32, dimes: u32, nickels: u32, pennies: u32) {
    let coins = quarters + dimes + nickels + pennies;
    let money = sum_of_denominations(quarters, dimes, nickels, pennies);
    println!(concat!(
        "\n",
        "RESULT\n",
        "Quarters: {}\n",
        "Dimes: {}\n",
        "Nickels: {}\n",
        "Pennies: {}\n",
        "\n",
        "CHECK\n",
        "Coins: {}\n",
        "Money: {}"
    ), quarters, dimes, nickels, pennies, coins, format_money(money));
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
