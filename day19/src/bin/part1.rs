use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input: &str = "";

    println!("Ans: {}", run(input))
}

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(Debug, Copy, Clone)]
enum Condition {
    Greater,
    Less,
    None,
}

#[derive(Debug, Copy, Clone)]
enum Category {
    X,
    M,
    A,
    S,
    None,
}

#[derive(Debug)]
struct Workflow<'a> {
    id: &'a str,
    rules: Vec<Rule<'a>>,
}

#[derive(Debug, Copy, Clone)]
struct Rule<'a> {
    category: Category,
    condition: Condition,
    value: u32,
    destination: &'a str
}

impl Part {
    pub fn from(input: &str) -> Part {
        let re = Regex::new(r"\{x=(?<x>\d+),m=(?<m>\d+),a=(?<a>\d+),s=(?<s>\d+)\}").unwrap();
        let caps = re.captures(input).unwrap();

        return Part {
            x: caps.name("x").unwrap().as_str().parse::<u32>().unwrap(),
            m: caps.name("m").unwrap().as_str().parse::<u32>().unwrap(),
            a: caps.name("a").unwrap().as_str().parse::<u32>().unwrap(),
            s: caps.name("s").unwrap().as_str().parse::<u32>().unwrap(),
        }
    }

    pub fn apply_rule<'b>(&'b self, rule: Rule<'b>) -> Option<&str> {
        match rule.condition {
            Condition::Less => {
                match rule.category {
                    Category::X => {
                        if self.x < rule.value {
                            return Some(rule.destination)
                        }
                        else {
                            return None
                        }
                    },
                    Category::M => {
                        if self.m < rule.value {
                            return Some(rule.destination)
                        }
                        else {
                            return None
                        }
                    },
                    Category::A => {
                        if self.a < rule.value {
                            return Some(rule.destination)
                        }
                        else {
                            return None
                        }
                    },
                    Category::S => {
                        if self.s < rule.value {
                            return Some(rule.destination)
                        }
                        else {
                            return None
                        }
                    },
                    Category::None => panic!(),
                }
            },
            Condition::Greater => {
                match rule.category {
                    Category::X => {
                        if self.x > rule.value {
                            return Some(rule.destination)
                        }
                        else {
                            return None
                        }
                    },
                    Category::M => {
                        if self.m > rule.value {
                            return Some(rule.destination)
                        }
                        else {
                            return None
                        }
                    },
                    Category::A => {
                        if self.a > rule.value {
                            return Some(rule.destination)
                        }
                        else {
                            return None
                        }
                    },
                    Category::S => {
                        if self.s > rule.value {
                            return Some(rule.destination)
                        }
                        else {
                            return None
                        }
                    },
                    Category::None => panic!(),
                }
            },
            Condition::None => {
                return Some(rule.destination)
            }
        }
    }
}

impl Rule<'_> {
    pub fn from(input: &str) -> Rule {
        if input.contains(":") {
            let re = Regex::new(r"(?<category>[xmas])(?<condition>[><])(?<value>\d+):(?<destination>\w+)").unwrap();
            let caps = re.captures(input).unwrap();

            return Rule {
                category: {
                    let cat = caps.name("category").unwrap().as_str();
                    match cat {
                        "x" => Category::X,
                        "m" => Category::M,
                        "a" => Category::A,
                        "s" => Category::S,
                        _ => panic!(),
                    }
                },
                condition: {
                    if caps.name("condition").unwrap().as_str() == ">" {
                        Condition::Greater
                    }
                    else {
                        Condition::Less
                    }
                },
                value: caps.name("value").unwrap().as_str().parse::<u32>().unwrap(),
                destination: caps.name("destination").unwrap().as_str(),
            }
        }
        else {
            return Rule {
                category: Category::None,
                condition: Condition::None,
                value: 0,
                destination: input,
            }
        }
    }
}

impl Workflow<'_> {
    pub fn from(input: &str) -> Workflow {
        let re = Regex::new(r"^(?<id>\w+)\{(?<rules>.*)\}").unwrap();
        let caps = re.captures(input).unwrap();

        return Workflow {
            id: caps.name("id").unwrap().as_str(),
            rules: caps.name("rules").unwrap().as_str().split(",").into_iter().map(
                |x| Rule::from(x)
            ).collect::<Vec<Rule>>(),
        }
    }
}

fn run(input: &str) -> u32 {
    let mut rating_numbers_sum: u32 = 0;

    let mut workflows: HashMap<&str, Vec<Rule>> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();

    for line in input.lines() {
        if line.starts_with("{") {
            parts.push(Part::from(line));
        }
        else if line.is_empty() {
            continue;
        }
        else {
            let w: Workflow = Workflow::from(line);
            workflows.insert(w.id, w.rules);
        }
    }

    for part in parts {
        let mut current_workflow = workflows.get("in").unwrap();

        'outer: loop {
            'inner: for rule in current_workflow {
                if let Some(des) = part.apply_rule(*rule) {
                    match des {
                        "R" => {
                            break 'outer;
                        },
                        "A" => {
                            rating_numbers_sum += part.x + part.m + part.a + part.s;
                            break 'outer;
                        },
                        _ => {
                            current_workflow = workflows.get(des).unwrap();
                            break 'inner;
                        },
                    }
                }
                else {
                    continue;
                }
            }
        }
    }

    return rating_numbers_sum
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn example_1() {
        let input: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

        let result: u32 = run(input);

        assert_eq!(result, 19114);
    }
}