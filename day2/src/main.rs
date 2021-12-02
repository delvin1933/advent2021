use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut depth: i32 = 0;
    let mut horizontal: i32 = 0;
    let mut aim: i32 = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            let parts: Vec<&str> = line.split_whitespace().collect();

            match parts[0] {
                "forward" => {
                    let value = parts[1].parse::<i32>().unwrap();
                    horizontal = horizontal + value;
                    depth = depth + aim * value;
                }
                "up" => aim = aim - parts[1].parse::<i32>().unwrap(),
                "down" => aim = aim + parts[1].parse::<i32>().unwrap(),
                _ => (),
            }
        }
    }

    println!("{}", depth * horizontal)
}

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect())
}
