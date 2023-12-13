use itertools::Itertools;
use nom::FindSubstring;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let response = input.lines().map(|line| {
        let numeric_chars: Vec<_> = line.chars().filter(|x| x.is_numeric()).collect();
        format!("{}{}",
                numeric_chars.first().unwrap(),
                numeric_chars.last().unwrap()
        )
            .parse::<u32>()
            .unwrap()
    }).sum();
    Some(response)
}

const INT_STRINGS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub fn part_two(input: &str) -> Option<u32> {
    let response = input.lines().map(|mut line| {
        let mut replaced = true;

        // Absolute overkill solution
        // At first, i thought nineight should be understood as 9ight
        // But actually, it is expected to be 98
        // Simple adjustment is to replace 9 -> 9e (keeping the last char to be reused later)
        while replaced {
            let mut int_substrings_index: Vec<_> = INT_STRINGS
                .iter()
                .enumerate()
                .filter_map(|(index, &int_string)|
                    line
                        .find_substring(int_string)
                        .map(|i| (i, index)))
                .sorted_by(|(a, _), (b, _)| Ord::cmp(b, a))
                .collect();

            if let Some((_, i)) = int_substrings_index.pop() {
                // The trick
                let last_char = INT_STRINGS[i].chars().last().unwrap();

                line = &*line.replacen(INT_STRINGS[i], &format!("{}{}", i + 1, last_char), 1).leak();
                replaced = true
            } else {
                replaced = false
            }
        }

        let numeric_chars: Vec<_> = line.chars().filter(|x| x.is_numeric()).collect();
        format!("{}{}",
                numeric_chars.first().unwrap(),
                numeric_chars.last().unwrap()
        )
            .parse::<u32>()
            .unwrap()
    }).sum();
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
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
