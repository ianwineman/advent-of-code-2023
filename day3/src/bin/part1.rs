use regex::Regex;

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
    let mut part_number_sum: u32 = 0;

    let re_numbers = Regex::new(r"\d+").unwrap();
    let re_symbols = Regex::new(r"@|\+|#|\$|-|&|/|\*|%|=").unwrap();
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
                if re_symbols.is_match(range_result) {
                    part_number_sum += match_.as_str().parse::<u32>().unwrap();
                    continue;
                }
            }
        }
        // b check
        if let Some(range_start) = match_range.start.checked_sub(width + 1) {
            if let Some(range_result) = input.get(
                std::ops::Range { start: range_start, end: match_range.end - width }
            ) {
                if re_symbols.is_match(range_result) {
                    part_number_sum += match_.as_str().parse::<u32>().unwrap();
                    continue;
                }
            }
        }
        // c check
        if let Some(range_start) = match_range.start.checked_sub(width) {
            if let Some(range_result) = input.get(
                std::ops::Range { start: range_start, end: match_range.end - width }
            ) {
                if re_symbols.is_match(range_result) {
                    part_number_sum += match_.as_str().parse::<u32>().unwrap();
                    continue;
                }
            }
        }
        // d check
        if let Some(range_start) = match_range.start.checked_sub(1) {
            if let Some(range_result) = input.get(
                std::ops::Range { start: range_start, end: match_range.end }
            ) {
                if re_symbols.is_match(range_result) {
                    part_number_sum += match_.as_str().parse::<u32>().unwrap();
                    continue;
                }
            }
        }
        // e check
        if let Some(range_end) = match_range.end.checked_add(1) {
            if let Some(range_result) = input.get(
                std::ops::Range { start: match_range.start, end: range_end }
            ) {
                if re_symbols.is_match(range_result) {
                    part_number_sum += match_.as_str().parse::<u32>().unwrap();
                    continue;
                }
            }
        }
        // f check
        if let Some(range_end) = match_range.end.checked_add(width) {
            if let Some(range_result) = input.get(
                std::ops::Range { start: match_range.start + width, end: range_end }
            ) {
                if re_symbols.is_match(range_result) {
                    part_number_sum += match_.as_str().parse::<u32>().unwrap();
                    continue;
                }
            }
        }
        // g check
        if let Some(range_end) = match_range.end.checked_add(width + 1) {
            if let Some(range_result) = input.get(
                std::ops::Range { start: match_range.start + width, end: range_end }
            ) {
                if re_symbols.is_match(range_result) {
                    part_number_sum += match_.as_str().parse::<u32>().unwrap();
                    continue;
                }
            }
        }
        // h check
        if let Some(range_end) = match_range.end.checked_add(width + 2) {
            if let Some(range_result) = input.get(
                std::ops::Range { start: match_range.start + width + 1, end: range_end }
            ) {
                if re_symbols.is_match(range_result) {
                    part_number_sum += match_.as_str().parse::<u32>().unwrap();
                    continue;
                }
            }
        }
    }

    return part_number_sum
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

        assert_eq!(result, 4361);
    }
}