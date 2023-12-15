use std::fs;
use crate::Spot::*;

#[derive(Debug, Clone, PartialEq)]
enum Spot {
    RoundRock,
    CubeRock,
    Clear,
}

enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let samples = load("c:\\projects\\advent-of-code-2023\\data\\day14.sample.dat");
    let inputs = load("c:\\projects\\advent-of-code-2023\\data\\day14.dat");
    println!("Part 1");

    let sample_map = parse_map(&samples);
    let tilted_sample_map = tilt_north(&sample_map);
    println!("[SAMPLE]: Original\n{}",pretty_print(&sample_map));
    println!("[SAMPLE]: Titled\n{}",pretty_print(&tilted_sample_map));
    println!("[SAMPLE]: {}", calculate_load(&tilted_sample_map)); // 136

    let input_map = parse_map(&inputs);
    let tilted_input_map = tilt_north(&input_map);
    println!("[INPUT]: {}", calculate_load(&tilted_input_map));

    println!("Part 2");
    let tilted_sample_map = spin_cycle(&sample_map, 1000);
    println!("[SAMPLE]: Titled  1000 cycle\n{}",pretty_print(&tilted_sample_map));
    println!("[SAMPLE]: {}", calculate_load(&tilted_sample_map)); // 64

    let tilted_input_map = spin_cycle(&input_map, 1000);
    println!("[input]: {}", calculate_load(&tilted_input_map)); //
}

fn pretty_print( map: &Vec<Vec<Spot>>) -> String{
    let mut output = "".to_string();

    for row in map {
        for col in row {
            output.push_str(match col { RoundRock=> "O", CubeRock => "#", _ => "."});
        }
        output.push_str("\n");
    }

    output
}

fn load(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect(&*format!("Unable to load the data file {path}"))
        .lines()
        .map(|l| l.to_string())
        .collect()
}

fn create_empty_map(rows: usize, cols: usize) -> Vec<Vec<Spot>> {
    std::iter::repeat(std::iter::repeat(Clear).take(cols).collect::<Vec<Spot>>()).take(rows).collect::<Vec<Vec<Spot>>>()
}

fn parse_map(lines: &Vec<String>) -> Vec<Vec<Spot>> {
    let cols = lines[0].len();
    let rows = lines.len();
    let mut map = create_empty_map(rows, cols);

    lines.iter().enumerate().for_each(|(row, line)| line.chars().enumerate().for_each(|(col, c)|
        match c {
            'O' => map[row][col] = RoundRock,
            '#' => map[row][col] = CubeRock,
            _ => ()
        }));

    map
}

fn spin_cycle(map: &Vec<Vec<Spot>>, cycles: u32) -> Vec<Vec<Spot>> {
    let mut tilted_map= map.clone();

    for cycle in 0..cycles {
        tilted_map = tilt_north(&tilted_map);
        // println!("N\n{}",pretty_print(&tilted_map));
        tilted_map = tilt_west(&tilted_map);
        // println!("W\n{}",pretty_print(&tilted_map));
        tilted_map = tilt_south(&tilted_map);
        // println!("S\n{}",pretty_print(&tilted_map));
        tilted_map = tilt_east(&tilted_map);
        // println!("E\n{}",pretty_print(&tilted_map));
    }

    tilted_map
}

fn tilt_north(map: &Vec<Vec<Spot>>) -> Vec<Vec<Spot>> {
    let rows = map.len();
    let cols = map.len();
    let mut tilted_map = create_empty_map(rows, cols);

    for (r, row) in map.iter().enumerate() {
        for (c, spot) in row.iter().enumerate() {
            match spot {
                CubeRock => {
                    tilted_map[r][c] = CubeRock;
                }
                RoundRock => {
                    let mut dist = 1;
                    let mut done = r == 0;
                    while !done {
                        done = dist > r || match tilted_map[r - dist][c] {
                            Clear => {
                                dist += 1;
                                false
                            }
                            _ => true
                        };
                    }
                    tilted_map[r + 1 - dist][c] = RoundRock;
                }
                _ => ()
            }
        }
    }

    tilted_map
}

fn tilt_west(map: &Vec<Vec<Spot>>) -> Vec<Vec<Spot>> {
    let rows = map.len();
    let cols = map.len();
    let mut tilted_map = create_empty_map(rows, cols);

    for (r, row) in map.iter().enumerate() {
        for (c, spot) in row.iter().enumerate() {
            match spot {
                CubeRock => {
                    tilted_map[r][c] = CubeRock;
                }
                RoundRock => {
                    let mut dist = 1;
                    let mut done = c == 0;
                    while !done {
                        done = dist > c || match tilted_map[r][c - dist] {
                            Clear => {
                                dist += 1;
                                false
                            }
                            _ => true
                        };
                    }
                    tilted_map[r][c + 1 - dist] = RoundRock;
                }
                _ => ()
            }
        }
    }

    tilted_map
}

fn tilt_south(map: &Vec<Vec<Spot>>) -> Vec<Vec<Spot>> {
    let rows = map.len();
    let cols = map.len();
    let mut tilted_map = create_empty_map(rows, cols);

    for (r, row) in map.iter().enumerate().rev() {
        for (c, spot) in row.iter().enumerate() {
            match spot {
                CubeRock => {
                    tilted_map[r][c] = CubeRock;
                }
                RoundRock => {
                    let mut dist = 1;
                    let mut done = r == rows - 1;
                    while !done {
                        done =dist + r >= rows || match tilted_map[r + dist][c] {
                            Clear => {
                                dist += 1;
                                false
                            }
                            _ => true
                        };
                    }
                    tilted_map[r + dist - 1][c] = RoundRock;
                }
                _ => ()
            }
        }
    }

    tilted_map
}

fn tilt_east(map: &Vec<Vec<Spot>>) -> Vec<Vec<Spot>> {
    let rows = map.len();
    let cols = map.len();
    let mut tilted_map = create_empty_map(rows, cols);

    for (r, row) in map.iter().enumerate() {
        for (c, spot) in row.iter().enumerate().rev() {
            match spot {
                CubeRock => {
                    tilted_map[r][c] = CubeRock;
                }
                RoundRock => {
                    let mut dist = 1;
                    let mut done = c == cols - 1;
                    while !done {
                        done = dist + c >= cols || match tilted_map[r][c + dist] {
                            Clear => {
                                dist += 1;
                                false
                            }
                            _ => true
                        };
                    }
                    tilted_map[r][c + dist - 1] = RoundRock;
                }
                _ => ()
            }
        }
    }

    tilted_map
}

fn calculate_load(map: &Vec<Vec<Spot>>) -> u32 {
    let max_load = map.len();

    map.iter().enumerate().map(|(row, rocks)| (rocks.iter().filter(|r| *r == &RoundRock).count() * (max_load - row)) as u32).sum()
}