use crate::money;
use crate::money::{NICKEL_VALUE, PENNY_VALUE};

pub fn calculate_result(coins: u32, money: u32) -> (u32, u32, u32, u32) {
    let mut quarters = coins; // Start with the max money value and work down.
    let mut dimes = 0;
    let mut nickels = 0;
    let mut pennies = 0;
    // Dime phase:
    while quarters > 0 &&
        money::sum_of_denominations(quarters, dimes, nickels, pennies) > money {
            quarters -= 1;
            dimes += 1;
        }
    // Nickel phase:
    while dimes > 0 &&
        money::sum_of_denominations(quarters, dimes, nickels, pennies) > money {
            dimes -= 1;
            nickels += 1;
        }
    // Penny phase:
    while nickels > 0 &&
        money::sum_of_denominations(quarters, dimes, nickels, pennies) > money {
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
    while nickels > 0 && money::sum_of_denominations(quarters, dimes, nickels, pennies) < money {
        nickels -= 1;
        dimes += 1;
    }
    while dimes > 0 && money::sum_of_denominations(quarters, dimes, nickels, pennies) < money {
        dimes -= 1;
        quarters += 1;
    }
    // Did we go over the desired value?  Convert back down:
    while dimes > 0 && money::sum_of_denominations(quarters, dimes, nickels, pennies) > money {
        dimes -= 1;
        nickels += 1;
    }
    (quarters, dimes, nickels, pennies)
}

pub fn check_result(expected_coins: u32, expected_money: u32,
                    quarters: u32, dimes: u32, nickels: u32, pennies: u32)
                    -> (u32, bool, u32, bool) {
    let calculated_coins = quarters + dimes + nickels + pennies;
    let calculated_money = money::sum_of_denominations(quarters, dimes, nickels, pennies);
    (calculated_coins, expected_coins == calculated_coins,
     calculated_money, expected_money == calculated_money)
}
