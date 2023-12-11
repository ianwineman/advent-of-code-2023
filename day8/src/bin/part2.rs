use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    println!("Ans: {}", run(input))
}

fn run(input: &str) -> u32 {
    let mut steps: u32 = 0;
    let lines: Vec<&str> = input.lines().collect();
    let turns: Vec<char> = lines[0].chars().collect();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut current_node: Vec<&str> = Vec::new();

    let re = Regex::new(r"(?<node>[A-Z0-9]{3})\s=\s\((?<L>[A-Z0-9]{3}),\s(?<R>[A-Z0-9]{3})").unwrap();
    for line in &lines {
        if let Some(match_) = re.captures(line) {
            let node: &str = match_.name("node").unwrap().as_str();

            map.insert(node, (match_.name("L").unwrap().as_str(), match_.name("R").unwrap().as_str()));

            if node.chars().last().unwrap() == 'A' {
                current_node.push(node)
            }
        }
    }

    let turn_number: usize = turns.len();
    let mut turn_counter: usize = 0;

    loop {
        let turn = turns[turn_counter];

        if turn == 'L' {
            let mut tmp = Vec::new();

            for node in current_node {
                tmp.push(map.get(node).unwrap().0)
            }

            current_node = tmp;
        }
        if turn == 'R' {
            let mut tmp = Vec::new();

            for node in current_node {
                tmp.push(map.get(node).unwrap().1)
            }

            current_node = tmp;
        }

        steps += 1;

        let mut endings: Vec<char> = current_node.clone().into_iter().map(|x| x.chars().last().unwrap()).collect();
        endings.sort();
        let test = endings.into_iter().collect::<String>();

        if test.contains("Z") {
            println!("{:?}", test);
        }

        if test.starts_with("Z") {
            return steps
        }
        else {
            if turn_counter == turn_number - 1 {
                turn_counter = 0;
            }
            else {
                turn_counter += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn example_1() {
        let input: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let result: u32 = run(input);

        assert_eq!(result, 6);
    }
}