use std::env;
use std::io::{self, prelude::*};
use std::fs::File;

pub fn main() -> io::Result<()>
{
    let path = env::args().nth(1).unwrap();
    let mut file = File::open(path)?;
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);

    for line in content.lines()
    {
        
    }

    Ok(())
}
