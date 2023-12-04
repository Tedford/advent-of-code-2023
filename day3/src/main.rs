use std::fs;
use std::hash::{Hash};
use itertools::Itertools;

#[allow(unused)]
struct Symbol {
    row: i32,
    column: i32,
    length: i32,
    symbol: char
}

#[derive(Copy,Clone,Hash,Eq,PartialEq)]
struct Number {
    row: i32,
    start: i32,
    end: i32,
    value: i32
}

fn main() {
    let samples = load("c:\\projects\\advent-of-code-2023\\data\\day3.sample.dat");
    let input = load("c:\\projects\\advent-of-code-2023\\data\\day3.dat");

    let sample_symbols = find_symbols(&samples);
    let sample_numbers = find_numbers(&samples);
    let sample_parts = find_parts(&sample_symbols, &sample_numbers);

    println!("Part 1");
    println!("[SAMPLE]: {}", sum_parts(&sample_parts)); // 4361

    let input_symbols = find_symbols(&input);
    let input_numbers = find_numbers(&input);
    let input_parts = find_parts(&input_symbols, &input_numbers);
    println!("[INPUT]: {}", sum_parts(&input_parts)); // 528819

    println!("Part 2");
    let sample_gear_ratios = find_gear_ratios(&sample_symbols, &sample_numbers);
    println!("[SAMPLE]: {}", sample_gear_ratios.iter().sum::<i32>()); // 467835
    let input_gear_ratios = find_gear_ratios(&input_symbols, &input_numbers);
    println!("[Input]: {}", input_gear_ratios.iter().sum::<i32>()); // 80403602
}

fn sum_parts( parts: &Vec<Number>) -> i32 {
    parts.iter().map(|p|p.value).sum()
}

fn find_gear_ratios(symbols: &Vec<Symbol>, numbers: &Vec<Number>) -> Vec<i32>{
    let mut ratios = Vec::new();

    for symbol in symbols.iter().filter(|s|s.symbol == '*') {
        let parts = numbers.iter().filter(|n|is_adjacent(symbol, n)).map(|n|n.value).collect_vec();
        if parts.len() > 1 {
            ratios.push(parts.iter().product());
        }
    }

     ratios
}

fn is_adjacent(s: &Symbol, n: &Number) -> bool {
    // (n.row == s.row && n.end  == s.column -1 ) || //same row proceeding
    //     (n.row == s.row && n.start == s.column + 1) || // same row trailing
    //     (n.row == s.row -1 && n.start <= s.column && n.end >= s.column) || // previous row covering
    //     (n.row == s.row -1 && n.end == s.column-1) || // previous row left diagonal
    //         (n.row == s.row -1 && n.start == s.column+1) || // previous row right diagonal
    //     (n.row == s.row +1 && n.start <= s.column && n.end >= s.column) || // next row covering
    //     (n.row == s.row +1 && n.end == s.column-1) || // next row left diagonal
    //     (n.row == s.row +1 && n.start == s.column+1)  // next row right diagonal
    let span = n.start..=n.end;
    (n.row-1..=n.row+1).contains (&s.row) && (span.contains(&s.column) || span.contains(&(s.column-1)) || span.contains(&(s.column+1)))
}
fn find_parts (symbols: &Vec<Symbol>, numbers: &Vec<Number>) -> Vec<Number> {
    let mut parts = Vec::<Number>::new();

    for symbol in symbols {
        for neighbor in numbers.iter().filter(|n| is_adjacent(symbol,n) ) {
            if !parts.contains(neighbor){
                parts.push(neighbor.clone());
            }
        }

    }

    parts
}

fn find_symbols(raw: &Vec<String>) -> Vec<Symbol>{
    let mut symbols = Vec::new();

    let mut row = 0;
    for line in raw {
        let mut column = 0;
        let chars = line.chars();
        for c in chars {
            if c != '.' && !(c.is_whitespace() || c.is_alphanumeric()) {
                symbols.push(Symbol {row, column, length:1, symbol: c})
            }
            column += 1;
        }
        row +=1;
    }

    symbols
}

fn find_numbers(raw: &Vec<String>) -> Vec<Number>{
    let mut numbers = Vec::new();
    let mut buffer = Vec::new();
    let mut row = 0;
    for line in raw {

        let mut column = 0;
        let chars = line.chars();
        for c in chars {
            let in_number = c.is_digit(10);
            if in_number {
                buffer.push(c);
            }

            if !in_number && buffer.len() > 0 {
                let length = buffer.len() as i32;
                let value = buffer.iter().collect::<String>().parse::<i32>().unwrap();
                buffer.clear();
                numbers.push(Number{row, start: column-length, end: column-1, value});
            }

            column += 1;
        }
        row +=1;
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

