use regex::Regex;

fn main() {
    let input: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    println!("Ans: {}", run(input))
}

fn counter(set: &str) -> (u32,u32,u32) {
    let mut red: u32 = 0;
    let mut green: u32 = 0;
    let mut blue: u32 = 0;

    let red_re = Regex::new(r"\d+\sred").unwrap();
    let green_re = Regex::new(r"\d+\sgreen").unwrap();
    let blue_re = Regex::new(r"\d+\sblue").unwrap();

    if let Some(match_) = red_re.find(set) {
        red += match_
                 .as_str()
                 .split(" ")
                 .collect::<Vec<&str>>()[0]
                 .parse::<u32>()
                 .unwrap();
    }
    if let Some(match_) = green_re.find(set) {
        green += match_
                   .as_str()
                   .split(" ")
                   .collect::<Vec<&str>>()[0]
                   .parse::<u32>()
                   .unwrap();
    }
    if let Some(match_) = blue_re.find(set) {
        blue += match_
                  .as_str()
                  .split(" ")
                  .collect::<Vec<&str>>()[0]
                  .parse::<u32>()
                  .unwrap();
    }

    return (red, green, blue)
}

fn run(input: &str) -> u32 {
    let mut game_id_sum: u32 = 0;

    let lines: Vec<&str> = input.lines().collect::<Vec<&str>>();

    for line in lines {
        let game_id: &str = line
                              .split(":")
                              .collect::<Vec<&str>>()[0]
                              .split(" ")
                              .collect::<Vec<&str>>()[1];

        let sets_parsed: Vec<(u32,u32,u32)> = line
                                           .split(":")
                                           .collect::<Vec<&str>>()[1]
                                           .split(";")
                                           .collect::<Vec<&str>>()
                                           .into_iter()
                                           .map(|x| counter(x))
                                           .collect::<Vec<(u32,u32,u32)>>();

        let mut max: (u32,u32,u32) = (0,0,0);
        for set in sets_parsed {
            if set.0 > max.0 {
                max.0 = set.0
            }
            if set.1 > max.1 {
                max.1 = set.1
            }
            if set.2 > max.2 {
                max.2 = set.2
            }
        }

        if max.0 <= 12 && max.1 <= 13 && max.2 <= 14 {
            game_id_sum += game_id.parse::<u32>().unwrap();
        }
    }

    return game_id_sum
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn example_1() {
        let input: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result: u32 = run(input);

        assert_eq!(result, 8);
    }
}