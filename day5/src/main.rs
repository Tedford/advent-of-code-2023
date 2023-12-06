use std::fs;
use std::ops::Range;

struct SeedLocation {
    seed: i64,
    location: i64
}

struct GardenContext {
    seeds: Vec<i64>,
    seed_to_soil: Concept,
    soil_to_fertilizer: Concept,
    fertilizer_to_water: Concept,
    water_to_light: Concept,
    light_to_temperature: Concept,
    temperature_to_humidity: Concept,
    humidity_to_location: Concept
}

struct Concept {
    name: String,
    maps: Vec<ConceptMap>
}

impl Concept {
    pub fn translate(&mut self, value: i64) -> i64 {
        let mut translate = None;
        for map in &mut self.maps {
            match map.translate(value) {
                Some(t)=> translate = Some(t),
                _=> ()
            }
        }

        if translate.is_none() {
            value
        } else {
            translate.unwrap()
        }
    }
}

struct ConceptMap {
    source: Range<i64>,
    destination: Range<i64>
}

impl ConceptMap {
    pub fn translate(&mut self, value: i64) -> Option<i64>{
        let source  = self.source.clone();
        let start = source.start;
        let end = source.end;
        if start <= value && end > value {
            let destination = self.destination.clone();
            Some(destination.start + value - start)
        }
        else {
            None
        }
        // match self.source.clone().position(|i|i == value) {
        //     Some(p) => self.destination.clone().nth(p),
        //     _ => None
        // }
    }
}

fn main() {
    let samples = load("c:\\projects\\advent-of-code-2023\\data\\day5.sample.dat");
    let inputs = load("c:\\projects\\advent-of-code-2023\\data\\day5.dat");

    println!("Part 1");

    let mut sample_garden = parse_context(&samples).expect("Unable to generate the sample garden");
    let mut sample_locations = get_seed_locations(&mut sample_garden);
    sample_locations.sort_by(|a,b|a.location.cmp(&b.location));
    println!("[SAMPLE]: {}",sample_locations[0].location); // 35

    let mut input_garden = parse_context(&inputs).expect("Unable to generate the sample garden");
    let mut input_locations = get_seed_locations(&mut input_garden);
    input_locations.sort_by(|a,b|a.location.cmp(&b.location));
    println!("[INPUT]: {}",input_locations[0].location); // 382895070

    println!("Part 2");
    let location = find_lowest_location(&mut sample_garden).expect("Unable to generate the sample garden");
    println!("[SAMPLE]: {}",location); // 46

    let location = find_lowest_location(&mut input_garden).expect("Unable to generate the sample garden");
    println!("[INPUT]: {}",location);
}

fn get_seed_location(seed: i64, context: &mut GardenContext) -> i64 {
    let soil = context.seed_to_soil.translate(seed);
    let fertilizer = context.soil_to_fertilizer.translate(soil);
    let water= context.fertilizer_to_water.translate(fertilizer);
    let light = context.water_to_light.translate(water);
    let temperature = context.light_to_temperature.translate(light);
    let humidity = context.temperature_to_humidity.translate(temperature);
    let location = context.humidity_to_location.translate(humidity);
    location
}

fn get_seed_locations(context: &mut GardenContext) -> Vec<SeedLocation>{
    let mut assignments = Vec::new();

    for seed in context.seeds.clone() {
        let location = get_seed_location(seed, context);
        assignments.push(SeedLocation{seed,location});
    }

    assignments
}

fn parse_concept_map(line: &String) -> Result<ConceptMap, String> {
    let parts = line.split(|c| c ==' ').collect::<Vec<&str>>();
    let destination = parts.iter().nth(0).expect(&*format!("Unable to access the destination value {line}")).parse::<i64>().expect("Unable to parse the destination value");
    let source = parts.iter().nth(1).expect(&*format!("Unable to access the source value {line}")).parse::<i64>().expect("Unable to parse the source value");
    let range = parts.iter().nth(2).expect(&*format!("Unable to access the range value {line}")).parse::<i64>().expect("Unable to parse the range value");
    Ok(ConceptMap { source: source..source+range, destination : destination..destination+range})
}

fn find_lowest_location(context: &mut GardenContext) -> Result<i64, String>{
    let mut location = i64::MAX;
    let seeds_ranges = context.seeds.clone();
    for i in (0..seeds_ranges.len()).step_by(2) {
        for seed in seeds_ranges[i]..seeds_ranges[i] + seeds_ranges[i+1]{
            let l2 = get_seed_location(seed, context);
            location = i64::min(location, l2);
        }
    }

    Ok(location)
}

fn parse_context (lines: &Vec<String>) -> Result<GardenContext,String> {
    let mut seeds = Vec::new();
    let mut seed_to_soil_map= Vec::new();
    let mut soil_to_fertilizer_map= Vec::new();
    let mut fertilizer_to_water_map= Vec::new();
    let mut water_to_light_map= Vec::new();
    let mut light_to_temperature_map= Vec::new();
    let mut temperature_to_humidity_map= Vec::new();
    let mut humidity_to_location_map = Vec::new();


    let mut map_name= "";
    for l in lines.iter() {
        if l.starts_with("seeds"){
            let mut buffer = Vec::new();

            for c in l[7..].chars() {
                if c.is_digit(10){
                    buffer.push(c);
                }
                else {
                    if buffer.len() > 0 {
                        seeds.push(buffer.iter().collect::<String>().parse::<i64>().expect(&*format!("Failed to parse {l}")));
                        buffer.clear();
                    }
                }
            }
            if buffer.len() > 0 {
                seeds.push(buffer.iter().collect::<String>().parse().expect(&*format!("Failed to parse {l}")));
            }
        }
        else if l.ends_with("map:"){
            map_name = &l[0..l.len()-5];
        }
        else if !l.is_empty(){
            let map = parse_concept_map(l)?;
            match  map_name {
                "seed-to-soil" => seed_to_soil_map.push(map),
                "soil-to-fertilizer" => soil_to_fertilizer_map.push(map),
                "fertilizer-to-water" => fertilizer_to_water_map.push(map),
                "water-to-light"=> water_to_light_map.push(map),
                "light-to-temperature" =>light_to_temperature_map.push(map),
                "temperature-to-humidity" => temperature_to_humidity_map.push(map),
                "humidity-to-location"=> humidity_to_location_map.push(map),
                _ => ()
            }
        }
    }

    let seed_to_soil = Concept{name: "seed-to-soil".to_string(), maps:seed_to_soil_map };
    let soil_to_fertilizer = Concept{name: "soil-to-fertilizer".to_string(), maps:soil_to_fertilizer_map };
    let fertilizer_to_water = Concept{name: "fertilizer-to-water".to_string(), maps:fertilizer_to_water_map };
    let water_to_light = Concept{name: "water-to-light".to_string(), maps:water_to_light_map };
    let light_to_temperature = Concept{name: "light-to-temperature".to_string(), maps:light_to_temperature_map };
    let temperature_to_humidity = Concept{name: "temperature-to-humidity".to_string(), maps:temperature_to_humidity_map };
    let humidity_to_location = Concept{name: "humidity-to-location".to_string(), maps:humidity_to_location_map };


    Ok(GardenContext{seeds, seed_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location })
}


fn load(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("Unable to load the data file")
        .lines()
        .map(|l| l.to_string())
        .collect()
}
