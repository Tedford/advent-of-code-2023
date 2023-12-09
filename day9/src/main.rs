use std::fs;
use itertools::Itertools;

fn main() {
    let samples = load("c:\\projects\\advent-of-code-2023\\data\\day9.sample.dat");
    let inputs = load("c:\\projects\\advent-of-code-2023\\data\\day9.dat");

    let sample_series = samples.iter().map(|l|parse_series(l)).collect::<Vec<Vec<i32>>>();
    let input_series = inputs.iter().map(|l|parse_series(l)).collect::<Vec<Vec<i32>>>();

    println!("Part 1");
    let extrapolations = sample_series.iter().map(|s|extrapolate(s)).sum::<i32>();
    println!("[SAMPLE]: {}", extrapolations); // 114
    let extrapolations = input_series.iter().map(|s|extrapolate(s)).sum::<i32>();
    println!("[INPUT]: {}", extrapolations); // 114
    println!("Part 2");
    let extrapolations = sample_series.iter().map(|s|reverse_extrapolate(s)).sum::<i32>();
    println!("[SAMPLE]: {}", extrapolations); // 2
    let extrapolations = input_series.iter().map(|s|reverse_extrapolate(s)).sum::<i32>();
    println!("[INPUT]: {}", extrapolations); // 2
}

fn extrapolate(series: &Vec<i32>) -> i32 {
    let differences = (0..series.len()-1).map(|i|series[i+1] - series[i] ).collect::<Vec<i32>>();
    let next = match  differences.iter().group_by(|e|*e).into_iter().count(){
         1 => differences[0],
         _ => extrapolate(&differences)
     };
    series[series.len()-1] + next
}

fn reverse_extrapolate(series: &Vec<i32>) -> i32 {
    let differences = (0..series.len()-1).map(|i|series[i+1] - series[i] ).collect::<Vec<i32>>();
    let next = match  differences.iter().group_by(|e|*e).into_iter().count(){
        1 => differences[0],
        _ => reverse_extrapolate(&differences)
    };
    series[0] - next
}

fn parse_series( lines: &String ) -> Vec<i32>{

    lines.split(' ')
        .map(|s|s.parse::<i32>())
        .filter_map(|i|i.ok())
        .collect()
}

fn load(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect(&*format!("Unable to load the data file {path}"))
        .lines()
        .map(|l| l.to_string())
        .collect()
}