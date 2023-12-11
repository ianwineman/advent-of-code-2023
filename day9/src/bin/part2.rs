fn main() {
    let input: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    println!("Ans: {}", run(input))
}

fn run(input: &str) -> i32 {
    let mut predicted_sum: i32 = 0;

    for line in input.lines() {
        let mut readings: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();

        let mut all_rows: Vec<Vec<i32>> = Vec::new();
        all_rows.push(readings.clone());

        loop {
            let mut tmp_readings: Vec<i32> = Vec::new();

            for i in 0..(readings.len() - 1) {
                tmp_readings.push( readings[i+1] - readings[i] )

            }

            all_rows.push(tmp_readings.clone());

            if tmp_readings == vec![0; tmp_readings.len()] {
                break;
            }
            else {
                readings = tmp_readings;
            }
        }

        let mut prediction = 0;

        all_rows.reverse();
        for i in 0..(all_rows.len() - 1) {
            prediction = all_rows[i+1][0] - prediction;
        }

        predicted_sum += prediction;
    }

    return predicted_sum
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn example_1() {
        let input: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let result: i32 = run(input);

        assert_eq!(result, 2);
    }
}