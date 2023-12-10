use core::num;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*};
use std::ops::Index;

pub fn main() -> io::Result<()>
{
    let path = env::args().nth(1).unwrap();
    let mut file = File::open(path)?;
    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf);

    let chars: Vec<char> = buf.chars().collect();

    let mut line = 0;

    let width = buf.chars().position(|c| c == '\n').unwrap() + 1;

    println!("{width}");

    for (i, char) in buf.chars().enumerate()
    {
        if char == '\n' { line += 1 }
        if char != '*' { continue }

        if line > 0
        {
            let start_idx = i - (i > 0) as usize;
            let end_idx = i + (chars[i] != '\n') as usize;

            let mut num_buf = String::new();

            for j in start_idx..end_idx
            {
                if !chars[j].is_ascii_digit() { continue }

                let mut k = j;
                while chars[k].is_ascii_digit()
                {
                    if !chars[k].is_ascii_digit() { break }

                    num_buf.push(chars[k]);

                    k -= 1;
                }

                while chars[k].is_ascii_digit()
                {
                    if !chars[k].is_ascii_digit() { break }

                    num_buf.push(chars[k]);

                    k -= 1;
                }
            }
        }
    }

    Ok(())
}
