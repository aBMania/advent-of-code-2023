use tailcall::tailcall;
advent_of_code::solution!(9);

pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line|
            line
                .split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect()
        )
        .collect()
}

fn serie_gaps(serie: &[i32]) -> Vec<i32> {
    serie
        .windows(2)
        .map(|slice| match slice {
            &[a, b] => b - a,
            _ => unreachable!(), // Handle other slice patterns if needed
        })
        .collect()
}

fn find_next(serie: &Vec<i32>) -> i32 {

    #[tailcall]
    fn find_next_inner(serie: &Vec<i32>) -> i32 {
        let gaps = serie_gaps(serie);
        if gaps.iter().all(|&n| n == 0) {
            *serie.last().expect("No more element in serie")
        } else {
            serie.last().expect("No more element in serie") + find_next(&gaps)
        }
    }
    find_next_inner(serie)
}

fn find_previous(serie: &Vec<i32>) -> i32 {
    #[tailcall]
    fn find_previous_inner(serie: &Vec<i32>) -> i32 {
        let gaps = serie_gaps(serie);
        if gaps.iter().all(|&n| n == 0) {
            *serie.first().expect("No more element in serie")
        } else {
            serie.first().expect("No more element in serie") - find_previous(&gaps)
        }
    }
    find_previous_inner(serie)
}

pub fn part_one(input: &str) -> Option<i32> {
    let series = parse_input(input);

    Some(
        series
            .into_iter()
            .map(|serie| {
                find_next(&serie)
            })
            .sum()
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    let series = parse_input(input);

    Some(
        series
            .into_iter()
            .map(|serie| {
                find_previous(&serie)
            })
            .sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
