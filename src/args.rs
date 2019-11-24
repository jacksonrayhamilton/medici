use std::{env, process};

pub fn parse_args() -> (u32, u32) {
    (parse_arg("coins", 1), parse_arg("money", 2))
}

fn parse_arg(name: &str, n: usize) -> u32 {
    match match env::args().nth(n) {
        Some(s) => match s.parse::<u32>() {
            Ok(n) => Ok(n),
            Err(e) => Err(format!("Unparseable arg “{}” (arg #{}): {}", name, n, e))
        },
        None => Err(format!("Missing arg “{}” (arg #{})", name, n))
    } {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Error! {}", e);
            eprintln!();
            usage();
            process::exit(1);
        }
    }
}

fn usage() {
    eprintln!(concat!(
        "USAGE:\n",
        "    medici COINS MONEY\n",
        "\n",
        "COINS: Number of coins of any denomination\n",
        "MONEY: Total cents the coins summate\n",
        "\n",
        "EXAMPLE:\n",
        "    medici 1692 10054"
    ));
}
