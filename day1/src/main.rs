use std::fs;

struct Token {
    item: i32,
    position: Option<usize>
}

fn main() {
    println!("Day 1");

    let input = load("c:\\projects\\advent-of-code-2023\\data\\day1.dat");
    let sample1 = load("c:\\projects\\advent-of-code-2023\\data\\day1.sample.1.dat");
    let sample2 = load("c:\\projects\\advent-of-code-2023\\data\\day1.sample.2.dat");
    let input_coordinates = map_to_coordinate(&input);
    let sample1_coordinates = map_to_coordinate(&sample1);

    println!("[Coordinate1] Sample: {}", sample1_coordinates.iter().sum::<i32>());
    println!("[Coordinate1] Input: {}", input_coordinates.iter().sum::<i32>());

    let input_coordinates2 = map_to_coordinate2(&input);
    let sample2_coordinates = map_to_coordinate2(&sample2);

    println!("[Coordinate2] Sample: {}", sample2_coordinates.iter().sum::<i32>());
    println!("[Coordinate2] Input: {}", input_coordinates2.iter().sum::<i32>());
}


fn load(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("Unable to load the data file")
        .lines()
        .map(|l|l.to_string())
        .collect()
}

fn map_to_coordinate(inputs: &Vec<String>) -> Vec<i32> {
    inputs.iter().map(|l| {
        let first = l.chars().find(|c| c.is_digit(10));
        let last = l.chars().rfind(|c| c.is_digit(10));

        format!("{}{}", first.unwrap(),last.unwrap()).parse::<i32>().unwrap()  })
        .collect()
}

fn map_to_coordinate2(inputs: &Vec<String>) -> Vec<i32> {
    let numbers = ["one", "two", "three", "four", "five","six", "seven", "eight", "nine"];
    inputs.iter().map(|l| {
        let mut tokens  = numbers.iter().flat_map(|t| {
            let mut matches = Vec::<Token>::new();
            let mut search = &l[0..];
            let mut offset = 0;
            let digit = numbers.iter().position(|i|i.to_string() == t.to_string()).unwrap()+1;

            while search.len() > 0 {
                match search.find(t) {
                    None => { search = ""},
                    Some(x) => {
                        matches.push(Token {item: digit as i32, position: Option::Some(x + offset)});
                        search = &search[x+t.len()..];
                        offset += x + t.len();
                    }
                }
            }
            matches
        }).collect::<Vec::<Token>>();

        let first_digit = l.chars().position(|c|c.is_digit(10));
        let last_digit = match l.chars().rev().position(|c|c.is_digit(10)) {
            Some(x) =>Option::Some(  l.len() - x - 1),
            _ => None
        };

        let first;
        let last;

        if tokens.len() > 0 {
            tokens.sort_by(|a, b| a.position.cmp(&b.position));

            if first_digit == None {
                first = tokens[0].item;
            }
            else {
                first = if tokens[0].position < first_digit { tokens[0].item } else { l.chars().nth(first_digit.unwrap()).unwrap().to_digit(10).unwrap() as i32 };
            }

            if last_digit == None {
                last = tokens.last().unwrap().item;
            }
            else {
                last = if tokens.last().unwrap().position > last_digit { tokens.last().unwrap().item } else { l.chars().nth(last_digit.unwrap() ).unwrap().to_digit(10).unwrap() as i32 };
            }
        } else {
            first = l.chars().nth(first_digit.unwrap()).unwrap().to_digit(10).unwrap() as i32;
            last = l.chars().nth( last_digit.unwrap()).unwrap().to_digit(10).unwrap() as i32;
        }

        format!("{}{}", first,last).parse::<i32>().unwrap()  })
        .collect()
}