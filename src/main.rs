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
    let dollars = money / 100;
    let cents = money % 100;
    println!(concat!(
        "Coins: {}\n",
        "Money: ${}.{}"
    ), coins, dollars, cents);
}

fn print_result(quarters: u32, dimes: u32, nickels: u32, pennies: u32) {
    println!(concat!(
        "Quarters: {}\n",
        "Dimes: {}\n",
        "Nickels: {}\n",
        "Pennies: {}"
    ), quarters, dimes, nickels, pennies);
}
