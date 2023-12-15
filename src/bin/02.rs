advent_of_code::solution!(2);

struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    draws: Vec<Draw>,
}

fn parse_draw(draw_str: &str) -> Draw {
    let mut draw = Draw {
        green: 0,
        blue: 0,
        red: 0,
    };
    for color_str in draw_str.split(',') {
        let mut parts = color_str.trim().split(' ');
        let n: u32 = parts.next().unwrap().parse().unwrap();
        let color: &str = parts.next().unwrap();

        match color {
            "red" => draw.red = n,
            "blue" => draw.blue = n,
            "green" => draw.green = n,
            _ => panic!(),
        }
    }

    draw
}

fn parse_draws(draws_str: &str) -> Vec<Draw> {
    draws_str.split(';').map(parse_draw).collect()
}

fn parse_game(game_str: &str) -> Game {
    let mut parts = game_str.splitn(2, ':');
    let id: u32 = parts
        .next()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let draws = parse_draws(parts.next().unwrap());

    Game { id, draws }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(parse_game)
            .filter(|game: &Game| {
                game.draws
                    .iter()
                    .all(|draw: &Draw| draw.blue <= 14 && draw.green <= 13 && draw.red <= 12)
            })
            .map(|game: Game| game.id)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(parse_game)
            .map(|game: Game| {
                let max_red = game
                    .draws
                    .iter()
                    .map(|draw: &Draw| draw.red)
                    .max()
                    .unwrap_or(0);
                let max_blue = game
                    .draws
                    .iter()
                    .map(|draw: &Draw| draw.blue)
                    .max()
                    .unwrap_or(0);
                let max_green = game
                    .draws
                    .iter()
                    .map(|draw: &Draw| draw.green)
                    .max()
                    .unwrap_or(0);

                max_blue * max_red * max_green
            })
            .sum(),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
