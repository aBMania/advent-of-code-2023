use std::cmp::Ordering;
use std::collections::HashMap;
use itertools::Itertools;
advent_of_code::solution!(7);


fn parse_input(input: &str) -> Vec<(&str, u32)> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let hand = iter.next().unwrap();
            let bid = iter.next().unwrap();

            (hand, bid.parse().unwrap())
        })
        .collect()
}

fn hand_type(hand: &str) -> u32 {
    /*
       6: Five of a kind
       5: Four of a kind
       4: Full house
       4: Three of a kind
       2: Two pair
       1: One pair
       0: High card
     */
    let mut char_hm: HashMap<char, u32> = HashMap::new();
    for c in hand.chars() {
        *char_hm.entry(c).or_insert(0) += 1
    }

    match char_hm.values().max().unwrap() {
        5 => 6, // Five of a kind
        4 => 5, // Four of a kind
        3 => {
            if char_hm.values().contains(&2) {
                4 // Full house
            } else {
                3 // Three of a kind
            }
        }
        2 => {
            if char_hm.values().filter(|&&n| n == 2).count() == 2 {
                2
            } else {
                1
            }
        }
        _ => 0
    }
}

fn get_card_value(card: char) -> u32 {
    let mapping = HashMap::from([
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

    *mapping.get(&card).unwrap()
}

fn get_card_value_joker(card: char) -> u32 {
    let mapping = HashMap::from([
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

    *mapping.get(&card).unwrap()
}

fn compare_card(card: char, other: char) -> Ordering {
    get_card_value(card).cmp(&get_card_value(other))
}

fn compare_card_jocker(card: char, other: char) -> Ordering {
    get_card_value_joker(card).cmp(&get_card_value_joker(other))
}

fn compare_hand(hand: &str, other: &str) -> Ordering {
    match hand_type(hand).cmp(&hand_type(other)) {
        Ordering::Equal => {
            for i in 0usize..5usize {
                match compare_card(hand.chars().nth(i).unwrap(), other.chars().nth(i).unwrap()) {
                    Ordering::Equal => { continue; }
                    o => {
                        return o;
                    }
                }
            }
            Ordering::Equal
        }
        a => {
            a
        }
    }
}

fn compare_hand_joker(hand: &str, other: &str) -> Ordering {
    match hand_type_joker(hand).cmp(&hand_type_joker(other)) {
        Ordering::Equal => {
            for i in 0..hand.len() {
                match compare_card_jocker(hand.chars().nth(i).unwrap(), other.chars().nth(i).unwrap()) {
                    Ordering::Equal => { continue; }
                    o => {
                        return o;
                    }
                }
            }
            Ordering::Equal
        }
        a => {
            a
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = parse_input(input);

    hands.sort_by(|(hand1, _), (hand2, _)| compare_hand(hand1, hand2));

    Some(
        hands.into_iter().enumerate().map(|(i, (_, bid))| (i as u32 + 1) * bid).sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands = parse_input(input);

    hands.sort_by(|(hand1, _), (hand2, _)| compare_hand_joker(hand1, hand2));

    Some(
        hands.into_iter().enumerate().map(|(i, (_, bid))| (i as u32 + 1) * bid).sum()
    )
}

// return hand type
fn hand_type_joker(hand: &str) -> u32 {
    let mut char_hm: HashMap<char, u32> = HashMap::new();
    for c in hand.chars() {
        *char_hm.entry(c).or_insert(0) += 1
    }

    if !char_hm.contains_key(&'J') {
        return hand_type(hand);
    }

    let (candidate, n) = char_hm
        .iter()
        .filter(|(&c, _)| c != 'J')
        .max_by(|(&card, &value), (&other_card, &other_value)| {
            match value.cmp(&other_value) {
                Ordering::Equal => get_card_value(card).cmp(&get_card_value(other_card)),
                o => o
            }
        })
        .unwrap_or((&'A', &2));

    if n > &1 {
        hand_type(&hand.replace('J', &candidate.to_string()))
    } else {
        let (best_card, _) = char_hm.iter().filter(|(&c, _)| c != 'J').max_by(|(&card, _), (&other, _)| get_card_value(card).cmp(&get_card_value(other))).unwrap();
        hand_type(&hand.replace('J', &best_card.to_string()))
    }

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
