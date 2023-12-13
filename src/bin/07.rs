use std::cmp::Ordering;
use std::collections::HashMap;
use itertools::Itertools;
use lazy_static::lazy_static;

advent_of_code::solution!(7);

struct Row<'a> {
    cards: &'a str,
    bid: u16,
}


fn parse_input(input: &str) -> Vec<Row> {
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_whitespace().collect_tuple().unwrap();
            let bid = bid.parse().unwrap();

            Row {
                bid,
                cards,
            }
        })
        .collect()
}

/*
   Hand types:
   6: Five of a kind
   5: Four of a kind
   4: Full house
   4: Three of a kind
   2: Two pair
   1: One pair
   0: High card
 */

fn hand_type(hand: &str) -> u8 {
    let mut card_count: HashMap<char, u8> = HashMap::with_capacity(5);

    for c in hand.chars() {
        *card_count.entry(c).or_insert(0) += 1;
    }

    let mut cards_iter = card_count.values().sorted().rev();
    let first = cards_iter.next();
    let second = cards_iter.next();

    match (first, second) {
        (Some(5), _) => 6, // Five of a kind
        (Some(4), _) => 5, // Four of a kind
        (Some(3), Some(2)) => 4, // Full house
        (Some(3), _) => 3, // Three of a kind
        (Some(2), Some(2)) => 2, // Two pair
        (Some(2), _) => 1, // One pair
        (_, _) => 0 // High card
    }
}

lazy_static! {
    static ref CARD_VALUES: HashMap<char, u8> = HashMap::from([
        ('1', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);
}

fn compare_hands((hand, hand_type): (&str, &u8), (other_hand, other_hand_type): (&str, &u8)) -> Ordering {
    match hand_type.cmp(other_hand_type) {
        Ordering::Equal => {
            for i in 0usize..5usize {
                match CARD_VALUES[&hand.chars().nth(i).unwrap()].cmp(&CARD_VALUES[&other_hand.chars().nth(i).unwrap()]) {
                    Ordering::Equal => { continue; }
                    o => {
                        return o;
                    }
                }
            }
            Ordering::Equal
        }
        ord => ord,
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let hands = parse_input(input);
    Some(
        hands
            .iter()
            .map(|row| (row, hand_type(row.cards)))
            .sorted_by(|(row, hand_type), (other_row, other_hand_type)|
                compare_hands((row.cards, hand_type), (other_row.cards, other_hand_type))
            )
            .enumerate()
            .map(|(i, (row, _))| row.bid as u64 * (i + 1) as u64)
            .sum()
    )
}

fn hand_type_joker(hand: &str) -> u8 {
    let mut card_count: HashMap<char, u8> = HashMap::with_capacity(5);

    for c in hand.chars() {
        *card_count.entry(c).or_insert(0) += 1;
    }

    let n_joker = card_count.remove(&'J').unwrap_or(0);
    let mut cards_iter = card_count.values().sorted().rev();
    let first = cards_iter.next();
    let second = cards_iter.next();


    match (first, second, n_joker) {
        (Some(n), _, j) if n + j == 5 => 6, // Five of a kind
        (None, _, 5) => 6, // Five of a kind, joker only
        (Some(n), _, j) if n + j == 4 => 5, // Four of a kind
        (Some(3), Some(2), 0) => 4, // Full house
        (Some(2), Some(2), 1) => 4, // Full house (including 1 joker)
        (Some(n), _, j) if n + j == 3 => 3, // Three of a kind
        (Some(2), Some(2), 0) => 2, // Two pairs
        (Some(n), _, j) if n + j == 2 => 1, // One pair
        (_, _, _) => 0 // High card
    }
}

lazy_static! {
    static ref CARD_VALUES_JOKER: HashMap<char, u8> = HashMap::from([
        ('1', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 0),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);
}

fn compare_hands_joker((hand, hand_type): (&str, &u8), (other_hand, other_hand_type): (&str, &u8)) -> Ordering {
    match hand_type.cmp(other_hand_type) {
        Ordering::Equal => {
            for i in 0usize..5usize {
                match CARD_VALUES_JOKER[&hand.chars().nth(i).unwrap()]
                    .cmp(&CARD_VALUES_JOKER[&other_hand.chars().nth(i).unwrap()]) {
                    Ordering::Equal => { continue; }
                    o => {
                        return o;
                    }
                }
            }
            Ordering::Equal
        }
        ord => ord,
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let hands = parse_input(input);
    Some(
        hands
            .iter()
            .map(|row| (row, hand_type_joker(row.cards)))
            .sorted_by(|(row, hand_type), (other_row, other_hand_type)|
                compare_hands_joker((row.cards, hand_type), (other_row.cards, other_hand_type))
            )
            .enumerate()
            .map(|(i, (row, _))| row.bid as u64 * (i + 1) as u64)
            .sum()
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
