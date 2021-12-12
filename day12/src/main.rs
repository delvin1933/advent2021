use std::collections::{HashMap, HashSet};

fn parse_line(input: String) -> (String, String) {
    let mut split_input = input.split("-");

    let start = split_input.next().unwrap().to_string();

    let end = split_input.next().unwrap().to_string();

    (start, end)
}

type Graph = HashMap<String, Vec<String>>;

fn parse_input(input: Vec<String>) -> Graph {
    let mut cave: Graph = HashMap::new();

    for line in input {
        let (begin_str, end_str) = parse_line(line);

        match cave.get_mut(&begin_str.clone()) {
            Some(value) => {
                if end_str != "start".to_string() {
                    value.push(end_str.clone());
                }
            }
            None => {
                if end_str != "start".to_string() {
                    cave.insert(begin_str.clone(), vec![end_str.clone()]);
                }
            }
        };

        match cave.get_mut(&end_str) {
            Some(value) => {
                if begin_str != "start".to_string() {
                    value.push(begin_str);
                }
            }
            None => {
                if begin_str != "start".to_string() {
                    let _ = cave.insert(end_str, vec![begin_str]);
                }
            }
        };
    }

    cave
}

fn is_lower(input: &String) -> bool {
    for c in input.chars() {
        if c.is_uppercase() {
            return false;
        }
    }
    true
}

fn calc_path(graph: &Graph, visited: HashSet<String>, current_node: &String, part2: bool) -> i32 {
    let mut result: i32 = 0;

    if current_node == "end" {
        return 1;
    }

    for node in graph.get(current_node).unwrap() {
        let mut sub_visited = visited.clone();
        if is_lower(node) {
            sub_visited.insert(node.clone());
            if !&visited.contains(node) {
                result = result + calc_path(&graph, sub_visited, &node, part2);
            } else if part2 {
                result = result + calc_path(&graph, sub_visited, &node, false);
            }
        } else {
            result = result + calc_path(&graph, sub_visited, &node, part2);
        }
    }
    result
}

fn main() {
    let input: Vec<String> = vec![
        "yb-pi".to_string(),
        "jg-ej".to_string(),
        "yb-KN".to_string(),
        "LD-start".to_string(),
        "end-UF".to_string(),
        "UF-yb".to_string(),
        "yb-xd".to_string(),
        "qx-yb".to_string(),
        "xd-end".to_string(),
        "jg-KN".to_string(),
        "start-qx".to_string(),
        "start-ej".to_string(),
        "qx-LD".to_string(),
        "jg-LD".to_string(),
        "xd-LD".to_string(),
        "ej-qx".to_string(),
        "end-KN".to_string(),
        "DM-xd".to_string(),
        "jg-yb".to_string(),
        "ej-LD".to_string(),
        "qx-UF".to_string(),
        "UF-jg".to_string(),
        "qx-jg".to_string(),
        "xd-UF".to_string(),
    ];

    let mut start_set: HashSet<String> = HashSet::new();

    let input = &parse_input(input);
    start_set.insert("start".to_string());
    let res = calc_path(input, start_set.clone(), &"start".to_string(), false);
    println!("Part 1 : {}", res);

    let res = calc_path(input, start_set, &"start".to_string(), true);
    println!("Part 2 : {}", res);
}

mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let input = "fs-end".to_string();
        assert_eq!(("fs".to_string(), "end".to_string()), parse_line(input));
    }

    #[test]
    fn test_parse_input() {
        //let input: Vec<String> = vec!["start-A".to_string(), "A-end".to_string()];

        let mut expected: HashMap<String, Vec<String>> = HashMap::new();
        expected.insert("start".to_string(), vec!["A".to_string()]);
        expected.insert(
            "A".to_string(),
            vec!["start".to_string(), "end".to_string()],
        );
        expected.insert("end".to_string(), vec!["A".to_string()]);
    }

    #[test]
    fn test_calc_input() {
        let input: Vec<String> = vec![
            "dc-end".to_string(),
            "HN-start".to_string(),
            "start-kj".to_string(),
            "dc-start".to_string(),
            "dc-HN".to_string(),
            "LN-dc".to_string(),
            "HN-end".to_string(),
            "kj-sa".to_string(),
            "kj-HN".to_string(),
            "kj-dc".to_string(),
        ];

        let input2 = vec![
            "start-A".to_string(),
            "start-b".to_string(),
            "A-c".to_string(),
            "A-b".to_string(),
            "b-d".to_string(),
            "A-end".to_string(),
            "b-end".to_string(),
        ];

        let mut start_set: HashSet<String> = HashSet::new();
        start_set.insert("start".to_string());

        let res = calc_path(
            &parse_input(input2.clone()),
            start_set.clone(),
            &"start".to_string(),
            false,
        );
        assert_eq!(10, res);

        let res = calc_path(
            &parse_input(input),
            start_set.clone(),
            &"start".to_string(),
            false,
        );
        assert_eq!(19, res);

        let res = calc_path(
            &parse_input(input2.clone()),
            start_set.clone(),
            &"start".to_string(),
            true,
        );
        assert_eq!(36, res);
        //assert_eq!(10, calc_path_from_start(&parse_input(input2)));
        //assert_eq!(19, calc_path_from_start(&parse_input(input)));
    }
}
