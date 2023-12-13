fn main() {
    let input: &str = "";

    println!("Ans: {}", run(input))
}

fn run(input: &str) -> u32 {

    return 0
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn example_1() {
        let input: &str = "";

        let result: u32 = run(input);

        assert_eq!(result, 374);
    }
}