use itertools::Itertools;
use std::collections::HashMap;
use utils::read_lines;

fn parse_input(input: Vec<String>) -> (String, HashMap<String, (char, String, String)>) {
    let mut input = input.iter();
    let seed = input.next().unwrap();

    input.next();

    let mut instructions: HashMap<String, (char, String, String)> = HashMap::new();

    for instruction in input {
        let mut parts = instruction.split(" -> ").into_iter();
        let mut keys = parts.next().unwrap().chars();
        let key = (keys.next().unwrap(), keys.next().unwrap());

        let middle = parts.next().unwrap().chars().next().unwrap();
        let front = format!("{}{}", key.0, middle);
        let back = format!("{}{}", middle, key.1);

        instructions.insert(format!("{}{}", key.0, key.1), (middle, front, back));
    }

    (seed.to_string(), instructions)
}

fn count_chars(input: &HashMap<char, u64>) -> u64 {
    let max = input.iter().max_by_key(|entry| entry.1).unwrap();
    let min = input.iter().min_by_key(|entry| entry.1).unwrap();

    max.1 - min.1
}

fn step(
    seed: &String,
    instructions: &HashMap<String, (char, String, String)>,
    iterations: u32,
) -> u64 {
    let mut char_counter: HashMap<char, u64> = HashMap::new();
    let mut current_step: HashMap<String, u64> = HashMap::new();

    let seed = seed.chars().collect::<Vec<char>>();

    for (front, back) in seed.iter().tuple_windows() {
        let count = char_counter.entry(*front).or_insert(0);
        *count += 1;

        *(current_step
            .entry(format!("{}{}", *front, *back))
            .or_insert(0)) += 1;
    }
    
    *(char_counter.entry(*seed.last().unwrap()).or_insert(0)) += 1;

    for _i in 0..iterations {
        let mut next_step: HashMap<String, u64> = HashMap::new();

        for (pair, count) in current_step.clone() {
            let (middle, front, back) = instructions.get(&pair).unwrap();

            *(char_counter.entry(*middle).or_insert(0)) += count;

            *(next_step.entry(front.clone()).or_insert(0)) += count;

            *(next_step.entry(back.clone()).or_insert(0)) += count;
        }
        current_step = next_step;
    }

    count_chars(&char_counter)
}

fn main() {
    let lines = read_lines("input.txt").unwrap_or_default();

    let (seed, instructions) = parse_input(lines);

    //let mut seed = step(seed, &instructions);

    println!("Part 1 : {}", step(&seed, &instructions, 10));

    println!("Part 2 : {}", step(&seed, &instructions, 40));
}

mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_parse_input() {
        let input = vec![
            "NNCB".to_string(),
            "".to_string(),
            "CH -> B".to_string(),
            "HH -> N".to_string(),
        ];

        let mut expected: HashMap<(char, char), char> = HashMap::new();
        expected.insert(('C', 'H'), 'B');
        expected.insert(('H', 'H'), 'N');

        //assert_eq!(parse_input(input), ("NNCB".to_string(), expected))
    }

    #[test]
    fn test_step() {
        let input = vec![
            "NNCB".to_string(),
            "".to_string(),
            "CH -> B".to_string(),
            "HH -> N".to_string(),
            "CB -> H".to_string(),
            "NH -> C".to_string(),
            "HB -> C".to_string(),
            "HC -> B".to_string(),
            "HN -> C".to_string(),
            "NN -> C".to_string(),
            "BH -> H".to_string(),
            "NC -> B".to_string(),
            "NB -> B".to_string(),
            "BN -> B".to_string(),
            "BB -> N".to_string(),
            "BC -> B".to_string(),
            "CC -> N".to_string(),
            "CN -> C".to_string(),
        ];

        let (seed, instructions) = parse_input(input);

        let count = step(&seed, &instructions, 10);
        assert_eq!(1588, count);
    }
}
