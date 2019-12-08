use std::cmp;
use crate::money;
use crate::money::{QUARTER_VALUE, DIME_VALUE, NICKEL_VALUE, PENNY_VALUE};

pub fn calculate_result(coins: u32, money: u32) -> (u32, u32, u32, u32) {
    // Start with max valid values and work down.
    let mut quarters = cmp::min(money / QUARTER_VALUE, coins);
    let mut dimes = cmp::min(money / DIME_VALUE, coins - quarters);
    let mut nickels = cmp::min(money / NICKEL_VALUE, coins - quarters - dimes);
    let mut pennies = cmp::min(money / PENNY_VALUE, coins - quarters - dimes - nickels);
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
    while (pennies * PENNY_VALUE) % NICKEL_VALUE != money % NICKEL_VALUE {
        if nickels > 0 { nickels -= 1 } else { dimes -= 1 }
        pennies += 1
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

#[cfg(test)]
pub fn check_result(expected_coins: u32, expected_money: u32,
                    quarters: u32, dimes: u32, nickels: u32, pennies: u32)
                    -> (u32, bool, u32, bool) {
    let calculated_coins = quarters + dimes + nickels + pennies;
    let calculated_money = money::sum_of_denominations(quarters, dimes, nickels, pennies);
    (calculated_coins, expected_coins == calculated_coins,
     calculated_money, expected_money == calculated_money)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::printer;

    const PERMS: u32 = 100; // Use `cargo test --release`!

    #[test]
    fn test_permutations() {
        for quarters in 0..PERMS {
            for dimes in 0..PERMS {
                for nickels in 0..PERMS {
                    for pennies in 0..PERMS {
                        let coins = quarters + dimes + nickels + pennies;
                        let money = money::sum_of_denominations(
                            quarters, dimes, nickels, pennies
                        );
                        assert_correct_result(coins, money);
                    }
                }
            }
        }
    }

    fn assert_correct_result(expected_coins: u32, expected_money: u32) {
        let (quarters, dimes, nickels, pennies) =
            calculate_result(expected_coins, expected_money);
        let (calculated_coins, coins_correct,
             calculated_money, money_correct) =
            check_result(expected_coins, expected_money,
                         quarters, dimes, nickels, pennies);
        if !(coins_correct && money_correct) {
            panic!("\n{}\n", printer::format_all(
                expected_coins, expected_money,
                quarters, dimes, nickels, pennies,
                calculated_coins, coins_correct,
                calculated_money, money_correct
            ));
        }
    }
}
