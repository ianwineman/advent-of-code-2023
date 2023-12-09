use regex::Regex;

fn main() {
    let input: &str = "Time:        46     80     78     66
Distance:   214   1177   1402   1024";

    println!("Ans: {}", run(input))
}

fn run(input: &str) -> usize {
    let re = Regex::new(r"(\d+)").unwrap();

    let input_lines: Vec<&str> = input.lines().collect();
    let time_list: Vec<&str> = re.find_iter(input_lines[0]).map(|m| m.as_str()).collect();
    let distance_list: Vec<&str> = re.find_iter(input_lines[1]).map(|m| m.as_str()).collect();

    let mut time: String = String::new();
    let mut distance: String = String::new();

    for i in 0..time_list.len() {
        time += time_list[i];
        distance += distance_list[i];
    }

    let race_record: usize = distance.parse::<usize>().unwrap();
    let mut pr_list: Vec<usize> = Vec::new();

    for j in 0..time.parse::<usize>().unwrap() {
        let speed: usize = j;
        let total_distance: usize = speed * (time.parse::<usize>().unwrap() - j);

        if total_distance > race_record {
            pr_list.push(total_distance);
        }
    }

    return pr_list.len()
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn example_1() {
        let input: &str = "Time:      7  15   30
Distance:  9  40  200";

        let result: usize = run(input);

        assert_eq!(result, 71503);
    }
}