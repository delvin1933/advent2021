use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut gamma: String = "".to_string();
    let mut epsilon: String = "".to_string();

    if let Ok(lines) = read_lines("./input.txt") {
        for i in 0..12 {
            if calculate_ratio(&lines, i) {
                gamma.push('1');
                epsilon.push('0');
            } else {
                gamma.push('0');
                epsilon.push('1');
            }
        }

        //Part 1

        let gamma = isize::from_str_radix(&gamma, 2).unwrap();
        let epsilon = isize::from_str_radix(&epsilon, 2).unwrap();

        println!("Gamma {}", gamma);
        println!("Epsilon {}", epsilon);
        println!("Product {}", gamma * epsilon);

        // Part 2
        let oxygen_rate = compute_oxygen_rate(lines.clone());
        println!("Oxygen {}", oxygen_rate);
        let co2_rate = compute_co2_rate(lines.clone());
        println!("CO2 {}", co2_rate);

        let oxygen_rate = isize::from_str_radix(&oxygen_rate, 2).unwrap();
        let co2_rate = isize::from_str_radix(&co2_rate, 2).unwrap();

        println!("Survival {}", oxygen_rate * co2_rate);
    }
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

fn calculate_ratio(lines: &Vec<String>, column: usize) -> bool {
    let mut nb1 = 0;
    let mut total_lines = 0;
    for line in lines {
        total_lines += 1;
        match line.chars().nth(column) {
            Some('1') => nb1 = nb1 + 1,
            _ => (),
        }
    }

    nb1 >= total_lines / 2
}

fn compute_oxygen_rate(lines: Vec<String>) -> String {
    let last_line = compute_oxygen_rate_inner(lines, 0, '1', '0');
    last_line[0].clone()
}

fn compute_co2_rate(lines: Vec<String>) -> String {
    let last_line = compute_oxygen_rate_inner(lines, 0, '0', '1');
    last_line[0].clone()
}

fn compute_oxygen_rate_inner(
    lines: Vec<String>,
    column_nb: usize,
    greater: char,
    lower: char,
) -> Vec<String> {
    if lines.len() == 1 {
        return lines;
    }

    let tmp_lines: Vec<String>;

    if calculate_ratio(&lines, column_nb) {
        tmp_lines = lines
            .into_iter()
            .filter(|s| s.chars().nth(column_nb).unwrap() == greater)
            .collect();
    } else {
        tmp_lines = lines
            .into_iter()
            .filter(|s| s.chars().nth(column_nb).unwrap() == lower)
            .collect();
    }
    return compute_oxygen_rate_inner(tmp_lines, column_nb + 1, greater, lower);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_calculate_ratio() {
        let lines: Vec<String> = vec![
            "00100".into(),
            "11110".into(),
            "10110".into(),
            "10111".into(),
            "10101".into(),
            "01111".into(),
            "00111".into(),
            "11100".into(),
            "10000".into(),
            "11001".into(),
            "00010".into(),
            "01010".into(),
        ];

        assert_eq!(calculate_ratio(lines, 0), true);
    }

    #[test]
    fn test_calculate_ratio_meme_nombre() {
        let lines: Vec<String> = vec!["0".into(), "1".into()];

        assert_eq!(calculate_ratio(lines, 0), true);
    }
}
