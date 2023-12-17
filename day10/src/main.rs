use std::{fmt, fs};
use crate::Boundary::*;
use crate::CardinalDirection::*;
use crate::Direction::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum CardinalDirection {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Boundary {
    Inside,
    Outside,
    Loop,
    Edge,
    Unknown,
}


#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Start,
    Ground,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Node {
    row: usize,
    col: usize,
    direction: Direction,
    distance: u32,
    boundary: Boundary,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match (self.direction, self.boundary) {
            (_, Inside) => "I",
            (_, Outside) => "O",
            (Start, Loop) => "S",
            (NorthSouth, Loop) => "|",
            (EastWest, Loop) => "-",
            (NorthEast, Loop) => "L",
            (NorthWest, Loop) => "J",
            (SouthWest, Loop) => "7",
            (SouthEast, Loop) => "F",
            (Ground, Unknown) => ".",
            _ => "?"
        })
    }
}

#[derive(Clone, Debug)]
struct Route {
    path: Vec<Node>,
    successful: bool,
}

fn main() {
    let samples = load("c:\\projects\\advent-of-code-2023\\data\\day10.sample.dat");
    let sample2 = load("c:\\projects\\advent-of-code-2023\\data\\day10.sample2.dat");
    let sample3 = load("c:\\projects\\advent-of-code-2023\\data\\day10.sample3.dat");
    let inputs = load("c:\\projects\\advent-of-code-2023\\data\\day10.dat");
    println!("Part 1");
    let sample_map = parse(&samples).expect("Failed to parse the sample map");
    let sample2_map = parse(&sample2).expect("Failed to parse the sample2 map");
    let sample3_map = parse(&sample3).expect("Failed to parse the sample3 map");
    let sample_route = map_cost(&sample_map);
    let sample2_route = map_cost(&sample2_map);
    let sample3_route = map_cost(&sample3_map);
    println!("[SAMPLE]: {}", sample_route.path.iter().map(|s| s.distance).max().unwrap()); // 8
    let input_map = parse(&inputs).expect("Failed to parse the input map");
    let input_route = map_cost(&input_map);
    println!("[INPUT]: {}", input_route.path.iter().map(|s| s.distance).max().unwrap()); // 6882

    println!("Part 2");
    // println!("[SAMPLE 1]: {}", count_contained(&sample_route, &sample_map)); // 4
    println!("[SAMPLE 2]: {}", count_contained(&sample2_route, &sample2_map)); // 8
    println!("[SAMPLE 3]: {}", count_contained(&sample3_route, &sample3_map)); // 10
}


fn parse(lines: &Vec<String>) -> Result<Vec<Vec<Node>>, String> {
    let mut map = Vec::new();

    for (row, l) in lines.iter().enumerate() {
        let mut nodes = Vec::new();
        for (col, c) in l.chars().enumerate() {
            let direction = match c {
                'S' => Start,
                '|' => NorthSouth,
                '-' => EastWest,
                'L' => NorthEast,
                'J' => NorthWest,
                '7' => SouthWest,
                'F' => SouthEast,
                _ => Ground
            };
            nodes.push(Node { row, col, direction, distance: 0, boundary: match direction { Ground => Unknown, _=> Edge} });
        }
        map.push(nodes);
    }

    Ok(map)
}

fn is_contained(row: usize, col: usize, map: &Vec<Vec<Node>>) -> bool {
    let rows = map.len();
    let cols = map[0].len();

    let mut north = false;
    let mut west = false;
    let mut south = false;
    let mut east = false;

    let mut i = row;
    while !north && i > 0 {
        north |= map[i][col].boundary == Loop;
        i -= 1;
    }

    i = col;
    while !west && i > 0 {
        west |= map[row][i].boundary == Loop;
        i -= 1;
    }

    i = row;
    while !south && i < rows {
        south |= map[i][col].boundary == Loop;
        i += 1;
    }

    i = col;
    while !east && i < cols {
        east |= map[row][i].boundary == Loop;
        i += 1;
    }

    north && west && south && east
}

fn pretty_print(map: &Vec<Vec<Node>>) {
    println!("{}",map.iter().map(|r| r.iter().map(|n| format!("{}",n)).collect::<String>()).collect::<Vec<_>>().join("\n"));
}

fn count_contained(route: &Route, map: &Vec<Vec<Node>>) -> u32 {
    let mut annotated = map.clone();
    let rows = annotated.len();
    let cols = annotated[0].len();

    for step in route.path.clone() {
        annotated[step.row][step.col].boundary = Loop;
    }

    let mut inside = 0;
    // let mut contained = false;
    // let mut onboundary = false;

    for r in 0..rows {
        for c in 0..cols {
            match (annotated[r][c].boundary, is_contained(r, c, &annotated)) {
                (Unknown, true) => {
                    annotated[r][c].boundary = Inside;
                    inside += 1;
                }
                (Unknown, false) => annotated[r][c].boundary = Outside,
                _ => ()
            }
            // match annotated[r][c].boundary {
            //     Edge => {
            //         if !onboundary {
            //             contained=true;
            //
            //         } else {
            //             contained = !contained;
            //         }
            //
            //     }
            //     Unknown => {
            //         annotated[r][c].boundary = match contained {
            //             true => Inside,
            //             false => Outside
            //         };
            //     },
            //     _ => ()
            // };
        }
    }

    pretty_print(&annotated);

    inside
}

fn map_cost(map: &Vec<Vec<Node>>) -> Route {
    // let annotated = map.clone();

    let rows = map.len();
    let cols = map[0].len();

    let mut r = 0;
    let mut c = 0;

    let mut start = None;
    while start.is_none() {
        if &map[r][c].direction == &Start {
            start = Some(&map[r][c]);
        }
        c += 1;

        if c % cols == 0 {
            c = 0;
            r += 1;
        }
    }

    match start {
        Some(n) => {
            r = n.row;
            c = n.col;
        }
        _ => { start.expect("Unable to find the starting node"); }
    };

    println!("\tStart at [{},{}]", r, c);

    let mut routes = Vec::new();

    if r > 0 { // look north
        routes.push(map_route(&map[r - 1][c], &start.unwrap(), map));
    }
    if c > 0 { // look west
        routes.push(map_route(&map[r][c - 1], &start.unwrap(), map));
    }
    if r + 1 < rows { // look south
        routes.push(map_route(&map[r + 1][c], &start.unwrap(), map));
    }
    if c + 1 < cols { // look east
        routes.push(map_route(&map[r][c + 1], &start.unwrap(), map));
    }

    let mut route = routes
        .iter()
        .filter(|r| r.successful)
        .map(|r| r.clone())
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .clone();

    let steps = route.path.len();
    let midpoint = steps / 2;
    let mut decrease = midpoint as u32;

    for i in midpoint + 1..steps {
        route.path[i].distance -= decrease;
        decrease += 1;
    }


    route
}

fn determine_orientation(n1: &Node, n2: &Node) -> CardinalDirection {
    if n1.row == n2.row {
        match n1.col > n2.col {
            true => West,
            _ => East
        }
    } else {
        match n1.row > n2.row {
            true => North,
            _ => South
        }
    }
}

fn map_route(next: &Node, origin: &Node, map: &Vec<Vec<Node>>) -> Route {
    let mut path = Vec::new();

    let cols = map[0].len();
    let rows = map.len();
    let mut n1 = origin;
    let mut n2 = next;

    path.push(Node { distance: 0, ..n1.clone() });

    let mut valid = is_valid(n1, n2);
    let mut bearing = determine_orientation(n1, n2);
    let mut distance = 1;

    while valid && n2 != origin {
        path.push(Node { distance, ..n2.clone() });

        let opt = match (bearing, n2.direction) {
            (South, NorthSouth) if n2.row + 1 < rows => Some((South, map[n2.row + 1][n2.col])),
            (South, NorthEast) if n2.col + 1 < cols => Some((East, map[n2.row][n2.col + 1])),
            (South, NorthWest) if n2.col > 0 => Some((West, map[n2.row][n2.col - 1])),
            (North, NorthSouth) if n2.row > 0 => Some((North, map[n2.row - 1][n2.col])),
            (North, SouthEast) if n2.col + 1 < cols => Some((East, map[n2.row][n2.col + 1])),
            (North, SouthWest) if n2.col > 0 => Some((West, map[n2.row][n2.col - 1])),
            (East, EastWest) if n2.col < cols => Some((East, map[n2.row][n2.col + 1])),
            (East, NorthWest) if n2.row > 0 => Some((North, map[n2.row - 1][n2.col])),
            (East, SouthWest) if n2.row < rows => Some((South, map[n2.row + 1][n2.col])),
            (West, EastWest) if n2.col > 0 => Some((West, map[n2.row][n2.col - 1])),
            (West, NorthEast) if n2.row > 0 => Some((North, map[n2.row - 1][n2.col])),
            (West, SouthEast) if n2.row < rows => Some((South, map[n2.row + 1][n2.col])),
            _ => None
        };

        n1 = n2;
        match opt {
            Some((exit, n)) => {
                bearing = exit;
                n2 = &map[n.row][n.col];
                valid = is_valid(n1, n2);
            }
            _ => ()
        };

        distance += 1;
    }

    Route { path, successful: n2.direction == Start }
}

fn is_valid(n1: &Node, n2: &Node) -> bool {
    match determine_orientation(n1, n2) {
        North => match n2.direction {
            NorthSouth | SouthWest | SouthEast => true,
            _ => false
        },
        South => match n2.direction {
            NorthSouth | NorthWest | NorthEast => true,
            _ => false
        },
        East => match n2.direction {
            EastWest | NorthWest | SouthWest => true,
            _ => false
        },
        West => match n2.direction {
            EastWest | NorthEast | SouthEast => true,
            _ => false
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