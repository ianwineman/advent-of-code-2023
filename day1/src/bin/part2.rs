use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    println!("Ans: {}", run(input))
}

fn run(input: &str) -> u32 {
    let numbers = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let numbers_reverse = HashMap::from([
        ("eno", "1"),
        ("owt", "2"),
        ("eerht", "3"),
        ("ruof", "4"),
        ("evif", "5"),
        ("xis", "6"),
        ("neves", "7"),
        ("thgie", "8"),
        ("enin", "9"),
    ]);

    let mut calibration_value_sum: u32 = 0;

    let lines: Vec<&str> = input.lines().collect::<Vec<&str>>();

    for line in lines {
        let re = Regex::new(
            r"(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)"
        ).unwrap();

        let re_reverse = Regex::new(
            r"(eno)|(owt)|(eerht)|(ruof)|(evif)|(xis)|(neves)|(thgie)|(enin)"
        ).unwrap();

        let mut new_line = String::from(line);

        // replace first number
        if re.is_match(&new_line) {
            let first_number = re.find(&new_line).unwrap().as_str();
            new_line = new_line.replacen(first_number, numbers.get(&first_number).unwrap(), 1);
        }

        // replace last number
        let new_line_reversed = &new_line.chars().rev().collect::<String>();
        if re_reverse.is_match(&new_line_reversed) {
            let last_number = re_reverse.find(&new_line_reversed).unwrap().as_str();
            new_line = new_line_reversed.replacen(last_number, numbers_reverse.get(&last_number).unwrap(), 1).chars().rev().collect::<String>();
        }

        let mut chars = new_line.as_str().chars();
        let mut digits: Vec<char> = Vec::new();

        for _c in 1..=chars.clone().count() {
            let char = chars.next().unwrap();

            if char.is_numeric() {
                digits.push(char);
            }
        }

        let first_digit: char = digits[0];
        let last_digit: char = digits[digits.len() - 1];

        let mut calibration_value = String::from("");
        calibration_value.push(first_digit);
        calibration_value.push(last_digit);

        calibration_value_sum += calibration_value.parse::<u32>().unwrap();
    }

    return calibration_value_sum
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn example_1() {
        let input: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let result: u32 = run(input);

        assert_eq!(result, 281);
    }

    #[test]
    fn example_2() {
        let input: &str = "oneoneight";

        let result: u32 = run(input);

        assert_eq!(result, 18);
    }
}