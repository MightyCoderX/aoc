use std::env;
use std::io::{self, prelude::*, BufReader};
use std::fs::File;

pub fn main() -> io::Result<()>
{
    let path = env::args().nth(1).unwrap();
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());

    for line in reader.lines()
    {
        let line = line.unwrap();
        let len = line.len();
        let chars = line.chars();
        let chars: Vec<char> = chars.collect();  

        // Algorithm here
    }

    Ok(())
}
