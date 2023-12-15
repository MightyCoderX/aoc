use std::env;
use std::fs::File;
use std::io::{self, prelude::*};

pub fn main() -> io::Result<()>
{
    let path = env::args().nth(1).unwrap();
    let mut file = File::open(path)?;
    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf);

    let lines: Vec<_> = buf.lines().map(|l| l.chars().collect::<Vec<_>>()).collect();

    let width = lines[0].len();
    let height = lines.len();

    let mut sum = 0;
    
    println!("{width}x{height}\n");
    
    for (y, line) in lines.iter().enumerate()
    {
        for (x, char) in line.iter().enumerate()
        {
            let mut part_numbers = 0;
            let mut ratio = 1;
            if *char != '*' { continue }
            
            println!("------[GEAR]------");
            
            let top = y - ((y > 0) as usize);
            let left = x - ((x > 0) as usize);
            let bottom = y + ((y < height) as usize);
            let right = x + ((x < width) as usize);

            println!("{} {} {} {}", top, left, bottom, right);
            
            for ry in top..=bottom
            {
                for rx in left..=right
                {
                    let char = lines[ry][rx];
                    print!("{char}");
                    
                    if char.is_ascii_digit()
                    {
                        let mut buf = String::new();
                        buf.push(char);
                        
                        let mut i = 1;
                        
                        while lines[ry][rx-i].is_ascii_digit()
                        {   
                            buf.insert(0, lines[ry][rx-i]);

                            if rx-i == 0
                            {
                                break;
                            }
                            
                            i+=1;

                        }

                        i = 1;
                        while rx+i < width && lines[ry][rx+i].is_ascii_digit()
                        {
                            buf.push(lines[ry][rx+i]);
                            i+=1;
                        }
                        println!("\nnum = {buf}");

                        ratio *= buf.parse::<i32>().unwrap();
                        part_numbers+=1;
                        
                        break;
                    }   
                }
            }
            
            println!("-------------------");
            
            if part_numbers < 2
            {
                ratio = 0;
            }

            println!("ratio: {ratio}\n");

            sum += ratio;
        }
    }

    println!("\n\nsum = {sum}");

    Ok(())
}
