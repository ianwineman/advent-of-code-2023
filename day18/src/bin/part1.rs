fn main() {
    let input: &str = "";

    println!("Ans: {}", run(input))
}

fn run(input: &str) -> u32 {
    let moves: Vec<(&str, u32)> = input.lines().map(|x| {
        let parts = x.split(" ").collect::<Vec<&str>>();
        (parts[0], parts[1].parse::<u32>().unwrap())
    }).collect();

    let size: u32 = moves.into_iter().fold(0, |acc, x| acc + x.1);

    let grid: Vec<&str> = Vec::new();


    println!("{:?}", moves);
    println!("{}", moves.into_iter().fold(0, |acc, x| acc + x.1));

    return 0
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn example_1() {
        let input: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

        let result: u32 = run(input);

        assert_eq!(result, 62);
    }
}