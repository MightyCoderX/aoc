use std::env;
use std::io::{self, prelude::*, BufReader};
use std::fs::File;

pub fn main() -> io::Result<()>
{
    let path = env::args().nth(1).unwrap();
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());

    let mut sum = 0;

    let mut i;
    let mut j;

    for line in reader.lines()
    {
        let line = line.unwrap();
        let len = line.len();
        let chars = line.chars();
        let chars: Vec<char> = chars.collect();

        let mut first = '\0';
        let mut last = '\0';

        i = 0;
        j = len - 1;
        
        while i < len || j > 0
        {
            // println!("{line} {i} {j} {chars:?}");
            if chars.get(i).unwrap().is_numeric() && first == '\0'
            {
                first = *chars.get(i).unwrap();
            }

            if chars.get(j).unwrap().is_numeric() && last == '\0'
            {
                last = *chars.get(j).unwrap();
            }

            i+=1;
            if j > 0 { j-=1; }
        }

        let value = format!("{}{}", first, last);

        sum += value.parse::<i32>().unwrap();
    }

    println!("Sum {}", sum);

    Ok(())
}
