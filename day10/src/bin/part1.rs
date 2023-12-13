fn main() {
    let input: &str = "";

    println!("Ans: {}", run(input))
}

fn run(input: &str) -> usize {
    let line_len: usize = input.lines().collect::<Vec<&str>>()[0].len();
    println!("{}", line_len);

    let mut active_paths: Vec<Vec<usize>> = Vec::new();
    let mut completed_path_lens: Vec<usize> = Vec::new();

    let mut current_position: usize = input.find('S').unwrap();
    println!("{:?} {:?}", current_position, input.get(std::ops::Range { start: current_position, end: current_position + 1 }).unwrap());
    active_paths.push(Vec::from([current_position]));

    'outer: loop {
        println!("{:?}", active_paths);
        for i in 0..(active_paths.len()) {
            let path = active_paths[i].clone();
            let mut new_steps: Vec<usize> = Vec::new();
            current_position = *path.last().unwrap();

            // N
            if let Some(tile) = input.get(std::ops::Range { start: current_position - line_len, end: current_position - line_len + 1 }) {
                match tile {
                    "|" | "F" | "7" => {
                        new_steps.push(current_position - line_len);
                    },
                    _ => (),
                }
            }

            // S
            if let Some(tile) = input.get(std::ops::Range { start: current_position + line_len, end: current_position + line_len + 1 }) {
                match tile {
                    "|" | "L" | "J" => {
                        new_steps.push(current_position + line_len);
                    },
                    _ => (),
                }
            }

            // E
            if let Some(tile) = input.get(std::ops::Range { start: current_position + 1, end: current_position + 2 }) {
                match tile {
                    "-" | "F" | "L" => {
                        new_steps.push(current_position + 1);
                    },
                    _ => (),
                }
            }

            // W
            if let Some(tile) = input.get(std::ops::Range { start: current_position - 1, end: current_position }) {
                match tile {
                    "-" | "7" | "J" => {
                        new_steps.push(current_position - 1);
                    },
                    _ => (),
                }
            }

            println!("{:?}", new_steps);
            if new_steps.len() == 0 {
                completed_path_lens.push(path.len());
                active_paths.retain(|x| *x != *path);

                if active_paths.len() == 0 {
                    break 'outer;
                }
            }
            else {
                active_paths.retain(|x| *x != *path);
                for step in new_steps {
                    let mut tmp = path.clone();
                    tmp.push(step);
                    active_paths.push(tmp);
                }
            }
        }
    }

    completed_path_lens.sort();
    return completed_path_lens.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn example_1() {
        let input: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

        let result: usize = run(input);

        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let input: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        let result: usize = run(input);

        assert_eq!(result, 8);
    }
}