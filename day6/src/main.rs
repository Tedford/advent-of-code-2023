use std::fs;

struct RaceMetric {
    time: i64,
    distance: i64,
}

fn main() {
    let samples = load("c:\\projects\\advent-of-code-2023\\data\\day6.sample.dat");
    let inputs = load("c:\\projects\\advent-of-code-2023\\data\\day6.dat");

    println!("Part 1");
    let sample_metrics = parse_race_metrics(&samples).expect("Unable to parse sample metrics");
    let sample_wins: Vec<_> = sample_metrics.iter().map(|m|calculate_wins(m)).collect();
    println!("[SAMPLE]: {}", sample_wins.iter().map(|w|w.len() as i64).product::<i64>()); // 288

    let input_metrics = parse_race_metrics(&inputs).expect("Unable to parse input metrics");
    let input_wins: Vec<_> = input_metrics.iter().map(|m|calculate_wins(m)).collect();
    println!("[INPUT]: {}", input_wins.iter().map(|w|w.len() as i64).product::<i64>());

    println!("Part 2");
    let sample_kerning_wins = calculate_wins(&parse_kerning_metrics(&samples).expect("Unable to parse sample kerning metrics"));
    println!("[SAMPLE]: {}",sample_kerning_wins.len()); // 71503
    let input_kerning_wins = calculate_wins(&parse_kerning_metrics(&inputs).expect("Unable to parse input kerning metrics"));
    println!("[INPUT]: {}",input_kerning_wins.len()); // 71503
}

fn calculate_wins(race_metric: &RaceMetric) -> Vec<RaceMetric> {
    let mut wins = Vec::new();
    for t in 1..race_metric.time -1 {
        let distance = calculate_distance(t, race_metric.time);
        if distance > race_metric.distance {
            wins.push(RaceMetric{time: t, distance});
        }
    }
    wins
}


fn calculate_distance(hold_time: i64, race_time: i64) -> i64 {
    let travel_time = race_time - hold_time;
    let distance  = travel_time * hold_time;
    distance
}


fn parse_numbers(s: String) -> Result<Vec<i64>, String> {
    let mut numbers = Vec::new();
    let mut buffer = Vec::new();
    for c in s.chars() {
        if c.is_digit(10) {
            buffer.push(c);
        }
        else if !buffer.is_empty(){
            numbers.push(buffer.iter().collect::<String>().parse::<i64>().expect(&*format!("Failed to parse {s}")));
            buffer.clear();
        }
    }
    if !buffer.is_empty(){
        numbers.push(buffer.iter().collect::<String>().parse::<i64>().expect(&*format!("Failed to parse {s}")));
    }

    Ok(numbers)
}

fn parse_race_metrics(data: &Vec<String>) -> Result<Vec<RaceMetric>, String> {
    if data.len() != 2 {
        return Err(format!("Expected 2 lines, instead detected {}", data.len()));
    }

    let times = parse_numbers(data[0][5..].to_string())?;
    let distances = parse_numbers(data[1][9..].to_string())?;



    let metrics = times.iter()
        .zip(distances.iter())
        .map(|(time, distance)| RaceMetric { time: *time, distance: *distance })
        .collect();


    Ok(metrics)
}

fn parse_kerning_metrics(data: &Vec<String>) -> Result<RaceMetric,String>{
    if data.len() != 2 {
        return Err(format!("Expected 2 lines, instead detected {}", data.len()));
    }

    let time = parse_numbers(data[0][5..].to_string().replace(" ", ""))?[0];
    let distance = parse_numbers(data[1][9..].to_string().replace(" ", ""))?[0];

    Ok(RaceMetric{time,distance})
}


fn load(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("Unable to load the data file")
        .lines()
        .map(|l| l.to_string())
        .collect()
}
