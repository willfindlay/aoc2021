use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::{Context, Result};

const INPUT: &str = include_str!("input.txt");

struct BingoBoard {
    indices: HashMap<usize, (usize, usize)>,
    unmarked: HashSet<usize>,
    row_matches_hist: HashMap<usize, usize>,
    col_matches_hist: HashMap<usize, usize>,
    finished: bool,
}

impl BingoBoard {
    pub fn new(numbers: Vec<Vec<usize>>) -> Self {
        let mut board = BingoBoard {
            indices: HashMap::default(),
            unmarked: HashSet::default(),
            row_matches_hist: HashMap::default(),
            col_matches_hist: HashMap::default(),
            finished: false,
        };
        for (col, row) in numbers.iter().enumerate() {
            for (row, &n) in row.iter().enumerate() {
                board.indices.insert(n, (row, col));
                board.unmarked.insert(n);
            }
        }
        board
    }

    pub fn play(&mut self, called_number: usize) -> Option<usize> {
        if let Some((row, col)) = self.indices.get(&called_number) {
            self.unmarked.remove(&called_number);

            let col_matches = self.col_matches_hist.entry(*col).or_insert(0);
            *col_matches += 1;

            let row_matches = self.row_matches_hist.entry(*row).or_insert(0);
            *row_matches += 1;

            // We have a winner
            if *col_matches >= 5 || *row_matches >= 5 {
                self.finished = true;
                println!("{:?}", self.unmarked);
                return Some(self.compute_score(called_number));
            }
        }

        None
    }

    fn compute_score(&self, called_number: usize) -> usize {
        self.unmarked.iter().sum::<usize>() * called_number
    }
}

struct BingoGame {
    numbers: Vec<usize>,
    boards: Vec<BingoBoard>,
}

impl BingoGame {
    pub fn play(&mut self) {
        // Play all numbers in order
        for n in self.numbers.iter() {
            // Play all boards in order
            for board in self.boards.iter_mut() {
                // First one wins
                if let Some(score) = board.play(*n) {
                    println!("score was {}", score);
                    return;
                }
            }
        }
    }

    pub fn play_to_lose(&mut self) {
        let num_boards = self.boards.len();
        let mut won = 0;

        // Play all numbers in order
        for n in self.numbers.iter() {
            // Play all boards in order
            for board in self.boards.iter_mut() {
                if board.finished {
                    continue;
                }
                // Last one to win is the one we want
                if let Some(score) = board.play(*n) {
                    won += 1;
                    if won == num_boards {
                        println!("score was {}", score);
                        return;
                    }
                }
            }
        }
    }
}

impl FromStr for BingoGame {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut boards = Vec::new();
        let mut lines = s.lines();

        // Parse out play numbers
        let numbers: Vec<usize> = lines
            .next()
            .context("No first line")?
            .split_terminator(',')
            .map(|s| s.parse::<usize>().map_err(anyhow::Error::from))
            .collect::<Result<_>>()
            .context("Failed to parse number")?;

        // Parse out individual boards
        let mut lines = lines.peekable();
        while lines.peek().is_some() {
            // Skip the whitespace
            let board: Vec<&str> = lines.by_ref().skip(1).take(5).collect();
            let board: Vec<Vec<usize>> = board
                .iter()
                .map(|row| {
                    row.split_whitespace()
                        .map(|n| n.parse::<usize>().map_err(anyhow::Error::from))
                        .collect::<Result<Vec<usize>>>()
                })
                .collect::<Result<_>>()?;
            boards.push(BingoBoard::new(board));
        }
        Ok(BingoGame { numbers, boards })
    }
}

fn part1() {
    let mut game = BingoGame::from_str(INPUT).expect("Failed to create game");
    game.play()
}

fn part2() {
    let mut game = BingoGame::from_str(INPUT).expect("Failed to create game");
    game.play_to_lose()
}

fn main() {
    part1();
    part2();
}
