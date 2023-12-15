use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*};

pub fn main() -> io::Result<()> {
    let path = env::args().nth(1).unwrap();
    let mut file = File::open(path)?;
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);

    let result = content
        .lines()
        .map(|line| line.split_once(':').unwrap().1.split_once('|'))
        .map(|card| {
            (
                card.unwrap()
                    .0
                    .split_ascii_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect::<HashSet<i32>>(),
                card.unwrap()
                    .1
                    .split_ascii_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect::<HashSet<i32>>(),
            )
        })
        .fold(0, |mut acc, game| {
            let (winning, mine) = game;
            let found: HashSet<i32> = mine.into_iter().filter(|n| winning.contains(n)).collect();

            if found.len() > 0 {
                acc += 2i32.pow((found.len() - 1) as u32);
            }

            return acc;
        });

    println!("{result}");
    Ok(())
}
