advent_of_code::solution!(6);

struct Race {
    time: u64,
    distance: u64,
}

fn parse_input_1(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times: Vec<_> = lines.next().expect("times").split_ascii_whitespace().filter_map(|s| s.parse::<u64>().ok()).collect();
    let distances: Vec<_> = lines.next().expect("distances").split_ascii_whitespace().filter_map(|s| s.parse::<u64>().ok()).collect();

    times.into_iter().zip(distances)
        .map(|(time, distance)| Race {
            time,
            distance,
        })
        .collect()
}

fn solve_race(race: &Race) -> u64 {
    let t = race.time as f64;
    let d = race.distance as f64;
    let x_min = (0.5f64 * (t - (t * t - 4f64 * d).sqrt())).floor() as u64 + 1;
    let x_max = (0.5f64 * (t + (t * t - 4f64 * d).sqrt())).ceil() as u64 - 1;

    x_max - x_min + 1
}

pub fn part_one(input: &str) -> Option<u64> {
    let races = parse_input_1(input);

    let product = races.iter().map(solve_race).product();

    Some(product)
}

fn parse_input_2(input: &str) -> Race {
    let mut lines = input.lines();
    let time = lines.next().expect("time").chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse::<u64>().expect("time parsing");
    let distance = lines.next().expect("distance").chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse::<u64>().expect("distance parsing");

    Race {
        time,
        distance,
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let race = parse_input_2(input);

    Some(
        solve_race(&race)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
