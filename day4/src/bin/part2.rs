use regex::Regex;
use std::collections::{HashSet, HashMap};

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
    let mut card_count: u32 = 0;
    let mut card_multiplier = HashMap::from([
        (1_u32, 1_u32) // (card_number, multiplier)
    ]);

    let cards = input.lines().collect::<Vec<&str>>();

    let re = Regex::new(r"Card\s+(?<card>\d+):\s(?<win>(\s*\d+)+)\s\|\s(?<yours>(\s*\d+)+)").unwrap();

    for card in cards {
        let mut card_score: u32 = 0;

        let cap = re.captures(card).unwrap();
        let card_number: u32 = cap.name("card").unwrap().as_str().parse::<u32>().unwrap();
        card_multiplier.insert(card_number, 1);

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

        for i in 1..card_score + 1 {
            if let Some(multiplier) = card_multiplier.get_mut(&(i + card_number)) {
                let current_multiplier = card_multiplier.get(&(card_number)).unwrap();
                card_multiplier.insert(i + card_number, *current_multiplier);
            }
            else {
                let current_multiplier = card_multiplier.get(&(card_number)).unwrap();
                card_multiplier.insert(
                    card_number + i, *current_multiplier
                );
            }
        }
    }

    // calculate card count by iterating over hashmap
    println!("{:?}", card_multiplier);
    return card_count
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

        assert_eq!(result, 30);
    }
}