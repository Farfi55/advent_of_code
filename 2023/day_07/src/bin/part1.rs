fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("RESULT:\n{}", result);
}

fn part1(input: &str) -> usize {
    let mut hands: Vec<Hand> = input.lines().map(|l| Hand::new(l)).collect();
    hands.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    let mut result = 0;
    for (i, hand) in hands.iter().enumerate() {
        result += hand.bid * (i + 1);
    }
    result
}

#[derive(Debug)]
struct Hand {
    cards_value: Vec<u8>,
    hand_type: HandType,
    bid: usize,
}

impl Hand {
    fn new(input: &str) -> Self {
        let mut parts = input.split_whitespace();
        let cards: Vec<char> = parts.next().unwrap().chars().collect();
        let bid: usize = parts.next().unwrap().parse().unwrap();

        let cards_value = cards
            .iter()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                _ => c.to_digit(10).unwrap() as u8,
            })
            .collect::<Vec<_>>();

        let hand_type = HandType::from_cards_value(&cards_value);

        Hand {
            cards_value,
            hand_type,
            bid,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards_value == other.cards_value
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let type_cmp = self.hand_type.partial_cmp(&other.hand_type).unwrap();

        if type_cmp == std::cmp::Ordering::Equal {
            for (v1, v2) in self.cards_value.iter().zip(other.cards_value.iter()) {
                let card_cmp = v1.partial_cmp(v2);
                if card_cmp != Some(std::cmp::Ordering::Equal) {
                    return card_cmp;
                }
            }
            return None;
        } else {
            return Some(type_cmp);
        }
    }
}

#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn type_score(&self) -> usize {
        match self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }

    fn from_cards_value(cards_value: &Vec<u8>) -> HandType {
        let mut counts: [i8; 15] = [0; 15];
        for v in cards_value {
            counts[(*v) as usize] += 1;
        }

        counts.sort_unstable_by(|a, b| b.cmp(a));
        let count_1st = counts[0];
        let count_2nd = counts[1];

        match (count_1st, count_2nd) {
            (5, _) => HandType::FiveOfAKind,
            (4, _) => HandType::FourOfAKind,
            (3, 2) => HandType::FullHouse,
            (3, _) => HandType::ThreeOfAKind,
            (2, 2) => HandType::TwoPair,
            (2, _) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

impl PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        self.type_score() == other.type_score()
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.type_score().partial_cmp(&other.type_score())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

        let result = part1(input);
        assert_eq!(result, 6440);
    }

    #[test]
    fn test_hand_type() {
        let inputs = vec![
            (HandType::FiveOfAKind, HandType::FourOfAKind),
            (HandType::FourOfAKind, HandType::FullHouse),
            (HandType::FullHouse, HandType::ThreeOfAKind),
            (HandType::ThreeOfAKind, HandType::TwoPair),
            (HandType::TwoPair, HandType::OnePair),
            (HandType::OnePair, HandType::HighCard),
        ];

        for (ht1, ht2) in inputs {
            assert_eq!(ht1 > ht2, true);
        }
    }

    #[test]
    fn test_parse() {
        let inputs = [
            ("32T3K 765", vec![3, 2, 10, 3, 13], HandType::OnePair, 765),
            (
                "T55J5 684",
                vec![10, 5, 5, 11, 5],
                HandType::ThreeOfAKind,
                684,
            ),
            ("KK677 28", vec![13, 13, 6, 7, 7], HandType::TwoPair, 28),
            (
                "KTJJT 220",
                vec![13, 10, 11, 11, 10],
                HandType::TwoPair,
                220,
            ),
            (
                "QQQJA 483",
                vec![12, 12, 12, 11, 14],
                HandType::ThreeOfAKind,
                483,
            ),
        ];

        for (input, cards_value, hand_type, bid) in inputs.into_iter() {
            let hand = Hand::new(input);
            assert_eq!(hand.cards_value, cards_value);
            assert_eq!(hand.hand_type, hand_type);
            assert_eq!(hand.bid, bid);
        }
    }
}
