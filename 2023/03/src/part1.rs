use std::env;
use std::io::{self, prelude::*};
use std::fs::File;

#[derive(Debug)]
struct Number
{
    value: i32,
    x: usize,
    y: usize
}

pub fn main() -> io::Result<()>
{
    let path = env::args().nth(1).unwrap();
    let mut file = File::open(path)?;
    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf);

    let lines = buf.lines().collect::<Vec<&str>>();

    let width = lines.get(0).unwrap().len();
    let height = lines.len();
    
    let mut schematic = vec![vec!['\0'; width]; height];
    let mut numbers: Vec<Number> = Vec::new();

    for (y, line) in lines.iter().enumerate()
    {
        let len = line.len();
        let chars: Vec<char> = line.chars().collect();
        
        let mut num_buf = String::new();
        let mut num_start_x = 0;

        for (x, char) in chars.iter().enumerate()
        {
            schematic[y][x] = *char;
            
            if char.is_ascii_digit()
            {
                if num_buf.is_empty() { num_start_x = x }
                num_buf.push(*char);
            }
            else if !num_buf.is_empty()
            {
                numbers.push(Number { value: num_buf.parse().unwrap(), x: num_start_x, y });
                num_buf.clear();
            }
        }
    }

    
    let mut sum = 0;
    
    for num in numbers
    {
        let mut found_part_number = false;
        let num_len = num.value.to_string().len();

        if num.y > 0
        {
            let start_x = num.x as i32 - (num.x > 0) as i32;
            let mut end_x = num.x + num_len;
            end_x += (end_x < width) as usize;

            // println!("{num:?} {start_x} {end_x}");
            for x in start_x..(end_x as i32)
            {
                let char = schematic.get(num.y-1).unwrap().get(x as usize).unwrap();
                print!("{char}");

                if !char.is_alphanumeric() && *char != '.'
                {
                    found_part_number = true;
                    // break;
                }
            }
        }

        println!();

        if num.x > 0
        {
            let char = schematic.get(num.y).unwrap().get(num.x - 1).unwrap();
            print!("{char}");

            if !char.is_alphanumeric() && *char != '.'
            {
                found_part_number = true;
            }
        }

        print!("{}", num.value);

        if num.x + num_len < width
        {
            let char = schematic.get(num.y).unwrap().get(num.x + num_len).unwrap();
            print!("{char}");
                
            if !char.is_alphanumeric() && *char != '.'
            {
                found_part_number = true;
            }
        }

        println!();

        if num.y+1 < height
        {
            let start_x = num.x as i32 - (num.x > 0) as i32;
            let mut end_x = num.x + num_len;
            end_x += (end_x < width) as usize;

            // print!("{num:?} {start_x} {end_x}");
            for x in start_x..(end_x as i32)
            {
                let char = schematic.get(num.y+1).unwrap().get(x as usize).unwrap();
                print!("{char}");
                
                if !char.is_alphanumeric() && *char != '.'
                {
                    found_part_number = true;
                    // break;
                }
            }
        }

        println!("\n{found_part_number}\n-------\n");
        
        if !found_part_number { continue }
        
        sum += num.value;

    }

    println!("{sum}");

    Ok(())
}
