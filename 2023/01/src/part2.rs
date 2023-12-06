use std::collections::HashMap;
use std::env;
use std::io::{self, prelude::*, BufReader};
use std::fs::File;

pub fn main() -> io::Result<()>
{
    let path = env::args().nth(1).unwrap();
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());

    let numbers = HashMap::from([
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

    let mut i;
    let mut i_offset;
    let mut j;
    let mut j_offset;

    for (n, line) in reader.lines().enumerate()
    {
        let line = line.unwrap();
        let len = line.len();
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

        println!("#{} len: {len}", n+1);
        
        while i < len || j > 0
        {
            println!("{i}\t{buf_first}\t{j}\t{buf_last}");

            let i_char = *chars.get(i).unwrap();
            let j_char = *chars.get(j).unwrap();
            
            if first == '\0'
            {
                if i_char.is_numeric()
                {
                    first = i_char;
                }
                else
                {
                    buf_first.push(i_char);
                    
                    if numbers.contains_key(&buf_first as &str)
                    {
                        first = *numbers.get(&buf_first as &str).unwrap();
                    }
                    else if buf_first.len() > 5 || (*chars.get(i+1).unwrap_or(&'\0')).is_numeric()
                    {
                        i = 0;
                        i_offset += 1;
                        buf_first = String::new();
                    }
                }
            }
            
            if last == '\0'
            {
                if j_char.is_numeric()
                {
                    last = j_char;
                }
                else
                {
                    buf_last.insert(0, j_char);
                    
                    if numbers.contains_key(&buf_last as &str)
                    {
                        last = *numbers.get(&buf_last as &str).unwrap();
                    }
                    else if buf_last.len() > 5 || (*chars.get(j+1).unwrap_or(&'\0')).is_numeric()
                    {
                        j = len - 1;
                        j_offset += 1;
                        buf_last = String::new();
                    }
                }
            }
            
            if i < len  { i += i_offset }
            if j > 0    { j -= j_offset }
        }

        println!("{first} {last}\n");
        
        let value = format!("{}{}", first, last);

        sum += value.parse::<i32>().unwrap();
    }

    println!("Sum {}", sum);

    Ok(())
}
