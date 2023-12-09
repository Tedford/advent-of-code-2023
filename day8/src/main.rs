use std::collections::HashMap;
use std::fs;
use crate::Direction::{Left, Right};
use num::Integer;

#[derive(Clone, Debug)]
struct Branch {
    name: String,
    left: String,
    right: String,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Map {
    directions: Vec<Direction>,
    branches: HashMap<String, Branch>,
}


fn main() {
    let samples = load("c:\\projects\\advent-of-code-2023\\data\\day8.sample.dat");
    let inputs = load("c:\\projects\\advent-of-code-2023\\data\\day8.dat");

    println!("Part 1");
    let sample_map = parse_map(&samples).expect("Unable to parse the sample map");
    println!("[SAMPLE]: {:?}", sample_map);

    println!("[SAMPLE]: Journey Steps: {}", walk(&sample_map));

    let input_map = parse_map(&inputs).expect("Unable to parse the input map");
    println!("[INPUT]: Journey Steps: {}", walk(&input_map)); // 16271

    println!("Part 2");
    let samples2 = load("c:\\projects\\advent-of-code-2023\\data\\day8.sample2.dat");
    let sample2_map = parse_map(&samples2).expect("Unable to parse the sample map");
    println!("[SAMPLE]: Journey Steps: {}", ghost_walk(&sample2_map)); // 6
    println!("[INPUT]: Journey Steps: {}", ghost_walk(&input_map));
}

fn walk(map: &Map) -> i64 {
    walk_branch(map.branches.get("AAA").expect("The starting location AAA was not found"), map, |location| location == "ZZZ")
}

fn walk_branch(branch: &Branch, map: &Map, success: impl Fn(&str)->bool) -> i64 {

    let mut steps = 0;
    let mut at_end = false;
    let mut i = 0;

    let mut b = branch;
    while !at_end {
        steps += 1;

        let location = match &map.directions[i] {
            Left => &b.left,
            _ => &b.right
        };

        if success(location) {
            at_end = true;
        } else {
            b = map.branches.get(location).expect(&*format!("The next location {location} was not found"));
            i += 1;
            i %= map.directions.len();
        }
    }

    steps
}


fn ghost_walk(map: &Map) -> i64 {
    map.branches
        .iter()
        .filter(|(k, _)| k.ends_with('A'))
        .map(|(_, v)| walk_branch(v, map, |location|location.ends_with('Z')))
        .reduce(|a,b|a.lcm(&b))
        .unwrap()
}

fn parse_map(lines: &Vec<String>) -> Result<Map, String> {
    let directions = lines[0].chars().map(|c| match c {
        'L' => Left,
        _ => Right
    }).collect();
    let mut branches = HashMap::new();

    for b in lines[2..].iter().map(|l| parse_branch(l)) {
        let b2 = b.clone();
        branches.insert(b2.name, b);
    }

    Ok(Map { directions, branches })
}

fn parse_branch(line: &String) -> Branch {
    let name = line[0..3].to_string();
    let left = line[7..10].to_string();
    let right = line[12..15].to_string();

    Branch { name, left, right }
}

fn load(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect(&*format!("Unable to load the data file {path}"))
        .lines()
        .map(|l| l.to_string())
        .collect()
}
