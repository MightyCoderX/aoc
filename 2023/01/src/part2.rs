use std::collections::HashMap;
use std::env;
use std::io::{self, prelude::*, BufReader};
use std::fs::File;

pub fn main() -> io::Result<()>
{
    let path = env::args().nth(1).unwrap();
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());

    let numbers: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9')
    ]);

    let mut sum = 0;

    let mut i: i32;
    let mut i_offset;
    let mut j: i32;
    let mut j_offset;

    for (n, line) in reader.lines().enumerate()
    {
        let line = line.unwrap();
        let len = line.len() as i32;
        let chars = line.chars();
        let chars: Vec<char> = chars.collect();  

        let mut first: char = '\0';
        let mut last: char = '\0';

        let mut buf_first = String::new();
        let mut buf_last = String::new(); 

        i = 0;
        j = len - 1;
        i_offset = 0;
        j_offset = 0;

        println!("#{} len: {len} {line:?}", n+1);

        while i < len && first == '\0'
        {
            println!("{i}\t{buf_first}");

            let i_char = *chars.get(i as usize).unwrap();

            if i_char.is_numeric()
            {
                first = i_char;
                break;
            }

            buf_first.push(i_char);
                
            if numbers.contains_key(&buf_first as &str)
            {
                first = *numbers.get(&buf_first as &str).unwrap();
                break;
            }
            
            if buf_first.len() > 5 || (*chars.get((i+1) as usize).unwrap()).is_numeric()
            {
                buf_first.clear();
                i = i_offset;
                i_offset += 1;
            }

            i+=1;
        }

        println!("first: {first:?}");
        
        while j > -1 && last == '\0'
        {
            println!("{j}\t{buf_last}");

            let j_char = *chars.get(j as usize).unwrap();
            
            if j_char.is_numeric()
            {
                last = j_char;
                break;
            }
            
            buf_last.insert(0, j_char);
            
            if numbers.contains_key(&buf_last as &str)
            {
                last = *numbers.get(&buf_last as &str).unwrap();
                break;
            }
            
            if buf_last.len() > 5 || (*chars.get((j-1) as usize).unwrap()).is_numeric()
            {
                buf_last.clear();
                j = (len - 1) - j_offset;
                j_offset += 1;
            }

            j -= 1;
        }

        println!("last: {last:?}");
        
        let value = format!("{}{}", first, last);

        println!("{value:?}\n");

        sum += value.parse::<i32>().unwrap();
    }

    println!("Sum {}", sum);

    Ok(())
}
