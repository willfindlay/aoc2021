const INPUT: &str = include_str!("input.txt");
const BITS: usize = 12;

fn part1() {
    let mut counts = [0usize; BITS];
    let nums: Vec<_> = INPUT
        .lines()
        .map(|l| u16::from_str_radix(l, 2).expect("bitstring must parse"))
        .collect();
    let num_count = nums.len();
    for n in nums {
        for i in 0..BITS {
            counts[i] += (if n & (1 << i) == 0 { 0 } else { 1 }) as usize;
        }
    }
    let gamma: u16 = counts.iter().enumerate().fold(0, |gamma, (i, &count)| {
        if count > (num_count / 2) {
            gamma | (1 << i)
        } else {
            gamma
        }
    });
    let epsilon = !gamma & (u16::MAX >> 4);
    println!("{}", gamma as usize * epsilon as usize);
}

fn part2() {
    let nums: Vec<_> = INPUT
        .lines()
        .map(|l| u16::from_str_radix(l, 2).expect("bitstring must parse"))
        .collect();
    todo!()
}

fn main() {
    part1();
    part2();
}
