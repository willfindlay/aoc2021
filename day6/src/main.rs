use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn simulation(seed: &[usize], days: usize) -> usize {
    let mut fish: HashMap<usize, usize> = HashMap::default();
    for &f in seed {
        *fish.entry(f).or_insert(0) += 1;
    }
    for _ in 0..days {
        // Simulate one iteration
        let spawning = *fish.get(&0).unwrap_or(&0);
        // Tick each fish over
        for pos in 0..7 {
            let swap = (pos + 1) % 8;
            let p1 = *fish.get(&pos).unwrap_or(&0);
            let p2 = *fish.get(&swap).unwrap_or(&0);
            fish.insert(pos, p2);
            fish.insert(swap, p1);
        }
        let baby = *fish.get(&8).unwrap_or(&0);
        let kid = *fish.get(&7).unwrap_or(&0);
        *fish.entry(6).or_insert(0) += kid;
        fish.insert(7, baby);
        fish.insert(8, spawning);
        println!("{:?}", fish);
    }
    fish.values().sum()
}

fn part1() {
    let seed: Vec<_> = INPUT
        .lines()
        .next()
        .unwrap()
        .split_terminator(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    println!("{}", simulation(&seed, 80));
}

fn part2() {
    let seed: Vec<_> = INPUT
        .lines()
        .next()
        .unwrap()
        .split_terminator(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    println!("{}", simulation(&seed, 256));
}

fn main() {
    part1();
    part2();
}
