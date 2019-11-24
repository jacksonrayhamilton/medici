pub const QUARTER_VALUE: u32 = 25;
pub const DIME_VALUE: u32 = 10;
pub const NICKEL_VALUE: u32 = 5;
pub const PENNY_VALUE: u32 = 1;

pub fn sum_of_denominations(quarters: u32, dimes: u32, nickels: u32, pennies: u32) -> u32 {
    quarters * QUARTER_VALUE +
        dimes * DIME_VALUE +
        nickels * NICKEL_VALUE +
        pennies * PENNY_VALUE
}
