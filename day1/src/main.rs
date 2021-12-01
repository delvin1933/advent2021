use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut increased = 0;
    let mut previous = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let measure = line.unwrap().parse::<i32>().unwrap() {
                if previous == 0 {
                    previous = measure
                } else if measure > previous {
                    previous = measure;
                    increased = increased + 1;
                } else {
                    previous = measure;
                }
            }
        }
    }

    println!("{}", increased);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
