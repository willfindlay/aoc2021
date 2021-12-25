use std::collections::HashSet;

use lazy_static::lazy_static;

const INPUT: &str = include_str!("input.txt");

lazy_static! {
    static ref NUM_SEGMENTS_UNIQUE: HashSet<u32> = {
        let mut set = HashSet::new();
        set.insert(2);
        set.insert(4);
        set.insert(3);
        set.insert(7);
        set
    };
}

fn part1() {
    let num_unique_counts = INPUT
        .lines()
        .map(|s| s.split_once(" | ").unwrap().1)
        .flat_map(|s| s.split_terminator(' ').map(|s| s.len() as u32))
        .filter(|len| NUM_SEGMENTS_UNIQUE.get(len).is_some())
        .count();
    println!("{}", num_unique_counts)
}

fn part2() {
    let mut ans = 0;
    for (patterns, numbers) in INPUT.lines().map(|s| s.split_once(" | ").unwrap()) {
        let mut decoded = String::new();
        // We need to find the set of segments that decodes to a one...
        let one = patterns
            .split_terminator(' ')
            .find(|p| p.len() == 2)
            .map(str::chars)
            .map(HashSet::<char>::from_iter)
            .unwrap();
        // ... and the set that decodes to a four.
        let four = patterns
            .split_terminator(' ')
            .find(|p| p.len() == 4)
            .map(str::chars)
            .map(HashSet::<char>::from_iter)
            .unwrap();
        // This works based on the observation that number of segments plus the
        // intersection between one's segments and four's segments is enough to
        // uniquely identify a given digit.
        for n in numbers
            .split_terminator(' ')
            .map(|n| HashSet::<char>::from_iter(n.chars()))
        {
            match (
                n.len(),
                n.intersection(&four).count(),
                n.intersection(&one).count(),
            ) {
                (2, _, _) => decoded.push('1'),
                (3, _, _) => decoded.push('7'),
                (4, _, _) => decoded.push('4'),
                (7, _, _) => decoded.push('8'),
                (5, 3, 2) => decoded.push('3'),
                (5, 3, 1) => decoded.push('5'),
                (5, 2, _) => decoded.push('2'),
                (6, 4, _) => decoded.push('9'),
                (6, 3, 2) => decoded.push('0'),
                (6, 3, 1) => decoded.push('6'),
                (_, _, _) => unreachable!(),
            }
        }
        ans += decoded.parse::<u32>().unwrap();
    }
    println!("{}", ans);
}

fn main() {
    part1();
    part2();
}
