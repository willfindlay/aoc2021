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
        for (i, count) in counts.iter_mut().enumerate().take(BITS) {
            *count += (if n & (1 << i) == 0 { 0 } else { 1 }) as usize;
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

enum Criteria {
    MostCommon,
    LeastCommon,
}

fn filter(criteria: Criteria, candidates: &[u16], i: usize) -> Vec<u16> {
    let len = candidates.len();
    let ones = candidates
        .iter()
        .fold(0, |acc, c| if (c & (1 << i)) == 0 { acc } else { acc + 1 });
    let zeroes = len - ones;
    let mcb = if ones >= zeroes { 1 << i } else { 0 };
    match criteria {
        Criteria::MostCommon => candidates
            .iter()
            .filter(|&&c| (c & (1 << i)) == mcb)
            .cloned()
            .collect(),
        Criteria::LeastCommon => candidates
            .iter()
            .filter(|&&c| (c & (1 << i)) != mcb)
            .cloned()
            .collect(),
    }
}

fn part2() {
    let nums: Vec<_> = INPUT
        .lines()
        .map(|l| u16::from_str_radix(l, 2).expect("bitstring must parse"))
        .collect();
    let o2 = {
        let mut candidates: Vec<_> = nums.clone();
        for i in (0..BITS).rev() {
            candidates = filter(Criteria::MostCommon, &candidates, i);
            println!("{:?}", candidates);
            if candidates.len() == 1 {
                break;
            }
        }
        candidates[0]
    };
    let co2 = {
        let mut candidates: Vec<_> = nums;
        for i in (0..BITS).rev() {
            candidates = filter(Criteria::LeastCommon, &candidates, i);
            println!("{:?}", candidates);
            if candidates.len() == 1 {
                break;
            }
        }
        candidates[0]
    };
    println!("{}", o2 as usize * co2 as usize);
}

fn main() {
    part1();
    part2();
}
