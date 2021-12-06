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
    //let lines = read_lines("./input.txt").unwrap_or_default();

    //let lanterns: Vec<usize> = parse_lanterns(&lines[0]);
    let lanterns: Vec<usize> = vec![
        2, 1, 1, 1, 1, 1, 1, 5, 1, 1, 1, 1, 5, 1, 1, 3, 5, 1, 1, 3, 1, 1, 3, 1, 4, 4, 4, 5, 1, 1,
        1, 3, 1, 3, 1, 1, 2, 2, 1, 1, 1, 5, 1, 1, 1, 5, 2, 5, 1, 1, 2, 1, 3, 3, 5, 1, 1, 4, 1, 1,
        3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4, 1, 5, 1, 2, 1, 1, 1, 1, 5, 1, 1, 1, 1, 1, 5, 1,
        1, 1, 4, 5, 1, 1, 3, 4, 1, 1, 1, 3, 5, 1, 1, 1, 2, 1, 1, 4, 1, 4, 1, 2, 1, 1, 2, 1, 5, 1,
        1, 1, 5, 1, 2, 2, 1, 1, 1, 5, 1, 2, 3, 1, 1, 1, 5, 3, 2, 1, 1, 3, 1, 1, 3, 1, 3, 1, 1, 1,
        5, 1, 1, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 3, 1, 1, 4, 1, 1, 3, 2, 1, 2, 1, 1, 2, 2, 1, 2, 1,
        1, 1, 4, 1, 2, 4, 1, 1, 4, 4, 1, 1, 1, 1, 1, 4, 1, 1, 1, 2, 1, 1, 2, 1, 5, 1, 1, 1, 1, 1,
        5, 1, 3, 1, 1, 2, 3, 4, 4, 1, 1, 1, 3, 2, 4, 4, 1, 1, 3, 5, 1, 1, 1, 1, 4, 1, 1, 1, 1, 1,
        5, 3, 1, 5, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5, 1, 1, 1, 1, 1,
        1, 1, 1, 5, 1, 4, 4, 1, 1, 1, 1, 1, 1, 1, 1, 3, 1, 3, 1, 4, 1, 1, 2, 2, 2, 1, 1, 2, 1, 1,
    ];

    //println!(" {:?}", lanterns.len());

    let mut lanterns = count_lanterns(lanterns);

    //println!(" {:?}", lanterns);

    let lanterns = grow(256, lanterns);

    //println!(" {:?}", lanterns);
    let nb_lanterns: i128 = lanterns.iter().sum();

    println!("Part2 {}", nb_lanterns);
}

fn parse_lanterns(input: &String) -> Vec<usize> {
    input
        .split(",")
        .map(|i| i.parse::<usize>().unwrap())
        .collect()
}

fn count_lanterns(input: Vec<usize>) -> [i128; 9] {
    let mut output: [i128; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];

    for num in input {
        output[num] += 1;
    }
    output
}

fn grow(nb_grow: i32, mut population: [i128; 9]) -> [i128; 9] {
    for _grow in 0..nb_grow {
        let pop0 = population[0];

        population[0] = population[1];
        population[1] = population[2];
        population[2] = population[3];
        population[3] = population[4];
        population[4] = population[5];
        population[5] = population[6];
        population[6] = population[7] + pop0;
        population[7] = population[8];
        population[8] = pop0;
    }

    population
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality() {
        let input: String = "3,4,3,1,2".to_string();

        let population = parse_lanterns(&input);

        let expected: Vec<usize> = vec![3, 4, 3, 1, 2];

        assert_eq!(expected, population);

        let expected: [i128; 9] = [0, 1, 1, 2, 1, 0, 0, 0, 0];

        let pop = count_lanterns(population);

        assert_eq!(expected, pop);

        let grow1_expected: Vec<usize> = vec![2, 3, 2, 0, 1];
        let grow1_expected = count_lanterns(grow1_expected);

        let grow1 = grow(1, pop);

        assert_eq!(grow1_expected, grow1);
    }
}
