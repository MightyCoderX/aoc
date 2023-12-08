use std::collections::HashMap;
use std::env;
use std::io::{self, prelude::*, BufReader};
use std::fs::File;

pub fn main() -> io::Result<()>
{
    let path = env::args().nth(1).unwrap();
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());

    let mut sum = 0;

    println!("Games: ");

    for line in reader.lines()
    {
        let line = line.unwrap();
        let len = line.len();
        let chars = line.chars();
        let chars: Vec<char> = chars.collect();

        let (game, sets) = line.split_once(": ").unwrap();

        let game_id = game.split_once(' ').unwrap().1;
        let sets: Vec<_> = sets.split("; ").collect();

        let mut color_limits = HashMap::from([
            ("red", 12),
            ("green", 13),
            ("blue", 14)
        ]);

        
        for set in sets
        {
            let color_count_pairs:Vec<_> = set.split(", ").collect();
            println!("\t{}", color_count_pairs.len());

            for pair in color_count_pairs
            {
                let (count, color) = pair.split_once(" ").unwrap();
                
                let count = count.parse::<i32>().unwrap();
                
                let current = color_limits.get(color).unwrap().to_owned();
                
                color_limits.insert(color, current-count);
            }
        }

        if color_limits.iter().any(|x| x.1 < &0)
        {
            continue;
        }

        println!("game_id: {game_id} ");
        sum += game_id.parse::<i32>().unwrap();
    }

    println!("\nSum: {sum}");

    Ok(())
}
