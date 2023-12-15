use itertools::Itertools;
advent_of_code::solution!(15);

fn hash(input: &str) -> u32 {
    input
        .chars()
        .fold(0, |acc, c| (17 * (acc + c as u32)) % 256)
}

pub fn part_one(input: &str) -> Option<u32> {
    // let input = input.to_string().leak();
    Some(input.trim().split(',').map(hash).sum())
}

#[derive(Debug, Clone)]
enum OperationChar {
    Dash,
    Equal(u8),
}

#[derive(Debug, Clone)]
struct Entry<'a> {
    label: &'a str,
    operation_char: OperationChar,
}

fn parse_entry(str: &str) -> Entry {
    if str.ends_with('-') {
        Entry {
            label: str.strip_suffix('-').unwrap(),
            operation_char: OperationChar::Dash,
        }
    } else {
        Entry {
            label: &str[..str.len() - 2],
            operation_char: OperationChar::Equal(str[str.len() - 1..].parse().unwrap()),
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    // let input = input.to_string().leak();

    let sum = input
        .trim()
        .split(',')
        .map(parse_entry)
        .fold(vec![vec![]; 256], |mut boxes: Vec<Vec<Entry>>, entry| {
            let hash = hash(entry.label) as usize;
            let current_box = boxes.get_mut(hash).unwrap();

            let entry_with_same_label =
                current_box.iter().find_position(|e| e.label == entry.label);

            match &entry.operation_char {
                OperationChar::Dash => {
                    if let Some((same_label_entry_position, _)) = entry_with_same_label {
                        current_box.remove(same_label_entry_position);
                    }
                }
                OperationChar::Equal(_) => {
                    if let Some((same_label_entry_position, _)) = entry_with_same_label {
                        current_box.push(entry);
                        current_box.swap_remove(same_label_entry_position);
                    } else {
                        current_box.push(entry);
                    }
                }
            }
            boxes
        })
        .iter()
        .enumerate()
        .map(|(i, b)| {
            (i + 1) as u32
                * b.iter()
                    .enumerate()
                    .map(|(ii, entry)| {
                        (ii + 1) as u32
                            * match entry.operation_char {
                                OperationChar::Dash => 0,
                                OperationChar::Equal(fl) => fl as u32,
                            }
                    })
                    .sum::<u32>()
        })
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
