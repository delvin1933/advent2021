use utils::read_lines;

#[derive(PartialEq, Eq, Debug)]
enum LineState {
    Incomplete(Vec<char>),
    Corrupted(char),
    Invalid,
    Valid,
}

fn match_closing_char(opening: char, closing: char) -> bool {
    match (opening, closing) {
        ('(', ')') => true,
        ('[', ']') => true,
        ('{', '}') => true,
        ('<', '>') => true,
        _ => false,
    }
}

fn get_closing_char(opening: char) -> char {
    match opening {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => '*',
    }
}

fn check_line_state(line: String) -> LineState {
    let mut opening: Vec<char> = Vec::new();

    for line_char in line.chars() {
        match line_char {
            '(' | '[' | '{' | '<' => opening.push(line_char),
            ')' | ']' | '}' | '>' => {
                match opening.pop() {
                    Some(opening) => {
                        if !match_closing_char(opening, line_char) {
                            return LineState::Corrupted(line_char);
                        }
                    }
                    None => return LineState::Incomplete(Vec::new()),
                };
            }
            _ => return LineState::Invalid,
        }
    }

    if opening.len() > 0 {
        let mut closing: Vec<char> = Vec::new();
        for item in opening {
            closing.push(get_closing_char(item))
        }
        closing.reverse();
        return LineState::Incomplete(closing);
    }
    LineState::Valid
}

fn get_char_points(input: char) -> i32 {
    match input {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn get_closing_point(closing: Vec<char>) -> i64 {
    let mut score: i64 = 0;

    for input in closing {
        score = score * 5;
        score = score + get_char_points2(input);
    }

    score
}

fn get_char_points2(input: char) -> i64 {
    match input {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn part1(lines: Vec<String>) -> (i32, i64) {
    let mut score_part1 = 0;
    let mut score_part2: Vec<i64> = Vec::new();
    for line in lines {
        match check_line_state(line) {
            LineState::Corrupted(invalid_char) => score_part1 += get_char_points(invalid_char),
            LineState::Incomplete(ending) => score_part2.push(get_closing_point(ending)),
            _ => (),
        }
    }

    score_part2.sort();

    let middle_index = (score_part2.len() as i32 / 2) as usize;

    let score_2: i64 = score_part2[middle_index];

    (score_part1, score_2)
}

fn main() {
    let lines = read_lines("./input.txt").unwrap_or_default();
    let (part1, part2) = part1(lines);
    println!("Part 1 : {}, Part2 : {}", part1, part2);
}

mod tests {
    use super::*;

    #[test]
    fn test_line_corrupted() {
        let line: String = "[({(<(())[]>[[{[]{<()<>>".to_string();
        assert_eq!(
            LineState::Incomplete(vec!['}', '}', ']', ']', ')', '}', ')', ']']),
            check_line_state(line)
        );

        let line: String = "{([(<{}[<>[]}>{[]{[(<()>".to_string();
        assert_eq!(LineState::Corrupted('}'), check_line_state(line));

        let line: String = "[<>({}){}[([])<>]]".to_string();
        assert_eq!(LineState::Valid, check_line_state(line));

        let line: String = "[<>({}){}[([])<>]])".to_string();
        assert_eq!(LineState::Incomplete(Vec::new()), check_line_state(line));

        let line: String = "[<>({}){}[([])<>]]*".to_string();
        assert_eq!(LineState::Invalid, check_line_state(line));
    }

    #[test]
    fn test_closing_score() {
        let chars = vec![']', ')', '}', '>'];

        assert_eq!(294, get_closing_point(chars));

        let chars = vec!['}', '}', ']', ']', ')', '}', ')', ']'];

        assert_eq!(288957, get_closing_point(chars));
    }

    #[test]
    fn test_part1() {
        let lines: Vec<String> = vec![
            "[({(<(())[]>[[{[]{<()<>>".to_string(),
            "[(()[<>])]({[<{<<[]>>(".to_string(),
            "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
            "(((({<>}<{<{<>}{[]{[]{}".to_string(),
            "[[<[([]))<([[{}[[()]]]".to_string(),
            "[{[{({}]{}}([{[{{{}}([]".to_string(),
            "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
            "[<(<(<(<{}))><([]([]()".to_string(),
            "<{([([[(<>()){}]>(<<{{".to_string(),
            "<{([{{}}[<[[[<>{}]]]>[]]".to_string(),
        ];

        let (part1, part2) = part1(lines);
        assert_eq!(26397, part1);
        assert_eq!(288957, part2);
    }
}
