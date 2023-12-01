fn main() {
    let input: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    println!("Ans: {}", run(input))
}

fn run(input: &str) -> u32 {
    let mut calibration_value_sum: u32 = 0;

    let lines: Vec<&str> = input.lines().collect::<Vec<&str>>();

    for line in lines {
        let mut chars = line.chars();
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
        let input: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let result: u32 = run(input);

        assert_eq!(result, 142);
    }
}