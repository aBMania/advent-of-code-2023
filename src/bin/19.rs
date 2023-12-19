use std::collections::HashMap;

advent_of_code::solution!(19);

#[derive(Copy, Clone, Debug)]
enum Comp {
    GreaterThan,
    LowerThan,
}

#[derive(Copy, Clone, Debug)]
enum Next<'a> {
    Accepted,
    Rejected,
    Rule(&'a str),
}

#[derive(Copy, Clone, Debug)]
struct Rule<'a> {
    input_value: &'a str,
    value: u32,
    comp: Comp,
    next_ok: Next<'a>,
}

#[derive(Copy, Clone, Debug)]
struct InputValue {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}


#[derive(Copy, Clone, Debug, PartialEq)]
struct Range {
    from: u32,
    // included
    to: u32, // excluded
}

impl Range {
    fn new(from: u32, to: u32) -> Self {
        assert!(from < to);
        Self { from, to }
    }

    fn split(self, at: u32) -> Vec<Range> {
        if at < self.from || at > self.to {
            return vec![self];
        }
        vec![
            Range::new(self.from, at + 1),
            Range::new(at + 1, self.to),
        ]
    }
}

fn parse_next(input: &str) -> Next {
    match input {
        "A" => Next::Accepted,
        "R" => Next::Rejected,
        next => Next::Rule(next),
    }
}

fn parse_rule(input: &str) -> (&str, Vec<Rule>, Next) {
    let (name, rest) = input.split_once('{').unwrap();
    let rest = rest.strip_suffix('}').unwrap();
    let (rules, fallback) = rest.rsplit_once(',').unwrap();

    let rules = rules.split(',').map(|rule| {
        let (comp, next) = rule.split_once(':').unwrap();

        Rule {
            input_value: &comp[..1],
            value: comp[2..].parse().unwrap(),
            comp: match &comp[1..2] {
                ">" => Comp::GreaterThan,
                "<" => Comp::LowerThan,
                _ => unimplemented!()
            },
            next_ok: parse_next(next),
        }
    }).collect();
    (name, rules, parse_next(fallback))
}

fn parse_rules(input: &str) -> HashMap<&str, (Vec<Rule>, Next)> {
    input.lines().fold(HashMap::new(), |mut acc, line| {
        let (name, rules, fallback) = parse_rule(line);
        acc.insert(name, (rules, fallback));
        acc
    })
}

fn parse_input_values(input: &str) -> Vec<InputValue> {
    input.lines().map(|line| {
        let line = line.strip_prefix('{').unwrap();
        let line = line.strip_suffix('}').unwrap();
        let mut parts = line.split(',');

        let x = parts.next().unwrap()[2..].parse().unwrap();
        let m = parts.next().unwrap()[2..].parse().unwrap();
        let a = parts.next().unwrap()[2..].parse().unwrap();
        let s = parts.next().unwrap()[2..].parse().unwrap();
        InputValue {
            x,
            m,
            a,
            s,
        }
    }).collect()
}

fn parse_input(input: &str) -> (HashMap<&str, (Vec<Rule>, Next)>, Vec<InputValue>) {
    let mut parts = input.split("\n\n");

    let rules = parse_rules(parts.next().unwrap().trim());
    let input_values = parse_input_values(parts.next().unwrap().trim());

    (rules, input_values)
}

fn solve(rules: HashMap<&str, (Vec<Rule>, Next)>, input_value: Vec<InputValue>) -> u32 {
    input_value
        .iter()
        .fold(0, |acc, input| {
            let mut next = &Next::Rule("in");

            'outer: while let Next::Rule(rule) = next {
                let (rules, fallback) = rules.get(rule).unwrap();

                for rule in rules.iter() {
                    let compared_value = match rule.input_value {
                        "a" => input.a,
                        "m" => input.m,
                        "s" => input.s,
                        "x" => input.x,
                        _ => unreachable!()
                    };

                    let rule_result = match rule.comp {
                        Comp::GreaterThan => compared_value > rule.value,
                        Comp::LowerThan => compared_value < rule.value
                    };

                    if rule_result {
                        next = &rule.next_ok;
                        continue 'outer;
                    }
                }

                next = fallback;
            }

            match next {
                Next::Accepted => acc + input.a + input.m + input.s + input.x,
                Next::Rejected => acc,
                _ => unreachable!()
            }
        })
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, input_values) = parse_input(input);

    Some(solve(rules, input_values))
}

fn solve_ranges(rules: HashMap<&str, (Vec<Rule>, Next)>) -> u64 {
    let mut remaining_ranges = vec![
        (
            "in",
            HashMap::from([
                ("x", vec![Range::new(1, 4001)]),
                ("m", vec![Range::new(1, 4001)]),
                ("a", vec![Range::new(1, 4001)]),
                ("s", vec![Range::new(1, 4001)])
            ])
        )
    ];

    let mut total = 0;

    while let Some((rule, mut ranges)) = remaining_ranges.pop() {
        let (rules, fallback) = rules.get(rule).unwrap();

        for rule in rules.iter() {
            let current_range = ranges.get_mut(rule.input_value).unwrap();

            let split_value = match rule.comp {
                Comp::GreaterThan => rule.value,
                Comp::LowerThan => rule.value - 1
            };

            let (included_ranges, excluded_ranges): (Vec<_>, Vec<_>) = current_range
                .iter()
                .flat_map(|range| range.split(split_value))
                .partition(|range| match rule.comp {
                    Comp::GreaterThan => range.from > rule.value,
                    Comp::LowerThan => range.from < rule.value
                });

            *current_range = excluded_ranges;

            match rule.next_ok {
                Next::Accepted => {
                    total += ranges
                        .iter()
                        .filter(|(&i, _)| i != rule.input_value)
                        .flat_map(|(_, range)| range)
                        .chain(included_ranges.iter())
                        .fold(1u64,|acc, range| acc * (range.to - range.from) as u64)
                }
                Next::Rule(r) => {
                    let mut future_ranges = ranges.clone();
                    future_ranges.insert(rule.input_value, included_ranges);
                    remaining_ranges.push((r, future_ranges));
                }
                _ => {}
            }
        }

        match fallback {
            Next::Accepted => {
                total += ranges
                    .iter()
                    .fold(1, |acc, (_, range)| acc * range.iter().map(|r| (r.to - r.from) as u64).sum::<u64>());
            }
            Next::Rejected => {}
            Next::Rule(r) => remaining_ranges.push((r, ranges))
        }
    }

    total
}

pub fn part_two(input: &str) -> Option<u64> {
    let (rules, _) = parse_input(input);


    Some(solve_ranges(rules))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
