use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*};

#[allow(non_camel_case_types)]
type int = u32;

#[derive(Debug, Clone)]
struct Card {
    num: int,
    winning_numbers: HashSet<int>,
    numbers_found: HashSet<int>,
}

pub fn main() -> io::Result<()> {
    let path = env::args().nth(1).unwrap();
    let mut file = File::open(path)?;
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);

    let cards = content
        .lines()
        .map(|line| line.split_once(':').unwrap().1.split_once('|'))
        .enumerate()
        .map(|(num, card)| Card {
            num: num as int,
            winning_numbers: card
                .unwrap()
                .0
                .split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<HashSet<int>>(),
            numbers_found: card
                .unwrap()
                .1
                .split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<HashSet<int>>(),
        });

    let result = cards.clone().fold(0, |acc, card| {
        acc + get_card_copies(card, cards.clone().collect())
    });

    println!("{result}");

    Ok(())
}

fn get_card_copies(card: Card, card_set: Vec<Card>) -> int {

    let common_nums: HashSet<int> = card
        .numbers_found
        .into_iter()
        .filter(|n| card.winning_numbers.contains(n))
        .collect();

    let mut count = 1;

    let start_idx = (card.num as usize) + 1;
    let end_idx = (card.num + common_nums.len() as int) as usize;

    let cards_won = &card_set[start_idx..=end_idx];

    for card_copy in cards_won
    {
        count += get_card_copies(card_copy.clone(), card_set.clone());
    }

    count
}
