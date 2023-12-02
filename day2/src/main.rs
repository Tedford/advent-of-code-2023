use std::fs;

struct Pull {
    red: i32,
    green: i32,
    blue: i32,
}

struct Game {
    id: i32,
    pulls: Vec<Pull>,
}

fn main() {
    let sample = load("c:\\projects\\advent-of-code-2023\\data\\day2.sample.dat");
    let input = load("c:\\projects\\advent-of-code-2023\\data\\day2.dat");

    let rubric = Pull { red: 12, green: 13, blue: 14 };

    let sample_games = sample.iter().map(|l| parse_game(l)).collect::<Vec<Game>>();
    let input_games = input.iter().map(|l| parse_game(l)).collect::<Vec<Game>>();

    let impossible_samples:&Vec<&Game> = &sample_games.iter().filter(|g| is_possible(g, &rubric)).collect();
    let impossible_games :&Vec<&Game>= &input_games.iter().filter(|g| is_possible(g, &rubric)).collect();

    println!("Sample: {}", get_count(impossible_samples));
    println!("Input: {}", get_count(impossible_games));

    println!("Power");
    println!("Sample: {}", sample_games.iter().map(|g|calculate_power(g)).sum::<i32>());
    println!("Input: {}", input_games.iter().map(|g|calculate_power(g)).sum::<i32>());
}


fn load(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("Unable to load the data file")
        .lines()
        .map(|l| l.to_string())
        .collect()
}

fn calculate_power(game: &Game) -> i32{
    let red = game.pulls.iter().map(|p|p.red).max().unwrap();
    let blue = game.pulls.iter().map(|p|p.blue).max().unwrap();
    let green = game.pulls.iter().map(|p|p.green).max().unwrap();
    red * blue * green
}

fn get_count(games: &Vec<&Game>) -> i32 {
    games.iter().map(|g| g.id).sum::<i32>()
}

fn is_possible(game: &Game, rubric: &Pull) -> bool {
    !is_impossible(game, rubric)
}

fn is_impossible(game: &Game, rubric: &Pull) -> bool {
    game.pulls.iter().map(|p| p.red).any(|r| r > rubric.red) ||
        game.pulls.iter().map(|p| p.blue).any(|r| r > rubric.blue) ||
        game.pulls.iter().map(|p| p.green).any(|r| r > rubric.green)
}

fn parse_game(game: &String) -> Game {
    let end_header = game.chars().position(|c| c == ':').unwrap();
    let id = game[5..end_header].parse::<i32>().unwrap();
    let pulls = game[end_header + 1..].split(';').map(|s| {
        let mut red: i32 = 0;
        let mut green: i32 = 0;
        let mut blue: i32 = 0;

        for part in s.split(',') {
            let trimmed = part.trim();
            let space = trimmed.chars().position(|c| c == ' ').unwrap();
            let value = trimmed[..space].parse::<i32>().unwrap();

            match &trimmed[space + 1..] {
                "green" => green += value,
                "blue" => blue += value,
                "red" => red += value,
                _ => ()
            }
        }

        Pull { red, green, blue }
    }
    ).collect();

    Game { id, pulls }
}