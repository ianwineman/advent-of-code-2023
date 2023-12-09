use std::collections::HashSet;
use std::ops::Range;

fn main() {
    let input: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    println!("Ans: {}", run(input))
}

fn range_parser(map: &str) -> (Range<i32>, i32) {
    let numbers: Vec<i32> = map
        .split(" ")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(
            |x| x.parse::<i32>().unwrap()
        )
        .collect();

    let destination_start = numbers[0];
    let source_start = numbers[1];
    let range_length = numbers[2];

    return (
        std::ops::Range {
            start: source_start, end: source_start + range_length
        },
        destination_start - source_start
    ) // (source_range, destination_range_offset)
}

fn run(input: &str) -> i32 {
    let almanac: Vec<&str> = input.lines().collect();

    let seeds: Vec<i32> = almanac[0].split(" ")
        .collect::<Vec<&str>>()
        .split_first()
        .unwrap()
        .1
        .into_iter()
        .map(
            |x| x.parse::<i32>().unwrap_or_default()
        )
        .collect();

    let mut mapped_seeds: Vec<i32> = Vec::from(seeds.clone());

    let mut ranges: HashSet<(Range<i32>, i32)> = HashSet::new();

    for line in almanac {
        if line.contains("map") {
            let mut new_mapped_seeds: Vec<i32> = Vec::new();

            for seed in mapped_seeds {
                for range in ranges.iter() {
                    let source_range = &range.0;
                    if source_range.contains(&seed) {
                        new_mapped_seeds.push(seed + range.1 + range.0.start);
                        break;
                    }
                }
                new_mapped_seeds.push(seed);
            }

            mapped_seeds = new_mapped_seeds;
            ranges = HashSet::new();
        }
        else if line == "\n" {
            continue;
        }
        else if line == "" {
            continue;
        }
        else if line.contains("seeds") {
            continue;
        }
        else {
            ranges.insert(range_parser(line));
        }
    }

    mapped_seeds.sort();
    mapped_seeds.reverse();
    return mapped_seeds.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn example_1() {
        let input: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let result: i32 = run(input);

        assert_eq!(result, 35);
    }
}