use std::char;
use std::collections::HashSet;
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

#[derive(Debug, Clone)]
struct Signal {
    pattern: Vec<String>,
    digit: Vec<String>,
    segments: Vec<String>,
    value: i32,
}

impl Signal {
    fn new(pattern: Vec<String>, digit: Vec<String>) -> Self {
        let mut sig = Signal {
            pattern,
            digit,
            segments: Vec::new(),
            value: 0,
        };
        sig.compute_segments();
        sig.compute_number();
        sig
    }

    /*
          0:      1:      2:      3:      4:
         aaaa    ....    aaaa    aaaa    ....
        b    c  .    c  .    c  .    c  b    c
        b    c  .    c  .    c  .    c  b    c
         ....    ....    dddd    dddd    dddd
        e    f  .    f  e    .  .    f  .    f
        e    f  .    f  e    .  .    f  .    f
         gggg    ....    gggg    gggg    ....

          5:      6:      7:      8:      9:
         aaaa    aaaa    aaaa    aaaa    aaaa
        b    .  b    .  .    c  b    c  b    c
        b    .  b    .  .    c  b    c  b    c
         dddd    dddd    ....    dddd    dddd
        .    f  e    f  .    f  e    f  .    f
        .    f  e    f  .    f  e    f  .    f
         gggg    gggg    ....    gggg    gggg


    0 : 6 abcefg
    1 : 2 cf
    2 : 5 acdeg
    3 : 5 acdfg
    4 : 4 bcdf
    5 : 5 abdfg
    6 : 6 abdefg
    7 : 3 acf
    8 : 7 abcdefg
    9 : 6 abcdfg


    1 : 2 cf
    7 : 3 acf
    4 : 4 bcdf
    2 : 5 acdeg
    3 : 5 acdfg
    5 : 5 abdfg
    6 : 6 abdefg
    0 : 6 abcefg
    9 : 6 abcdfg
    8 : 7 abcdefg

        */

    fn compute_segments(&mut self) {
        let one = &self.pattern[0];
        let seven = &self.pattern[1];
        let four = &self.pattern[2];
        let eight = &self.pattern[9];

        let mut zero: String = "".to_string();
        let mut two: String = "".to_string();
        let mut three: String = "".to_string();
        let mut five: String = "".to_string();
        let mut six: String = "".to_string();
        let mut nine: String = "".to_string();

        for i in 3..=5 {
            let tmp = &self.pattern[i];
            if contains(one, tmp) {
                three = tmp.to_string();
            } else if count_commons(four, tmp) == 2 {
                five = tmp.to_string();
            } else {
                two = tmp.to_string();
            }
        }

        for i in 6..=8 {
            let tmp = &self.pattern[i];
            if contains(four, tmp) {
                nine = tmp.to_string();
            }
            //reste 6 et 0
            else if contains(one, tmp) {
                zero = tmp.to_string();
            } else {
                six = tmp.to_string();
            }
        }
        self.segments.push(zero);
        self.segments.push(one.to_string());
        self.segments.push(two);
        self.segments.push(three);
        self.segments.push(four.to_string());
        self.segments.push(five.to_string());
        self.segments.push(six);
        self.segments.push(seven.to_string());
        self.segments.push(eight.to_string());
        self.segments.push(nine.to_string());
    }

    fn compute_number(&mut self) {
        let mut in_str: String = "".to_string();
        for di in &self.digit {
            for i in 0..=9 {
                if count_commons(&self.segments[i], di) == 0 {
                    in_str = format!("{}{}", in_str, i.to_string());
                }
            }
        }
        self.value = in_str.parse::<i32>().unwrap();
    }
}

fn contains(a: &String, b: &String) -> bool {
    // get which one is shorter
    let (shorter, longer) = if b.len() > a.len() { (a, b) } else { (b, a) };

    // fill the set with the characters from the shorter string
    let set: HashSet<char> = longer.chars().collect();

    shorter.chars().all(|c| set.contains(&c))
}

fn count_commons(a: &String, b: &String) -> i32 {
    let (shorter, longer) = if a.len() > b.len() { (b, a) } else { (a, b) };

    let mut tmp = longer.clone();

    for mychar in shorter.chars() {
        tmp = tmp.replace(mychar, "");
        //println!("{:?}", tmp);
    }

    tmp.len() as i32
}

fn parse_entries(input: Vec<String>) -> Vec<Signal> {
    let mut result: Vec<Signal> = Vec::new();
    for line in input {
        println!("Line {:?}", line);
        let mut parts = line.split(" | ").into_iter();

        let mut pattern: Vec<String> = if let Some(i) = parts.next() {
            i.split(" ").map(|s| s.to_string()).collect::<Vec<String>>()
        } else {
            vec![]
        };
        pattern.sort_by(|a, b| a.len().cmp(&b.len()));
        println!("PATTERN : {:?}", pattern);

        let digit: Vec<String> = if let Some(i) = parts.next() {
            i.split(" ").map(|s| s.to_string()).collect()
        } else {
            vec![]
        };
        // println!("Pattern {:?}", pattern);
        // println!("Digit {:?}", digit);

        result.push(Signal::new(pattern, digit))
    }

    return result;
}

fn part1(signals: Vec<Signal>) -> u32 {
    let mut part1 = 0;
    for signal in signals {
        let filtered: Vec<&String> = signal
            .digit
            .iter()
            .filter(|d| d.len() == 2 || d.len() == 4 || d.len() == 7 || d.len() == 3)
            .collect();
        println!("{:?}", filtered);
        part1 += filtered.len();
    }
    part1 as u32
}

fn main() {
    let lines = read_lines("./input.txt").unwrap_or_default();

    let signals = parse_entries(lines);

    println!("Part1 : {}", part1(signals.clone()));

    println!("Part2 : {}", signals.iter().map(|t| t.value).sum::<i32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality() {
        let test_data:Vec<String> = vec!["be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".into(),
                                         "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".into(),
                                         "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".into(),
                                         "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb".into(),
                                         "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea".into(),
                                         "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb".into(),
                                         "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe".into(),
                                         "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef".into(),
                                         "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".into(),
                                         "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".into()];

        let test = parse_entries(test_data.clone());

        assert_eq!(26, part1(test));

        let test = parse_entries(test_data);
        println!("{:?}", &test.iter().map(|t| t));
        assert_eq!(61229, test.iter().map(|t| t.value).sum())
    }

    #[test]
    fn test_share_chars() {
        let three = "acdfg".to_string();
        let one = "cf".to_string();
        let two = "acdeg".to_string();

        assert_eq!(true, contains(&one, &three));
        assert_eq!(false, contains(&one, &two));
    }

    #[test]
    fn test_count_commons() {
        let five = "abdfg".to_string();
        let four = "bcdf".to_string();
        let two = "abdeg".to_string();

        assert_eq!(2, count_commons(&four, &five));
        assert_eq!(3, count_commons(&four, &two));
        assert_eq!(0, count_commons(&two, &two));
    }
}
