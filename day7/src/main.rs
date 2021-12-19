const INPUT: &str = include_str!("input.txt");

fn part1() -> u32 {
    let mut crabs: Vec<u32> = INPUT
        .lines()
        .next()
        .unwrap()
        .split_terminator(',')
        .map(|s| s.parse().unwrap())
        .collect();
    crabs.sort_unstable();
    let median = *crabs.get(crabs.len() / 2).unwrap();
    crabs
        .iter()
        .map(|&pos| (pos as i64 - median as i64).abs() as u32)
        .sum()
}

fn part2() -> u32 {
    let crabs: Vec<u32> = INPUT
        .lines()
        .next()
        .unwrap()
        .split_terminator(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let mean = crabs.iter().sum::<u32>() as f64 / crabs.len() as f64;
    let lo = mean.floor() as u32;
    let hi = mean.ceil() as u32;
    let delta_lo: u32 = crabs
        .iter()
        .map(|&pos| (pos as i64 - lo as i64).abs() as u32)
        .map(partial_sum)
        .sum();
    let delta_hi: u32 = crabs
        .iter()
        .map(|&pos| (pos as i64 - hi as i64).abs() as u32)
        .map(partial_sum)
        .sum();
    delta_lo.min(delta_hi)
}

fn partial_sum(n: u32) -> u32 {
    n * (n + 1) / 2
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
