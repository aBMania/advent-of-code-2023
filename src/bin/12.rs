use memoize::memoize;
use rayon::prelude::*;

advent_of_code::solution!(12);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Row<'a> {
    pattern: &'a str,
    consecutives: Vec<u8>,
}

fn parse_input(input: &str) -> Vec<Row> {
    input
        .lines()
        .map(|line| {
            let [pattern, consecutives] = line.split_whitespace().collect::<Vec<_>>()[0..2] else { panic!() };
            let consecutives: Vec<_> = consecutives.split(',').map(|i| i.parse().unwrap()).collect();
            Row {
                pattern,
                consecutives,
            }
        })
        .collect()
}

#[memoize]
fn solve_row(row: Row<'static>) -> u64 {
    // let row_clone = row.clone();
    // println!("solving {:?}", &row);
    if row.consecutives.is_empty() {
        return match row.pattern.contains('#') {
            true => {
                // println!("{row_clone:?}");
                // println!("return 0 (consecutive empty, # remaining)");
                0
            }
            false => {
                // println!("{row_clone:?}");
                // println!("return 1 (consecutive empty, no #)");
                1
            }
        };
    }

    if row.pattern.is_empty() {
        // println!("{row_clone:?}");
        // println!("return 0 (pattern empty)");
        return 0;
    }

    let mut row = row.clone();
    row.pattern = row.pattern.trim_end_matches('.'); // Remove trailing dots
    let consecutive = row.consecutives.pop().unwrap() as usize;

    let last_dot_pos = row.pattern.rfind('.');

    let left_space = match last_dot_pos {
        None => row.pattern.len(),
        Some(last_dot_pos) => row.pattern.len() - last_dot_pos - 1
    };

    if left_space < consecutive {
        return match last_dot_pos {
            None => {
                // println!("{row_clone:?}");
                // println!("return 0 (no space left for a {consecutive} consecutive)");
                0
            }
            Some(last_dot_pos) => {
                if row.pattern[last_dot_pos..].contains('#') {
                    // println!("{row_clone:?}");
                    // println!("return 0 (cannot include # in a consecutive)");
                    0
                } else {
                    let mut consecutive_with_popped = row.consecutives.clone();
                    consecutive_with_popped.push(consecutive as u8);
                    

                    // println!("{row_clone:?}");
                    // println!("return {sub_solve} (no solution, went to subsolve for {:?})", &row.pattern[..last_dot_pos]);

                    solve_row(Row {
                        consecutives: consecutive_with_popped,
                        pattern: &row.pattern[..last_dot_pos],
                    })
                }
            }
        };
    }

    let current = &row.pattern[row.pattern.len() - left_space..];
    let rest = &row.pattern[..row.pattern.len() - left_space];

    if left_space == consecutive {
        let sub_case_replace_by_hashtag = Row {
            consecutives: row.consecutives.clone(),
            pattern: rest,
        };

        return if current.chars().all(|c| c.eq(&'?')) {
            let mut consecutive_with_popped = row.consecutives.clone();
            consecutive_with_popped.push(consecutive as u8);
            let sub_case_replace_by_dot = Row {
                consecutives: consecutive_with_popped,
                pattern: rest,
            };

            let sub_case_replace_by_dot_solve = solve_row(sub_case_replace_by_dot);
            let sub_case_replace_by_hashtag_solve = solve_row(sub_case_replace_by_hashtag);

            // println!("{row_clone:?}");
            // println!("return {sub_case_replace_by_dot_solve} + {sub_case_replace_by_hashtag_solve} (dot + hashtag)");
            sub_case_replace_by_dot_solve + sub_case_replace_by_hashtag_solve
        } else {
            

            // println!("{row_clone:?}");
            // println!("return sub_case_replace_by_hashtag_solve (hashtag)");

            solve_row(sub_case_replace_by_hashtag)
        };
    }

    let possibilities = left_space - consecutive + 1;
    let start_looking = row.pattern.len() - left_space;
    let end_looking = start_looking + possibilities;

    let mut total = 0;

    // Consider replacing everything by dots
    if let Some(last_dot_pos) = last_dot_pos {
        if !row.pattern[last_dot_pos..].contains('#') {
            let mut consecutive_with_popped = row.consecutives.clone();
            consecutive_with_popped.push(consecutive as u8);
            let sub_solve = solve_row(Row {
                consecutives: consecutive_with_popped,
                pattern: &row.pattern[..last_dot_pos],
            });

            total += sub_solve;
        }
    }


    for i in start_looking..end_looking {
        let before = match i {
            0 => '.',
            i => row.pattern.chars().nth(i - 1).unwrap()
        };
        let after = &row.pattern[i + consecutive..];
        // println!("{i} before: {before} after: {after}");
        if (before == '?' || before == '.') &&
            after.chars().all(|c| c.eq(&'?')) {
            let sub_row = match i {
                0 => Row {
                    consecutives: row.consecutives.clone(),
                    pattern: "",
                },
                i => Row {
                    consecutives: row.consecutives.clone(),
                    pattern: &row.pattern[..i - 1],
                }
            };

            total += solve_row(sub_row)
        }
    }

    // println!("{row_clone:?}");
    // println!("return sum of {:?}: {}", &total, total.iter().sum::<u64>());
    total
}

fn solve(rows: Vec<Row<'static>>) -> u64 {
    rows
        .par_iter()
        .map(|row| {
            
            // println!("{:?} {:?} {}", row.pattern, row.consecutives, solve);
            solve_row(row.clone())
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = input.to_string().leak();
    let input = parse_input(input);

    Some(solve(input))
}

fn complicate_things(input: &mut [Row]) {
    for row in input.iter_mut() {
        let mut duplicated = format!("{}?", row.pattern).repeat(5);
        duplicated.pop(); // remove trailing ?
        row.pattern = duplicated.leak();
        row.consecutives = row.consecutives.repeat(5);

        // println!("{:?}", row);
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input.to_string().leak();
    let mut input = parse_input(input);

    complicate_things(&mut input);

    Some(solve(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve_row_trivial() {
        assert_eq!(solve_row(Row {
            pattern: "#",
            consecutives: vec![1],
        }), 1);
        assert_eq!(solve_row(Row {
            pattern: "?",
            consecutives: vec![1],
        }), 1);
        assert_eq!(solve_row(Row {
            pattern: ".",
            consecutives: vec![1],
        }), 0);
        assert_eq!(solve_row(Row {
            pattern: ".#",
            consecutives: vec![1],
        }), 1);
        assert_eq!(solve_row(Row {
            pattern: "..##",
            consecutives: vec![2],
        }), 1);
        assert_eq!(solve_row(Row {
            pattern: "###.",
            consecutives: vec![3],
        }), 1);
        assert_eq!(solve_row(Row {
            pattern: "?.?.?",
            consecutives: vec![1],
        }), 3);
    }

    #[test]
    fn test_solve_example_individual() {
        assert_eq!(solve_row(Row {
            pattern: "..???",
            consecutives: vec![1, 1],
        }), 1);

        assert_eq!(solve_row(Row {
            pattern: ".??..??...?##.",
            consecutives: vec![1, 1, 3],
        }), 4);

        assert_eq!(solve_row(Row {
            pattern: "?#?#?#?#?#?#?#?",
            consecutives: vec![1, 3, 1, 6],
        }), 1);

        assert_eq!(solve_row(Row {
            pattern: "????.#...#...",
            consecutives: vec![4, 1, 1],
        }), 1);

        assert_eq!(solve_row(Row {
            pattern: "????.######..#####.",
            consecutives: vec![1, 6, 5],
        }), 4);

        assert_eq!(solve_row(Row {
            pattern: "?###????????",
            consecutives: vec![3, 2, 1],
        }), 10);
    }

    #[test]
    fn test_solve_part_1_individual() {
        assert_eq!(solve_row(Row { pattern: "##????????#?#??????", consecutives: vec![4, 1, 8, 2] }), 4);
        assert_eq!(solve_row(Row { pattern: "?.#??????.#????#??", consecutives: vec![1, 1, 1, 1, 1, 7] }), 1);
        assert_eq!(solve_row(Row { pattern: ".#??.??.????###?????", consecutives: vec![1, 1, 2, 8, 3] }), 1);
        assert_eq!(solve_row(Row { pattern: "??.???#????", consecutives: vec![1, 4, 1] }), 13);
        assert_eq!(solve_row(Row { pattern: "?????.??????##.", consecutives: vec![2, 3, 3] }), 8);
        assert_eq!(solve_row(Row { pattern: ".??#??.??#", consecutives: vec![3, 2] }), 3);
        assert_eq!(solve_row(Row { pattern: "?.#?##??#.?#?????", consecutives: vec![1, 5, 1, 2, 3] }), 3);
        assert_eq!(solve_row(Row { pattern: "?.###??.??#??????", consecutives: vec![4, 8] }), 2);
        assert_eq!(solve_row(Row { pattern: "?#?????#???", consecutives: vec![2, 1, 1] }), 9);
        assert_eq!(solve_row(Row { pattern: "???????..??#?.", consecutives: vec![3, 1] }), 5);
        assert_eq!(solve_row(Row { pattern: "??#??###.????#??.???", consecutives: vec![1, 6, 2, 3, 3] }), 3);
        assert_eq!(solve_row(Row { pattern: "???????#?????#..??", consecutives: vec![5, 2] }), 4);
        assert_eq!(solve_row(Row { pattern: "....#?##????.??#??", consecutives: vec![4, 1] }), 1);
        assert_eq!(solve_row(Row { pattern: "?#??.?.?#?????", consecutives: vec![2, 1, 4] }), 8);
        assert_eq!(solve_row(Row { pattern: "?#?##????#??.#?#", consecutives: vec![5, 4, 1, 1] }), 5);
        assert_eq!(solve_row(Row { pattern: "..?.????#??????????", consecutives: vec![1, 1, 1, 1, 1, 4] }), 26);
        assert_eq!(solve_row(Row { pattern: "?.????#????", consecutives: vec![4, 2] }), 3);
        assert_eq!(solve_row(Row { pattern: "??.#???.?????", consecutives: vec![1, 3, 1, 1] }), 12);
        assert_eq!(solve_row(Row { pattern: ".?###???????.?##", consecutives: vec![4, 2, 3] }), 9);
        assert_eq!(solve_row(Row { pattern: "#??.?##????#?????", consecutives: vec![3, 8, 2] }), 5);
        assert_eq!(solve_row(Row { pattern: "?#.???.?#?", consecutives: vec![2, 1, 2] }), 6);
        assert_eq!(solve_row(Row { pattern: "?#???.#????.??", consecutives: vec![4, 2, 1, 2] }), 4);
        assert_eq!(solve_row(Row { pattern: ".???.??.#?????#?#", consecutives: vec![1, 1, 4, 1, 1] }), 7);
    }

    #[test]
    fn test_solve_part_1_cases() {
        assert_eq!(solve_row(Row {
            pattern: "???.",
            consecutives: vec![1, 1],
        }), 1);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
