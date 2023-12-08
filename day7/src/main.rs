use std::cmp::Ordering;
use std::{fmt, fs};

extern crate itertools;

use itertools::Itertools;

#[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Debug)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

#[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Debug)]
enum Card2 {
    Ace = 14,
    King = 13,
    Queen = 12,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Joker = 1,

}

#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
    bid: i64,
}

#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
struct Hand2 {
    cards: [Card2; 5],
    hand_type: HandType,
    bid: i64,
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "type: {:?} bid: {:3} cards: {:?}", self.hand_type, self.bid, self.cards)
    }
}

impl fmt::Display for Hand2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "type: {:?} bid: {:3} cards: {:?}", self.hand_type, self.bid, self.cards)
    }
}

fn main() {
    let samples = load("c:\\projects\\advent-of-code-2023\\data\\day7.sample.dat");
    let inputs = load("c:\\projects\\advent-of-code-2023\\data\\day7.dat");

    println!("Part 1");
    let mut sample_hands = parse_hand(&samples).expect("Unable to parse the sample data");
    sample_hands.sort_by(|a, b| compare_hands(a, b));
    let winnings = sample_hands.iter().enumerate().map(|(i, e)| e.bid * (i as i64 + 1)).sum::<i64>();

    for hand in sample_hands {
        println!("{}", hand);
    }
    // println!("Samples: {:?}",sample_hands);

    println!("[SAMPLE]: Winnings: {winnings}"); // 6440

    let mut input_hands = parse_hand(&inputs).expect("Unable to parse the input data");
    input_hands.sort_by(|a, b| compare_hands(a, b));
    let winnings = input_hands.iter().enumerate().map(|(i, e)| e.bid * (i as i64 + 1)).sum::<i64>();

    for hand in input_hands {
        println!("{}", hand);
    }

    println!("[INPUT]: Winnings: {winnings}"); // 255048101

    println!("Part 2");
    let mut sample_hands2 = parse_hand2(&samples).expect("Unable to parse the sample data");
    sample_hands2.sort_by(|a, b| compare_hands2(a, b));
    let winnings = sample_hands2.iter().enumerate().map(|(i, e)| e.bid * (i as i64 + 1)).sum::<i64>();

    for hand in sample_hands2 {
        println!("{}", hand);
    }

    println!("[SAMPLE]: Winnings: {winnings}"); // 5905

    let mut input_hands2 = parse_hand2(&inputs).expect("Unable to parse the sample data");
    input_hands2.sort_by(|a, b| compare_hands2(a, b));
    let winnings = input_hands2.iter().enumerate().map(|(i, e)| e.bid * (i as i64 + 1)).sum::<i64>();

    for hand in input_hands2 {
        println!("{}", hand);
    }

    println!("[SAMPLE]: Winnings: {winnings}"); //
}

fn compare_hands(hand1: &Hand, hand2: &Hand) -> Ordering {
    if hand1.hand_type != hand2.hand_type {
        (hand1.hand_type as i32).cmp(&(hand2.hand_type as i32))
    } else {
        for i in 0..5 {
            if hand1.cards[i] != hand2.cards[i] {
                return hand1.cards[i].cmp(&hand2.cards[i]);
            }
        }
        Ordering::Equal
    }
}

fn compare_hands2(hand1: &Hand2, hand2: &Hand2) -> Ordering {
    if hand1.hand_type != hand2.hand_type {
        (hand1.hand_type as i32).cmp(&(hand2.hand_type as i32))
    } else {
        for i in 0..5 {
            if hand1.cards[i] != hand2.cards[i] {
                return hand1.cards[i].cmp(&hand2.cards[i]);
            }
        }
        Ordering::Equal
    }
}

fn parse_hand(lines: &Vec<String>) -> Result<Vec<Hand>, String> {
    let result = lines.iter().map(|l| {
        let mut cards = l[0..5]
            .chars()
            .map(|c|
                match c {
                    'A' => Some(Card::Ace),
                    'K' => Some(Card::King),
                    'Q' => Some(Card::Queen),
                    'J' => Some(Card::Jack),
                    'T' => Some(Card::Ten),
                    '9' => Some(Card::Nine),
                    '8' => Some(Card::Eight),
                    '7' => Some(Card::Seven),
                    '6' => Some(Card::Six),
                    '5' => Some(Card::Five),
                    '4' => Some(Card::Four),
                    '3' => Some(Card::Three),
                    '2' => Some(Card::Two),
                    _ => None
                }
            ).filter(|c| c.is_some())
            .map(|c| c.unwrap())
            .collect::<Vec<Card>>();

        let mut hand_type = HandType::HighCard;

        let mut c2 = cards.clone();
        c2.sort();
        c2.reverse();

        let mut groups = c2
            .iter()
            .group_by(|e| (*e).clone())
            .into_iter()
            .map(|(e, group)| (e, group.count() as i32))
            .collect::<Vec<(Card, i32)>>();

        groups.sort_by(|(_, a_count), (_, b_count)| b_count.cmp(&a_count));

        if groups.len() == 1 {
            hand_type = HandType::FiveOfAKind
        } else if groups.len() == 2 {
            if groups[0].1 == 4 {
                hand_type = HandType::FourOfAKind
            } else {
                hand_type = HandType::FullHouse;
            }
        } else if groups.len() == 3 {
            if groups[0].1 == 3 {
                hand_type = HandType::ThreeOfAKind
            } else {
                hand_type = HandType::TwoPair
            }
        } else if groups.len() == 4 {
            hand_type = HandType::OnePair
        }

        Hand {
            cards: cards.try_into().unwrap(),
            bid: l[6..].parse::<i64>().expect(&*format!("Unable to parse the bid from {l}")),
            hand_type,
        }
    }
    ).collect();

    Ok(result)
}

fn parse_hand2(lines: &Vec<String>) -> Result<Vec<Hand2>, String> {
    let result = lines.iter().map(|l| {
        let mut cards = l[0..5]
            .chars()
            .map(|c|
                match c {
                    'A' => Some(Card2::Ace),
                    'K' => Some(Card2::King),
                    'Q' => Some(Card2::Queen),
                    'J' => Some(Card2::Joker),
                    'T' => Some(Card2::Ten),
                    '9' => Some(Card2::Nine),
                    '8' => Some(Card2::Eight),
                    '7' => Some(Card2::Seven),
                    '6' => Some(Card2::Six),
                    '5' => Some(Card2::Five),
                    '4' => Some(Card2::Four),
                    '3' => Some(Card2::Three),
                    '2' => Some(Card2::Two),
                    _ => None
                }
            ).filter(|c| c.is_some())
            .map(|c| c.unwrap())
            .collect::<Vec<Card2>>();

        let mut hand_type = HandType::HighCard;

        let mut c2 = cards.clone();
        c2.sort();
        c2.reverse();

        let mut groups = c2
            .iter()
            .group_by(|e| (*e).clone())
            .into_iter()
            .map(|(e, group)| (e, group.count() as i32))
            .collect::<Vec<(Card2, i32)>>();

        groups.sort_by(|(_, a_count), (_, b_count)| b_count.cmp(&a_count));

        let jokers = groups
            .iter()
            .find(|(c, n)| c == &Card2::Joker)
            .map(|(_, n)| n)
            .unwrap_or(&0);

        if groups.len() == 1 {
            hand_type = HandType::FiveOfAKind;
        } else if groups.len() == 2 {
            hand_type = match jokers {
                0 => {
                    if groups[0].1 == 4 {
                        HandType::FourOfAKind
                    } else {
                        HandType::FullHouse
                    }
                }
                _ => HandType::FiveOfAKind
            }
        } else if groups.len() == 3 {
            hand_type = match jokers {
                3 => HandType::FourOfAKind,
                2 => HandType::FourOfAKind,
                1 =>  if groups[0].1 == 2 {
                    HandType::FullHouse
                } else {
                    HandType::FourOfAKind
                },
                _ => if groups[0].1 == 3 {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            };
        } else if groups.len() == 4 {
            hand_type = match jokers {
                2 => HandType::ThreeOfAKind,
                1 => HandType::ThreeOfAKind,
                _ => HandType::OnePair
            };
        } else {
            if *jokers > 0 {
                hand_type = HandType::OnePair;
            }
        }

        Hand2 {
            cards: cards.try_into().unwrap(),
            bid: l[6..].parse::<i64>().expect(&*format!("Unable to parse the bid from {l}")),
            hand_type,
        }
    }
    ).collect();

    Ok(result)
}

fn load(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect(&*format!("Unable to load the data file {path}"))
        .lines()
        .map(|l| l.to_string())
        .collect()
}
