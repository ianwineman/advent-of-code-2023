use regex::Regex;
use std::collections::HashSet;

fn main() {
    let input: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    println!("Ans: {}", run(input))
}

fn run(input: &str) -> u32 {
    let mut card_point_sum: u32 = 0;

    let lines = input.lines().collect::<Vec<&str>>();

    let re = Regex::new(r"Card\s+\d+:\s(?<win>(\s*\d+)+)\s\|\s(?<yours>(\s*\d+)+)").unwrap();

    for line in lines {
        let mut card_score: u32 = 0;

        let cap = re.captures(line).unwrap();
        let win: Vec<&str> = cap.name("win").unwrap().as_str().split(" ").collect::<Vec<&str>>();
        let yours: Vec<&str> = cap.name("yours").unwrap().as_str().split(" ").collect::<Vec<&str>>();

        let mut win_set: HashSet<&str> = HashSet::with_capacity(win.len());
        let mut yours_set: HashSet<&str> = HashSet::with_capacity(yours.len());

        for i in win.into_iter() {
            win_set.insert(i);
        }
        for i in yours.into_iter() {
            yours_set.insert(i);
        }

        let common_set = win_set.intersection(&yours_set);

        for i in common_set.into_iter() {
            if i != &"" {
                if card_score == 0 {
                    card_score += 1;
                }
                else {
                    card_score *= 2;
                }
            }
        }

        card_point_sum += card_score;
    }

    return card_point_sum
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn example_1() {
        let input: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result: u32 = run(input);

        assert_eq!(result, 13);
    }
}