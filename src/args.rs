pub fn parse_args(line: String) -> (u32, u32) {
    let mut split = line.split(",");
    let coins = split.next().unwrap().parse::<u32>().unwrap();
    let money = (split.next().unwrap().parse::<f64>().unwrap() * 100.0) as u32;
    (coins, money)
}
