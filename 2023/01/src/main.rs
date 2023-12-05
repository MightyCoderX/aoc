use std::env;
use std::io::{self, prelude::*, BufReader};
use std::fs::File;

fn main() -> io::Result<()>
{
    let path = env::args().nth(1).unwrap();
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());
    
    let mut first = 0;
    let mut last = 0;

    for line in reader.lines()
    {
        
        println!("{}", line?);
    }

    Ok(())
}
