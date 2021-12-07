use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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

fn main() {
    let lines = read_lines("./input.txt").unwrap_or_default();
    let numbers: Vec<i32> = lines[0]
        .split(",")
        .map(|l| l.to_string().parse::<i32>().unwrap())
        .collect();
    println!("Part1 : {}", compute_smallest_part1(numbers.clone()));

    println!("Part2 : {}", compute_smallest_part2(numbers));
}

fn compute_smallest_part1(numbers: Vec<i32>) -> i32 {
    let mut fuel_consumption: Vec<i32> = Vec::new();

    for nb in 0..numbers.len() {
        fuel_consumption.insert(nb, numbers.iter().map(|n| (n - nb as i32).abs()).sum());
    }
    fuel_consumption.sort();
    return fuel_consumption[0];
}

fn compute_smallest_part2(numbers: Vec<i32>) -> i32 {
    let mut fuel_consumption: Vec<i32> = Vec::new();

    for nb in 0..numbers.len() {
        fuel_consumption.insert(
            nb,
            numbers
                .iter()
                .map(|n| (n - nb as i32).abs())
                .map(|steps| (1..steps + 1).fold(0, |cost, i| cost + i))
                .sum(),
        );
    }
    fuel_consumption.sort();
    return fuel_consumption[0];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality() {
        let numbers = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        assert_eq!(37, compute_smallest_part1(numbers.clone()));

        assert_eq!(168, compute_smallest_part2(numbers));
    }
}
