use std::collections::HashMap;
use lazy_static::lazy_static;
use rayon::prelude::*;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let response = input
        .lines()
        .map(|line| {
            let numeric_chars: Vec<_> = line.chars().filter(|x| x.is_numeric()).collect();
            format!(
                "{}{}",
                numeric_chars.first().unwrap(),
                numeric_chars.last().unwrap()
            )
            .parse::<u32>()
            .unwrap()
        })
        .sum();
    Some(response)
}

lazy_static! {
    static ref CARD_VALUES: HashMap<&'static str, u8> = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
}


pub fn part_two(input: &str) -> Option<u32> {
    let response = input
        .par_lines()
        .map(|line| {
            let findings: Vec<_> = CARD_VALUES.iter().map(|(s, val)| (*val, line.find(s), line.rfind(s))).filter(|(_, min, _)| min.is_some()).collect();
            let min = findings.iter().min_by(|(_, min_index, _), (_, other_min_index, _)| min_index.cmp(other_min_index));
            let max = findings.iter().max_by(|(_, _, max_index), (_, _, other_max_index)| max_index.cmp(other_max_index));


            (min.unwrap().0 * 10 + max.unwrap().0) as u32
        })
        .sum();
    Some(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
