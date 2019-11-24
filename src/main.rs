const QUARTER_VALUE: u32 = 25;
const DIME_VALUE: u32 = 10;
const NICKEL_VALUE: u32 = 5;
const PENNY_VALUE: u32 = 1;

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
    let (quarters, dimes, nickels, pennies) = calculate_result(coins, money);
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

fn calculate_result(coins: u32, money: u32) -> (u32, u32, u32, u32) {
    let mut quarters = coins; // Start with the max money value and work down.
    let mut dimes = 0;
    let mut nickels = 0;
    let mut pennies = 0;
    // Dime phase:
    while quarters > 0 &&
        sum_of_denominations(quarters, dimes, nickels, pennies) > money {
            quarters -= 1;
            dimes += 1;
        }
    // Nickel phase:
    while dimes > 0 &&
        sum_of_denominations(quarters, dimes, nickels, pennies) > money {
            dimes -= 1;
            nickels += 1;
        }
    // Penny phase:
    while nickels > 0 &&
        sum_of_denominations(quarters, dimes, nickels, pennies) > money {
            nickels -= 1;
            pennies += 1;
        }
    // Ensure a valid penny value:
    while nickels > 0 && ((pennies * PENNY_VALUE) % NICKEL_VALUE) != (money % NICKEL_VALUE) {
        nickels -= 1;
        pennies += 1;
    }
    while dimes > 0 && ((pennies * PENNY_VALUE) % NICKEL_VALUE) != (money % NICKEL_VALUE) {
        dimes -= 1;
        pennies += 1;
    }
    while quarters > 0 && ((pennies * PENNY_VALUE) % NICKEL_VALUE) != (money % NICKEL_VALUE) {
        quarters -= 1;
        pennies += 1;
    }
    // Increase the money back to the desired value:
    while nickels > 0 && sum_of_denominations(quarters, dimes, nickels, pennies) < money {
        nickels -= 1;
        dimes += 1;
    }
    while dimes > 0 && sum_of_denominations(quarters, dimes, nickels, pennies) < money {
        dimes -= 1;
        quarters += 1;
    }
    // Did we go over the desired value?  Convert back down:
    while dimes > 0 && sum_of_denominations(quarters, dimes, nickels, pennies) > money {
        dimes -= 1;
        nickels += 1;
    }
    (quarters, dimes, nickels, pennies)
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
