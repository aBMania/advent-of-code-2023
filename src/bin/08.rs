use num::integer::lcm;
use std::collections::HashMap;
advent_of_code::solution!(8);

#[derive(Eq, PartialOrd, PartialEq, Hash, Copy, Clone, Debug)]
enum Direction {
    Right,
    Left,
}

fn parse_input(input: &str) -> (Vec<Direction>, HashMap<(&str, Direction), &str>) {
    let (directions, nodes) = input
        .split_once("\n\n")
        .expect("split directions and nodes");
    let directions: Vec<_> = directions
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unhandled direction char"),
        })
        .collect();

    let nodes = nodes
        .lines()
        .flat_map(|line| {
            let node = &line[..3];
            let left_node = &line[7..10];
            let right_node = &line[12..15];

            [
                ((node, Direction::Left), left_node),
                ((node, Direction::Right), right_node),
            ]
        })
        .collect();

    (directions, nodes)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (directions, nodes) = parse_input(input);

    let mut n = 0;
    let mut current_node = "AAA";

    for direction in directions.iter().cycle() {
        if current_node == "ZZZ" {
            break;
        }
        current_node = nodes.get(&(current_node, *direction)).unwrap();
        n += 1;
    }

    Some(n)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (directions, nodes) = parse_input(input);

    let mut current_nodes: Vec<_> = nodes
        .keys()
        .filter(|(node, direction)| node.ends_with('A') && direction.eq(&Direction::Left))
        .map(|(node, _)| *node)
        .collect();

    let mut node_cycle: Vec<Option<u64>> = (0..current_nodes.len()).map(|_| None).collect();

    for (n, direction) in directions.iter().cycle().enumerate() {
        for (i, node) in current_nodes.iter().enumerate() {
            if node.ends_with('Z') {
                node_cycle[i] = Some(n as u64);
            }
        }

        if node_cycle.iter().all(|cycle| cycle.is_some()) {
            break;
        }

        for current_node in current_nodes.iter_mut() {
            *current_node = nodes.get(&(current_node, *direction)).unwrap();
        }
    }

    Some(
        node_cycle
            .into_iter()
            .fold(1u64, |acc, cycle| lcm(acc, cycle.unwrap())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
