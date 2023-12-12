use std::env;
use std::fs::File;
use std::io::{self, prelude::*};

pub fn main() -> io::Result<()>
{
    let path = env::args().nth(1).unwrap();
    let mut file = File::open(path)?;
    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf);

    let lines: Vec<_> = buf.lines().collect();
    let chars: Vec<_> = buf.chars().collect();

    let width = lines[0].len();
    let height = lines.len();

    println!("{width}x{height}");

    for (y, line) in buf.lines().enumerate()
    {
        
        for (x, char) in buf.chars().enumerate()
        {
            if char == '*' { continue }

            if let c = chars[y*(x -1)].is_ascii_digit()
            {
                
            }


        }
    }

    Ok(())
}
