use std::fs::File;
use std::io::{ Lines, BufReader };

struct Card {
    winning_numbers: Vec<u32>,
    having_numbers: Vec<u32>,
}

impl Card {
    fn from_string_input(s: &str) -> Self {
        let colon_index = s.find(":").expect("Bad input. No Card ID");
        let rest = &s[colon_index+1..];

        let pipe_index = rest.find("|").expect("Bad input. No pipe delimiter.");
        let (winning, having) = rest.split_at(pipe_index);

        let winning_numbers: Vec<u32> = winning.split(" ")
            .map(|x| x.parse::<u32>())
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect();

        let having_numbers: Vec<u32> = having.split(" ")
            .map(|x| x.parse::<u32>())
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect();
        
        Card {winning_numbers, having_numbers}
    }

    fn matching_numbers(&self) -> u32 {
        // this procedure assumes there are no duplicate numbers in
        // either the winning numbers or having numbers

        let mut all_nums = self.winning_numbers.clone();
        all_nums.append(&mut self.having_numbers.clone());

        let total_nums = all_nums.len();
        
        all_nums.sort_unstable();
        all_nums.dedup();
        let deduped_nums = all_nums.len();

        u32::try_from(total_nums - deduped_nums).unwrap()
    }

    fn points(&self) -> u32 {
        let base: u32 = 2;
        let diff = self.matching_numbers();
        match diff {
            0 => 0,
            1.. => base.pow(diff-1),
        }
    }
}

pub(crate) fn solve_part1(input: Lines<BufReader<File>>)  -> String {
    let mut ttl = 0;
    for line in input {
        let card = Card::from_string_input(&line.unwrap());
        ttl += card.points();
    }

    ttl.to_string()
}

pub(crate) fn solve_part2(input: Lines<BufReader<File>>)  -> String {
    let cards: Vec<Card> = input.map(|x| Card::from_string_input(&x.unwrap())).collect();

    // to start, we have 1 of each card
    let mut card_counts: Vec<u32> = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let matching_numbers = card.matching_numbers();
        let matching_numbers = usize::try_from(matching_numbers).expect("Number of cards exceeds usize");

        let copies_of_current_card = card_counts[i];

        let lbound = i+1;
        let ubound = i + matching_numbers;
        for i in lbound..=ubound {
            card_counts[i] += copies_of_current_card;
        }
    }

    let ttl_number_of_cards: u32 = card_counts.iter().sum();
    ttl_number_of_cards.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_constructor() {
        let card = Card::from_string_input("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        let expected_winning = vec!(41, 48, 83, 86, 17);
        let expected_having = vec!(83, 86, 6, 31, 17, 9, 48, 53);

        assert_eq!(card.winning_numbers, expected_winning);
        assert_eq!(card.having_numbers, expected_having);
    }

    #[test]
    fn test_card_points() {
        let card = Card::from_string_input("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(8, card.points());

        let card = Card::from_string_input("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19");
        assert_eq!(2, card.points());

        let card = Card::from_string_input("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36");
        assert_eq!(0, card.points());
    }
}
