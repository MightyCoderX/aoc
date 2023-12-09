use std::env;
use std::io::{self, prelude::*, BufReader};
use std::fs::File;

pub fn main() -> io::Result<()>
{
    let path = env::args().nth(1).unwrap();
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());

    let mut sum = 0;

    for line in reader.lines()
    {
        let line = line.unwrap();
        let len = line.len();
        let chars = line.chars();
        let chars: Vec<char> = chars.collect();

        let sets = line.split_once(": ").unwrap().1;

        let sets: Vec<_> = sets.split("; ").collect();

        let mut red_max = 0;
        let mut green_max = 0;
        let mut blue_max = 0;

        for set in sets
        {
            let color_count_pairs:Vec<_> = set.split(", ").collect();

            for pair in color_count_pairs
            {
                let (count, color) = pair.split_once(" ").unwrap();
                
                let count = count.parse::<i32>().unwrap();
                
                match color
                {
                    "red" => if count > red_max { red_max = count },
                    "green" => if count > green_max { green_max = count},
                    "blue" => if count > blue_max { blue_max = count },
                    _ => {}
                }
            }
        }

        sum += red_max * green_max * blue_max;
    }

    println!("\nSum: {sum}");

    Ok(())
}
