use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    println!("Ans: {}", run(input))
}

fn run(input: &str) -> u32 {
    let mut gear_ratio_sum: u32 = 0;

    let mut gear_parts: HashMap<usize, Vec<u32>> = HashMap::from([
        // (location, <part, part, ...>),
    ]);

    let re_numbers = Regex::new(r"\d+").unwrap();
    let re_gear = Regex::new(r"\*").unwrap();
    let width = input.split("\n").collect::<Vec<&str>>()[0].len();

    for match_ in re_numbers.find_iter(input) {
        let match_range = match_.range();

        // a b c
        // d * e
        // f g h

        // a check
        if let Some(range_start) = match_range.start.checked_sub(width + 2) {
            if let Some(range_result) = input.get(
                std::ops::Range { start: range_start, end: match_range.end - width - 1 }
            ) {
                if let Some(gear) = re_gear.find(range_result) {
                    let gear_location = gear.range().start + range_start;

                    if let Some(parts) = gear_parts.get_mut(&gear_location) {
                        parts.push(match_.as_str().parse::<u32>().unwrap());
                    }
                    else {
                        gear_parts.insert(
                            gear_location, vec![match_.as_str().parse::<u32>().unwrap()]
                        );
                    }
                }
            }
        }
        // b check
        if let Some(range_start) = match_range.start.checked_sub(width + 1) {
            if let Some(range_result) = input.get(
                std::ops::Range { start: range_start, end: match_range.end - width }
            ) {
                if let Some(gear) = re_gear.find(range_result) {
                    let gear_location = gear.range().start + range_start;

                    if let Some(parts) = gear_parts.get_mut(&gear_location) {
                        parts.push(match_.as_str().parse::<u32>().unwrap());
                    }
                    else {
                        gear_parts.insert(
                            gear_location, vec![match_.as_str().parse::<u32>().unwrap()]
                        );
                    }
                }
            }
        }
        // c check
        if let Some(range_start) = match_range.start.checked_sub(width) {
            if let Some(range_result) = input.get(
                std::ops::Range { start: range_start, end: match_range.end - width }
            ) {
                if let Some(gear) = re_gear.find(range_result) {
                    let gear_location = gear.range().start + range_start;

                    if let Some(parts) = gear_parts.get_mut(&gear_location) {
                        parts.push(match_.as_str().parse::<u32>().unwrap());
                    }
                    else {
                        gear_parts.insert(
                            gear_location, vec![match_.as_str().parse::<u32>().unwrap()]
                        );
                    }
                }
            }
        }
        // d check
        if let Some(range_start) = match_range.start.checked_sub(1) {
            if let Some(range_result) = input.get(
                std::ops::Range { start: range_start, end: match_range.end }
            ) {
                if let Some(gear) = re_gear.find(range_result) {
                    let gear_location = gear.range().start + range_start;

                    if let Some(parts) = gear_parts.get_mut(&gear_location) {
                        parts.push(match_.as_str().parse::<u32>().unwrap());
                    }
                    else {
                        gear_parts.insert(
                            gear_location, vec![match_.as_str().parse::<u32>().unwrap()]
                        );
                    }
                }
            }
        }
        // e check
        if let Some(range_end) = match_range.end.checked_add(1) {
            if let Some(range_result) = input.get(
                std::ops::Range { start: match_range.start, end: range_end }
            ) {
                if let Some(gear) = re_gear.find(range_result) {
                    let gear_location = gear.range().start + match_range.start;

                    if let Some(parts) = gear_parts.get_mut(&gear_location) {
                        parts.push(match_.as_str().parse::<u32>().unwrap());
                    }
                    else {
                        gear_parts.insert(
                            gear_location, vec![match_.as_str().parse::<u32>().unwrap()]
                        );
                    }
                }
            }
        }
        // f check
        if let Some(range_end) = match_range.end.checked_add(width) {
            if let Some(range_result) = input.get(
                std::ops::Range { start: match_range.start + width, end: range_end }
            ) {
                if let Some(gear) = re_gear.find(range_result) {
                    let gear_location = gear.range().start + match_range.start + width;

                    if let Some(parts) = gear_parts.get_mut(&gear_location) {
                        parts.push(match_.as_str().parse::<u32>().unwrap());
                    }
                    else {
                        gear_parts.insert(
                            gear_location, vec![match_.as_str().parse::<u32>().unwrap()]
                        );
                    }
                }
            }
        }
        // g check
        if let Some(range_end) = match_range.end.checked_add(width + 1) {
            if let Some(range_result) = input.get(
                std::ops::Range { start: match_range.start + width, end: range_end }
            ) {
                if let Some(gear) = re_gear.find(range_result) {
                    let gear_location = gear.range().start + match_range.start + width;

                    if let Some(parts) = gear_parts.get_mut(&gear_location) {
                        parts.push(match_.as_str().parse::<u32>().unwrap());
                    }
                    else {
                        gear_parts.insert(
                            gear_location, vec![match_.as_str().parse::<u32>().unwrap()]
                        );
                    }
                }
            }
        }
        // h check
        if let Some(range_end) = match_range.end.checked_add(width + 2) {
            if let Some(range_result) = input.get(
                std::ops::Range { start: match_range.start + width + 1, end: range_end }
            ) {
                if let Some(gear) = re_gear.find(range_result) {
                    let gear_location = gear.range().start + match_range.start + width + 1;

                    if let Some(parts) = gear_parts.get_mut(&gear_location) {
                        parts.push(match_.as_str().parse::<u32>().unwrap());
                    }
                    else {
                        gear_parts.insert(
                            gear_location, vec![match_.as_str().parse::<u32>().unwrap()]
                        );
                    }
                }
            }
        }
    }

    for val in gear_parts.values() {
        let mut v = val.clone();
        v.sort();
        v.dedup();

        if v.len() == 2 {
            gear_ratio_sum += v[0] * v[1];
        }
    }

    return gear_ratio_sum
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn example_1() {
        let input: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let result: u32 = run(input);

        assert_eq!(result, 467835);
    }
}