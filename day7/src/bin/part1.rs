use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let input: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    println!("Ans: {}", run(input))
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum HandType {
    FiveKind, // an element with 5 occurences
    FourKind, // an element with 4 occurrences
    FullHouse, // an element with 3 occurences and another with 2 occurrences
    ThreeKind, // an element with 3 occurences
    TwoPair, // an element with 2 occurences and another with 2 occurences
    OnePair, // an element with 2 occurences
    HighCard, // else
}

#[derive(Debug, Copy, Clone)]
struct Hand<'a> {
    cards: &'a str,
    bet: usize,
    hand_type: HandType,
}

impl Hand<'_> {
    fn from(x: (&str, usize)) -> Hand {
        let card_vec: Vec<char> = x.0.chars().collect();
        let mut card_map: HashMap<char, u32> = HashMap::new();

        for card in &card_vec {
            if let Some(count) = card_map.get_mut(&card) {
                *count += 1;
            }
            else {
                card_map.insert(*card, 1);
            }
        }

        let mut card_map_values = card_map.values().collect::<Vec<&u32>>();
        card_map_values.sort();

        if card_map_values.contains(&&5_u32) {
            return Hand {
                cards: x.0,
                bet: x.1,
                hand_type: HandType::FiveKind,
            }
        }
        else if card_map_values.contains(&&4_u32) {
            return Hand {
                cards: x.0,
                bet: x.1,
                hand_type: HandType::FourKind,
            }
        }
        else if card_map_values.contains(&&3_u32) && card_map_values.contains(&&2_u32) {
            return Hand {
                cards: x.0,
                bet: x.1,
                hand_type: HandType::FullHouse,
            }
        }
        else if card_map_values.contains(&&3_u32) {
            return Hand {
                cards: x.0,
                bet: x.1,
                hand_type: HandType::ThreeKind,
            }
        }
        else if card_map_values == Vec::from([&1_u32,&2_u32,&2_u32]) {
            return Hand {
                cards: x.0,
                bet: x.1,
                hand_type: HandType::TwoPair,
            }
        }
        else if card_map_values.contains(&&2_u32) {
            return Hand {
                cards: x.0,
                bet: x.1,
                hand_type: HandType::OnePair,
            }
        }
        else {
            return Hand {
                cards: x.0,
                bet: x.1,
                hand_type: HandType::HighCard,
            }
        }
    }

    fn rank(&self, y: Hand) -> Ordering {
        let type_ranks: HashMap<HandType, u32> = HashMap::from([
            (HandType::FiveKind, 7),
            (HandType::FourKind, 6),
            (HandType::FullHouse, 5),
            (HandType::ThreeKind, 4),
            (HandType::TwoPair, 3),
            (HandType::OnePair, 2),
            (HandType::HighCard, 1),
        ]);

        let card_ranks: HashMap<char, u32> = HashMap::from([
            ('A', 13),
            ('K', 12),
            ('Q', 11),
            ('J', 10),
            ('T', 9),
            ('9', 8),
            ('8', 7),
            ('7', 6),
            ('6', 5),
            ('5', 4),
            ('4', 3),
            ('3', 2),
            ('2', 1),
        ]);

        if type_ranks.get(&self.hand_type).unwrap() > type_ranks.get(&y.hand_type).unwrap() {
            return Ordering::Greater
        }
        else if type_ranks.get(&self.hand_type).unwrap() < type_ranks.get(&y.hand_type).unwrap() {
            return Ordering::Less
        }
        else {
            let mut self_chars = self.cards.chars();
            let mut y_chars = y.cards.chars();

            for _i in 0..self.cards.len() {
                let self_chars_next = self_chars.next().unwrap();
                let y_chars_next = y_chars.next().unwrap();

                if card_ranks.get(&self_chars_next).unwrap() > card_ranks.get(&y_chars_next).unwrap() {
                    return Ordering::Greater
                }
                else if card_ranks.get(&self_chars_next).unwrap() < card_ranks.get(&y_chars_next).unwrap() {
                    return Ordering::Less
                }
                else {
                    continue;
                }
            }
            return Ordering::Equal
        }
    }
}

fn run(input: &str) -> usize {
    let mut winnings: usize = 0;

    let mut hands: Vec<Hand> = input
        .lines()
        .collect::<Vec<&str>>()
        .into_iter()
        .map(
            |x| {
                let y: Vec<&str> = x.split(" ").collect();
                Hand::from((y[0], y[1].parse::<usize>().unwrap()))
            }
        )
        .collect();

    hands.sort_by(|x, y| x.rank(*y));

    let mut rank: usize = 1;

    for hand in hands {
        winnings += rank * hand.bet;
        rank += 1;
    }

    return winnings
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn example_1() {
        let input: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let result: usize = run(input);

        assert_eq!(result, 6440);
    }
}