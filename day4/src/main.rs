use std::fs;
use std::iter::Map;

#[allow(unused)]
struct Card {
    id: i32,
    numbers: Vec<i32>,
    winning_numbers: Vec<i32>
}

#[allow(unused)]
struct ScoredCard {
    card: Card,
    matches: i32,
    score: i32
}

fn main() {
    let samples = load("c:\\projects\\advent-of-code-2023\\data\\day4.sample.dat");
    let input = load("c:\\projects\\advent-of-code-2023\\data\\day4.dat");

    println!("Part 1:");
    let sample_cards =samples.iter().map(|l|parse_card(&l)).map(|c|score(c)).collect();
    println!("[SAMPLE]: {}", total_score(&sample_cards)); // 13
    let input_cards =input.iter().map(|l|parse_card(&l)).map(|c|score(c)).collect();
    println!("[INPUT]: {}", total_score(&input_cards)); // 25651

    println!("Part 2:");
    println!("[SAMPLE]: {}", accumulate_cards(&sample_cards)); // 30
    println!("[INPUT]: {}", accumulate_cards(&input_cards)); // 19499881
}

fn accumulate_cards(cards: &Vec<ScoredCard>) -> i32 {
    // let total = cards.len();
    let mut accumulated = 0;

    for c in cards {
        accumulated += find_wins(c, cards);
    }

    accumulated + cards.len() as i32
}

fn find_wins(card: &ScoredCard, cards: &Vec<ScoredCard> ) -> i32 {
    let mut wins;
    let headroom = cards.len() as i32 - (card.card.id + card.matches);

    if headroom < 0 {
        wins =  card.matches - headroom.abs();
    }
    else {
        wins = card.matches;
    }

    for i in 1..=card.matches {
        let nth = card.card.id + i - 1;
        wins += match cards.iter().nth(nth as usize) {
            Some(x) =>find_wins( x ,cards),
            _ => 0
        }
    }

    wins
}

fn total_score(cards: &Vec<ScoredCard>) -> i32 {
    Map::sum(cards.iter().map(|c| c.score))
}

fn parse_card(line: &String) -> Card {
    // println!("Parsing {}",line);
    let space = line.chars().position(|c| c == ' ').unwrap();
    let header = line.chars().position(|c| c==':').unwrap();
    let id = line[space+1..header].trim().parse::<i32>().unwrap();
    let divider = line.chars().position(|c| c== '|').unwrap();
    let numbers = extract_numbers(line[header+1..divider].to_string());
    let winning_numbers = extract_numbers(line[divider+1..].to_string());
    Card {id, numbers, winning_numbers}
}

fn score(card: Card)-> ScoredCard {
    let mut matches = 0;
    let mut score =0;

    for n in &card.numbers {
        if card.winning_numbers.contains(&n){
            matches += 1;
        }
    }
    if matches > 0 {
        score += 1;
    }

    for _ in 1..matches {
        score *= 2;
    }

    ScoredCard{ card, matches, score }
}

fn extract_numbers(s: String) -> Vec<i32> {
    let mut numbers = Vec::new();
    let mut in_number;
    let mut buffer = Vec::new();
    for c in s.chars() {
        if c.is_digit(10){
            in_number = true;
            buffer.push(c);
        }
        else {
            in_number = false;
        }

        if !in_number && buffer.len() > 0 {
            numbers.push(buffer.iter().collect::<String>().parse::<i32>().unwrap());
            buffer.clear();
        }
    }

    if buffer.len() > 0 {
        numbers.push(buffer.iter().collect::<String>().parse::<i32>().unwrap());
    }

    numbers
}

fn load(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("Unable to load the data file")
        .lines()
        .map(|l| l.to_string())
        .collect()
}
