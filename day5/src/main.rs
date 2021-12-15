use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::ops::Deref;
use std::{collections::HashMap, str::FromStr};

lazy_static! {
    static ref LINE_RE: Regex =
        Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").expect("regex should compile");
}

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Line {
    pub p1: (u32, u32),
    pub p2: (u32, u32),
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = LINE_RE
            .captures(s)
            .ok_or_else(|| anyhow::anyhow!("Failed to capture"))?;
        let (x1, y1, x2, y2) = (
            captures[1].parse::<u32>().unwrap(),
            captures[2].parse::<u32>().unwrap(),
            captures[3].parse::<u32>().unwrap(),
            captures[4].parse::<u32>().unwrap(),
        );
        Ok(Self {
            p1: (x1, y1),
            p2: (x2, y2),
        })
    }
}

struct Diagram {
    spaces: HashMap<(u32, u32), usize>,
}

fn part1() -> Result<()> {
    let lines: Vec<_> = INPUT
        .lines()
        .map(|s| Line::from_str(s))
        .collect::<Result<_>>()?;

    let mut diagram = Diagram {
        spaces: HashMap::default(),
    };

    lines.iter().for_each(|line| {
        let (x1, y1) = line.p1;
        let (x2, y2) = line.p2;
        if x1 == x2 {
            let (small, large) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            (small..=large).for_each(|y| {
                let point = (x1, y);
                *diagram.spaces.entry(point).or_insert(0) += 1;
            });
        } else if y1 == y2 {
            let (small, large) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            (small..=large).for_each(|x| {
                let point = (x, y1);
                *diagram.spaces.entry(point).or_insert(0) += 1;
            });
        }
    });

    let ans = diagram.spaces.values().filter(|&&v| v >= 2).count();
    println!("{}", ans);

    Ok(())
}

fn part2() -> Result<()> {
    let lines: Vec<_> = INPUT
        .lines()
        .map(|s| Line::from_str(s))
        .collect::<Result<_>>()?;

    let mut diagram = Diagram {
        spaces: HashMap::default(),
    };

    lines.iter().for_each(|line| {
        let (x1, y1) = line.p1;
        let (x2, y2) = line.p2;
        if x1 == x2 {
            let (small, large) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            (small..=large).for_each(|y| {
                let point = (x1, y);
                *diagram.spaces.entry(point).or_insert(0) += 1;
            });
        } else if y1 == y2 {
            let (small, large) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            (small..=large).for_each(|x| {
                let point = (x, y1);
                *diagram.spaces.entry(point).or_insert(0) += 1;
            });
        } else {
            let xs: Vec<_> = if x1 < x2 {
                (x1..=x2).collect()
            } else {
                (x2..=x1).rev().collect()
            };
            let ys: Vec<_> = if y1 < y2 {
                (y1..=y2).collect()
            } else {
                (y2..=y1).rev().collect()
            };
            xs.iter().zip(ys.iter()).for_each(|(&x, &y)| {
                let point = (x, y);
                *diagram.spaces.entry(point).or_insert(0) += 1;
            });
        }
    });

    let ans = diagram.spaces.values().filter(|&&v| v >= 2).count();
    println!("{}", ans);

    Ok(())
}

fn main() {
    part1().expect("failed to run part 1");
    part2().expect("failed to run part 2");
}
