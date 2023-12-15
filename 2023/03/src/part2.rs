use std::env;
use std::fs::File;
use std::io::{self, prelude::*};

pub fn main() -> io::Result<()> {
    let verbose: bool = !env::var("VERBOSE").unwrap_or("".to_string()).is_empty();
    
    let path = env::args().nth(1).unwrap();
    let mut file = File::open(path)?;
    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf);

    let lines: Vec<Vec<char>> = buf.lines().map(|l| l.chars().collect::<Vec<_>>()).collect();

    let width = lines[0].len();
    let height = lines.len();

    let mut result = 0;

    if verbose {
        println!("{width}x{height}\n");
    }

    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            let mut part_numbers = 0;
            let mut ratio = 1;
            if *char != '*' {
                continue;
            }

            if verbose {
                println!("------[GEAR]------");
            }

            let top = y - ((y > 0) as usize);
            let left = x - ((x > 0) as usize);
            let bottom = y + ((y < height) as usize);
            let right = x + ((x < width) as usize);

            for ry in top..=bottom {
                let mut to_skip = 0;
                if verbose {
                    println!(
                        "{}",
                        lines[ry][left..=right].into_iter().collect::<String>()
                    );
                }

                for rx in left..=right {
                    if to_skip > 0 {
                        // println!("\nskipping {:?}", lines[ry][rx]);
                        to_skip -= 1;
                        continue;
                    }

                    let char = lines[ry][rx];

                    let mut buf = String::new();

                    if !char.is_ascii_digit() { continue }
                    
                    buf.push(char);

                    let mut i = 1;

                    while lines[ry][rx - i].is_ascii_digit() {
                        buf.insert(0, lines[ry][rx - i]);

                        if rx - i == 0 {
                            break;
                        }

                        i += 1;
                    }

                    i = 1;
                    while rx + i < width && lines[ry][rx + i].is_ascii_digit() {
                        buf.push(lines[ry][rx + i]);
                        i += 1;
                    }

                    to_skip = i - 1;

                    ratio *= buf.parse::<i32>().unwrap();
                    part_numbers += 1;

                    if verbose {
                        if part_numbers > 1 && !buf.is_empty() {
                            let num1 = ratio / buf.parse::<i32>().unwrap();
                            println!("\nPN1 = {num1}");
                            println!("PN2 = {buf}");
                        }
                    }
                }
            }

            if verbose {
                println!("\nratio: {ratio}");
                println!("-------------------\n");
            }

            if part_numbers < 2 {
                ratio = 0;
            }

            result += ratio;
        }
    }

    println!("result = {result}");

    Ok(())
}
