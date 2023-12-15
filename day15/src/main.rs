use std::fs;
use std::collections::HashMap;
use crate::Operation::*;

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: usize,
}


#[derive(Debug, Clone)]
enum Operation {
    Remove { index: usize, label: String },
    Insert { index: usize, label: String, focal_length: usize },
}

fn main() {
    let samples = load("c:\\projects\\advent-of-code-2023\\data\\day15.sample.dat");
    let inputs = load("c:\\projects\\advent-of-code-2023\\data\\day15.dat");

    println!("Part 1");
    let sample_hashes = decode(&samples[0]);
    println!("[SAMPLE]: Parts {:?}", sample_hashes);
    println!("[SAMPLE]: {}", sample_hashes.iter().sum::<i32>()); // 1320

    let input_hashes = decode(&inputs[0]);
    println!("[INPUT]: {}", input_hashes.iter().sum::<i32>()); // 502139

    println!("Part 2");

    let sample_operations = decode_operations(&samples[0]);
    let sample_lenses = align(&sample_operations);
    let focus_power = calculate_focus_power(&sample_lenses);
    println!("[SAMPLE]: {}", focus_power); // 145

    let input_operations = decode_operations(&inputs[0]);
    let input_lenses = align(&input_operations);
    let focus_power = calculate_focus_power(&input_lenses);
    println!("[INPUT]: {}", focus_power); //
}

fn hash(current: i32, value: char) -> i32 {
    (current + value as i32) * 17 % 256
}

fn decode(input: &String) -> Vec<i32> {
    input.split(',')
        .map(|s| s.chars().fold(0, |current, c| hash(current, c)))
        .collect()
}

fn calculate_focus_power(lenses: &Vec<Vec<Lens>>) -> u64 {
    lenses.iter().enumerate().map(|(slot,tray) |  tray.iter().enumerate().map(|(index,lens)| ((slot+1) * (index+1) * lens.focal_length) as u64 ).sum::<u64>()).sum()
}

fn coerce_operation(s: String, map: &mut HashMap<String, i32>) -> Operation {
    let position = s.chars().position(|c| !c.is_alphabetic()).unwrap();
    let label = s[0..position].to_string();
    let index = match map.get(&*label) {
        Some(value) => *value,
        _ => {
            let value = label.chars().fold(0, |current, c| hash(current, c));
            map.insert(label.clone(), value);
            value
        }
    } as usize;

    match s.chars().nth(position).unwrap() {
        '-' => Remove { label, index },
        _ => Insert { label, index, focal_length: s[position + 1..position + 2].parse().unwrap() }
    }
}

fn decode_operations(input: &String) -> Vec<Operation> {
    let mut label_map = HashMap::<String, i32>::new();

    input.split(',')
        .map(|s| coerce_operation(s.to_string(), &mut label_map))
        .collect()
}

fn align(operations: &Vec<Operation>) -> Vec<Vec<Lens>> {
    let mut boxes = std::iter::repeat(vec![]).take(256).collect();

    operations.iter().for_each(|op| execute_operation(op.clone(), &mut boxes));

    boxes
}

fn execute_operation(op: Operation, boxes: &mut Vec<Vec<Lens>>) {
    match op {
        Remove { index, label } => {
            let tray = &mut boxes[index];
            match tray.iter().position(|l| l.label == label) {
                Some(i) => { tray.remove(i); }
                _ => ()
            };
        }
        Insert { index, label, focal_length } => {
            let tray = &mut boxes[index];
            match tray.iter().position(|l| l.label == label) {
                Some(i) => tray[i] = Lens { label, focal_length },
                _ => tray.push(Lens { label, focal_length })
            };
        }
    }
}

fn load(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect(&*format!("Unable to load the data file {path}"))
        .lines()
        .map(|l| l.to_string())
        .collect()
}