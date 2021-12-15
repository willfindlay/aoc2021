use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

lazy_static! {
    static ref LINE_RE: Regex =
        Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").expect("regex should compile");
}

const INPUT: &str = include_str!("input.txt");

struct Diagram {
    spaces: HashMap<(usize, usize), usize>,
}

impl Diagram {
    pub fn part1(&self) {
        println!("{:#?}", self.spaces);
        let score = self.spaces.values().filter(|&&x| x >= 2).count();
        println!("{}", score);
    }
}

impl FromStr for Diagram {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s
            .lines()
            .map(|s| {
                LINE_RE
                    .captures(s)
                    .map(|c| {
                        (
                            (
                                c[1].parse::<usize>().unwrap(),
                                c[2].parse::<usize>().unwrap(),
                            ),
                            (
                                c[3].parse::<usize>().unwrap(),
                                c[4].parse::<usize>().unwrap(),
                            ),
                        )
                    })
                    .ok_or_else(|| anyhow::anyhow!("Failed to parse ints"))
            })
            .collect::<Result<_>>()?;

        let mut diagram = Diagram {
            spaces: HashMap::default(),
        };

        for ((mut x1, mut y1), (x2, y2)) in lines {
            let x_step = if x1 < x2 { 1 } else { -1 };
            let y_step = if y1 < y2 { 1 } else { -1 };

            while x1 != x2 && y1 != y2 {
                *diagram.spaces.entry((x1, y1)).or_insert(0) += 1;
                if x1 != x2 {
                    x1 = (x1 as isize + x_step) as usize;
                }
                if y1 != y2 {
                    y1 = (y1 as isize + y_step) as usize;
                }
            }
        }

        Ok(diagram)
    }
}

fn main() {
    let diagram = Diagram::from_str(INPUT).unwrap();
    diagram.part1();
}
