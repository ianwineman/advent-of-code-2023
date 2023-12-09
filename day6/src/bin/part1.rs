use regex::Regex;

fn main() {
    let input: &str = "Time:      7  15   30
Distance:  9  40  200";

    println!("Ans: {}", run(input))
}

fn run(input: &str) -> usize {
    let mut error_margin_list: Vec<usize> = Vec::new();

    let re = Regex::new(r"(\d+)").unwrap();

    let input_lines: Vec<&str> = input.lines().collect();
    let time_list: Vec<u32> = re.find_iter(input_lines[0]).map(|m| m.as_str().parse::<u32>().unwrap()).collect();
    let distance_list: Vec<u32> = re.find_iter(input_lines[1]).map(|m| m.as_str().parse::<u32>().unwrap()).collect();

    for i in 0..time_list.len() {
        let race_record: u32 = distance_list[i];
        let mut pr_list: Vec<u32> = Vec::new();

        for j in 0..time_list[i] {
            let speed: u32 = j;
            let total_distance: u32 = speed * (time_list[i] - j);

            if total_distance > race_record {
                pr_list.push(total_distance);
            }
        }

        error_margin_list.push(pr_list.len());
    }

    return error_margin_list.into_iter().reduce(|x, y| x * y).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn example_1() {
        let input: &str = "Time:      7  15   30
Distance:  9  40  200";

        let result: usize = run(input);

        assert_eq!(result, 288);
    }
}