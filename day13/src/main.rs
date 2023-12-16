use std::cmp::min;
use std::fs;
use crate::Spot::*;
use crate::Symmetry::{Horizontal, Vertical};

#[derive(PartialEq, Clone, Debug)]
enum Spot {
    Ash,
    Rock,
}

#[derive(PartialEq, Clone, Debug)]
enum Symmetry {
    Vertical { left: usize, right: usize, span: usize },
    Horizontal { top: usize, bottom: usize, span: usize },
}

fn main() {
    let samples = load("c:\\projects\\advent-of-code-2023\\data\\day13.sample.dat");
    let inputs = load("c:\\projects\\advent-of-code-2023\\data\\day13.dat");

    println!("Part 1");
    let sample_patterns = parse_patterns(&samples);
    // println!("[SAMPLE]: {:?}", find_symmetry(&sample_patterns[0]));
    // println!("[SAMPLE]: {:?}", find_symmetry(&sample_patterns[1]));

    let sample_symmetries: Vec<Symmetry> = sample_patterns.iter().map(|p| find_symmetry(&p).unwrap()).collect();
    println!("[SAMPLE]: {}", sample_symmetries.iter().map(|s| score_symmetry(s)).sum::<u32>()); // 405

    let input_patterns = parse_patterns(&inputs);
    let input_symmetries: Vec<Symmetry> = input_patterns.iter().map(|p| find_symmetry(&p).unwrap()).collect();
    println!("[INPUT]: {}", input_symmetries.iter().map(|s| score_symmetry(s)).sum::<u32>()); // 29165
}

fn score_symmetry(symmetry: &Symmetry) -> u32 {
    match symmetry {
        Vertical { left, right, span } => (left + 1) as u32,
        Horizontal { top, bottom, span } => ((top + 1) * 100) as u32
    }
}

fn find_symmetry(matrix: &Vec<Vec<Spot>>) -> Option<Symmetry> {
    match find_horizontal_symmetry(&matrix) {
        None => find_vertical_symmetry(&matrix),
        x => x
    }
}

fn find_horizontal_symmetry(grid: &Vec<Vec<Spot>>) -> Option<Symmetry> {
    let mut symmetry = None;
    let mut i = 1;

    while symmetry.is_none() && i < grid.len() {
        let span = min(i, grid.len() - i);

        let top = stringify((&grid[i - span..i]).iter().cloned().rev().flatten().collect());
        let bottom = stringify((&grid[i..i + span]).iter().cloned().flatten().collect());

        if top == bottom {
            symmetry = Some(Symmetry::Horizontal { top: i - 1, bottom: i, span });
        }
        i += 1;
    }

    symmetry
}

fn transpose(matrix: &Vec<Vec<Spot>>) -> Vec<Vec<Spot>> {
    let mut transposed = Vec::new();
    let rows = matrix.len();
    let cols = matrix[0].len();

    for c in 0..cols {
        let mut row = Vec::new();
        for r in 0..rows {
            row.push(matrix[r][c].clone());
        }
        transposed.push(row);
    }

    transposed
}

fn stringify(slice: Vec<Spot>) -> String {
    slice.iter()
        .map(|s| match s {
            Ash => '.',
            Rock => '#'
        })
        .collect()
}

fn find_vertical_symmetry(grid: &Vec<Vec<Spot>>) -> Option<Symmetry> {
    match find_horizontal_symmetry(&transpose(grid)) {
        Some(Horizontal { top, bottom, span }) => Some(Vertical { left: top, right: bottom, span }),
        _ => None
    }
}

fn parse_patterns(lines: &Vec<String>) -> Vec<Vec<Vec<Spot>>> {
    let mut patterns = Vec::new();
    let mut buffer: Vec<String> = Vec::new();

    for l in lines {
        if l.is_empty() {
            patterns.push(parse_pattern(&buffer));
            buffer.clear();
        } else {
            buffer.push(l.clone());
        }
    }

    if !buffer.is_empty() {
        patterns.push(parse_pattern(&buffer));
    }

    patterns
}

fn parse_pattern(lines: &Vec<String>) -> Vec<Vec<Spot>> {
    lines.iter()
        .map(|l| l.chars().map(|c| match c {
            '#' => Rock,
            _ => Ash
        }).collect())
        .collect()
}


fn load(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect(&*format!("Unable to load the data file {path}"))
        .lines()
        .map(|l| l.to_string())
        .collect()
}

