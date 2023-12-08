use std::cmp::Ordering;
use std::fs::File;
use std::io::{ Lines, BufReader };

const A_VAL: u32 = 14;
const K_VAL: u32 = 13;
const Q_VAL: u32 = 12;
const J_VAL: u32 = 11;
const T_VAL: u32 = 10;

type HandCards = [u32; 5];

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq, Debug)]
struct Hand {
    cards: HandCards,
    typ: HandType,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.typ < other.typ {
            return Ordering::Less;
        } else if self.typ > other.typ {
            return Ordering::Greater;
        }

        if self.cards > other.cards {
            return Ordering::Greater;
        } else if self.cards < other.cards {
            return Ordering::Less;
        }
        return Ordering::Equal;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

pub(crate) fn solve_part1(input: Lines<BufReader<File>>)  -> String {
    let mut data = parse_input_part_1(input);
    data.sort();


    let mut total_winnings = 0;

    for (i, (_, bid)) in data.iter().enumerate() {
        let rank = i as u32 + 1;
        let winnings = rank * bid;
        total_winnings += winnings;
    }

    return total_winnings.to_string();
}

fn parse_input_part_1(input: Lines<BufReader<File>>) -> Vec<(Hand, u32)> {
    let mut hands = vec!();

    for line in input {
        let line = line.unwrap();
        let hand = convert_str_to_hand_part_1(&line[..5]);
        let bid = (&line[6..]).parse::<u32>().unwrap();

        hands.push((hand, bid));
    }
    hands
}

fn convert_str_to_hand_part_1(s: &str) -> Hand {
    let mut cards: HandCards = [0; 5];
    let mut counts = [0; 15];

    let mut chars = s.chars();

    for i in 0..5 {
        let c = chars.next().unwrap();
        let v = match c {
            'A' => A_VAL,
            'K' => K_VAL,
            'Q' => Q_VAL,
            'J' => J_VAL,
            'T' => T_VAL,
            '2'..='9' => c.to_digit(10).unwrap(),
            _ => panic!("Not a valid card!"),
        };
        counts[v as usize] += 1;
        cards[i] = v;
    }

    let mut counts: Vec<i32> = counts.iter().map(|x| *x).filter(|&x| x>1).collect();
    counts.sort();

    let typ ;

    if counts.is_empty() {
        typ = HandType::HighCard;
    } else if counts == vec!(2) {
        typ = HandType::OnePair;
    } else if counts == vec!(2, 2) {
        typ = HandType::TwoPair;
    } else if counts == vec!(3) {
        typ = HandType::ThreeOfAKind;
    } else if counts == vec!(2, 3) {
        typ = HandType::FullHouse;
    } else if counts == vec!(4) {
        typ = HandType::FourOfAKind;
    } else if counts == vec!(5) {
        typ = HandType::FiveOfAKind
    } else {
        panic!("Unexpected card type: {:?}", counts);
    }

    Hand{ cards, typ }
}

pub(crate) fn solve_part2(input: Lines<BufReader<File>>)  -> String {
    let mut data = parse_input_part_2(input);
    data.sort();


    let mut total_winnings = 0;

    for (i, (_, bid)) in data.iter().enumerate() {
        let rank = i as u32 + 1;
        let winnings = rank * bid;
        total_winnings += winnings;
    }

    return total_winnings.to_string();
}

fn parse_input_part_2(input: Lines<BufReader<File>>) -> Vec<(Hand, u32)> {
    let mut hands = vec!();

    for line in input {
        let line = line.unwrap();
        let hand = convert_str_to_hand_part_2(&line[..5]);
        let bid = (&line[6..]).parse::<u32>().unwrap();

        hands.push((hand, bid));
    }
    hands
}

fn convert_str_to_hand_part_2(s: &str) -> Hand {
    let mut cards: HandCards = [0; 5];
    let mut counts = [0; 15];

    let mut chars = s.chars();

    for i in 0..5 {
        let c = chars.next().unwrap();
        let v = match c {
            'A' => A_VAL,
            'K' => K_VAL,
            'Q' => Q_VAL,
            'J' => 0, // In part 2, J is now the weakest individual card
            'T' => T_VAL,
            '2'..='9' => c.to_digit(10).unwrap(),
            _ => panic!("Not a valid card!"),
        };
        counts[v as usize] += 1;
        cards[i] = v;
    }

    let joker_count = counts[0];

    let mut counts: Vec<i32> = counts[1..].iter().map(|x| *x).filter(|&x| x>1).collect();
    counts.sort();
    if joker_count > 0 {
        if counts.len() > 0 {
            let last_index = counts.len() - 1;
            counts[last_index] += joker_count;
        } else if joker_count < 5 {
            counts = vec!(joker_count + 1);
        } else {
            counts = vec!(5);
        }
    }

    let typ ;

    if counts.is_empty() {
        typ = HandType::HighCard;
    } else if counts == vec!(2) {
        typ = HandType::OnePair;
    } else if counts == vec!(2, 2) {
        typ = HandType::TwoPair;
    } else if counts == vec!(3) {
        typ = HandType::ThreeOfAKind;
    } else if counts == vec!(2, 3) {
        typ = HandType::FullHouse;
    } else if counts == vec!(4) {
        typ = HandType::FourOfAKind;
    } else if counts == vec!(5) {
        typ = HandType::FiveOfAKind
    } else {
        panic!("Unexpected card type: {:?}", counts);
    }

    Hand{ cards, typ }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decide_hand_type() {
        let hands = vec!(
            ("A2345", HandType::HighCard),
            ("AA234", HandType::OnePair),
            ("23445", HandType::OnePair),
            ("2T42T", HandType::TwoPair),
            ("23433", HandType::ThreeOfAKind),
            ("KQKQQ", HandType::FullHouse),
            ("JJJ4J", HandType::FourOfAKind),
            ("AAAAA", HandType::FiveOfAKind),
        );

        for (txt, expected) in hands {
            let hand = convert_str_to_hand_part_1(txt);
            assert_eq!(expected, hand.typ, "Expected {:?} for hand: {}, got: {:?}", expected, txt, hand.typ);
        }
    }

    #[test]
    fn test_compare_hands_by_card_strength() {
        let hand1 = convert_str_to_hand_part_1("2AAAA").cards;
        let hand2 = convert_str_to_hand_part_1("2AAAA").cards;
        let hand3 = convert_str_to_hand_part_1("AA2AA").cards;
        let hand4 = convert_str_to_hand_part_1("AAAAA").cards;

        assert!(hand1 == hand2);
        assert!(hand2 < hand3);
        assert!(hand4 > hand3);
    }

    #[test]
    fn test_compare_hand_types() {
        assert!(HandType::FiveOfAKind > HandType::FourOfAKind);
        assert!(HandType::FourOfAKind > HandType::FullHouse);
        assert!(HandType::FullHouse > HandType::ThreeOfAKind);
        assert!(HandType::ThreeOfAKind > HandType::TwoPair);
        assert!(HandType::TwoPair > HandType::OnePair);
        assert!(HandType::OnePair > HandType::HighCard);
    }

    #[test]
    fn test_compare_hand() {
        let hand1 = convert_str_to_hand_part_1("A2AAA");
        let hand2 = convert_str_to_hand_part_1("2AAAA");
        assert!(hand1 > hand2);

        let hand3 = convert_str_to_hand_part_1("AA222");
        let hand4 = convert_str_to_hand_part_1("222AA");
        assert!(hand3 > hand4);
        assert!(hand1 > hand3);
        assert!(hand2 > hand3);
        assert!(hand1 > hand4);
        assert!(hand3 > hand4);

        let hand5 = convert_str_to_hand_part_1("22222");
        let hand6 = convert_str_to_hand_part_1("83456");
        let hand7 = convert_str_to_hand_part_1("83456");
        assert!(hand5 > hand6);
        assert!(hand5 > hand1);
        assert_eq!(hand6, hand7);
    }

}
