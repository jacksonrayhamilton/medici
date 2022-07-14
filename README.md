# Medici

## Inspiration

This project was inspired by a math problem posed to *Parade* magazine's
Marilyn Vos Savant in November 2019:

> In a contest, a friend guessed the exact number of coins in a jar: 1,692. She
> won all of the coins, which totaled $100.54. They consisted of pennies,
> nickels, dimes and quarters. How many coins of each kind were in the jar?

*Source:* https://parade.com/951440/marilynvossavant/can-you-solve-this-coin-riddle/

This project serves as a generalized solver program given any number of coins
and any dollar amount.

## Usage

### Setup

[Install Rust.](https://www.rust-lang.org/tools/install)

Compile the program:

```sh
$ cargo build
```

### Run program

```
USAGE
    medici COINS MONEY

COINS: Number of coins of any denomination
MONEY: Total cents the coins summate
```

```
$ cargo build
$ ./target/debug/medici 1692 10054
```

The program will print the total number of pennies, nickels, dimes and quarters.  e.g.:

```
INPUT
Coins: 1692
Money: $100.54

RESULT
Quarters: 0
Dimes: 322
Nickels: 1366
Pennies: 4

CHECK
Coins: 1692 (✓)
Money: $100.54 (✓)
```
