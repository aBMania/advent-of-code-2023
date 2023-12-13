use std::collections::HashMap;
advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let input: Vec<_> = input.lines()
        .map(|line| {
            let parts: Vec<_> = line.splitn(2, ": ").collect();
            let id_parts: Vec<_> = parts.first().unwrap().splitn(2, ' ').collect();
            let id: u32 = id_parts.get(1).unwrap().trim().parse().unwrap();

            let numbers_parts: Vec<_> = parts.get(1).unwrap().splitn(2, '|').collect();
            let winning_numbers: Vec<u32> = numbers_parts.first().unwrap().split(' ').collect::<Vec<_>>().iter()
                .filter_map(|&number| number.trim().parse().ok())
                .collect();
            let my_numbers: Vec<u32> = numbers_parts.get(1).unwrap().split(' ').collect::<Vec<_>>().iter()
                .filter_map(|&number| number.trim().parse().ok())
                .collect();
            (id, winning_numbers, my_numbers)
        })
        .collect();

    Some(
        input.iter()
            .map(|(_, winning_numbers, my_numbers)| {
                let n = winning_numbers.iter().filter(|n| my_numbers.contains(n)).count();
                match n {
                    0 => 0,
                    _ => 2u32.pow(n as u32 - 1)
                }
            }).sum()
    )
}


pub fn part_two(input: &str) -> Option<u32> {
    let input: Vec<_> = input.lines()
        .map(|line| {
            let parts: Vec<_> = line.splitn(2, ": ").collect();
            let id_parts: Vec<_> = parts.first().unwrap().splitn(2, ' ').collect();
            let id: u32 = id_parts.get(1).unwrap().trim().parse().unwrap();

            let numbers_parts: Vec<_> = parts.get(1).unwrap().splitn(2, '|').collect();
            let winning_numbers: Vec<u32> = numbers_parts.first().unwrap().split(' ').collect::<Vec<_>>().iter()
                .filter_map(|&number| number.trim().parse().ok())
                .collect();
            let my_numbers: Vec<u32> = numbers_parts.get(1).unwrap().split(' ').collect::<Vec<_>>().iter()
                .filter_map(|&number| number.trim().parse().ok())
                .collect();
            (id, winning_numbers, my_numbers)
        })
        .collect();

    let mut cards: HashMap<u32, u32> = HashMap::new();

    Some(
        input.iter()
            .map(|(id, winning_numbers, my_numbers)| {
                let n_cards = 1 + *cards.get(id).unwrap_or(&0);


                let n = winning_numbers.iter().filter(|n| my_numbers.contains(n)).count();
                for i in (*id + 1)..=(*id + n as u32) {
                    *cards.entry(i).or_insert(0) += n_cards;
                };

                n_cards
            }).sum()
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
