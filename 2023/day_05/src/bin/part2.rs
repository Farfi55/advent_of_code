use std::borrow::BorrowMut;
use std::sync::Mutex;
use rayon::prelude::*;
use linya::Progress;


fn main() {
    println!("include_str!... ");
    let input = include_str!("./input.txt");
    let result = part2(input);
    println!("RESULT:\n{}", result);
}

#[derive(Debug)]
struct Map(Vec<(usize, usize, usize)>);

#[derive(Debug)]
struct AlmanacMaps {
    seeds: Vec<usize>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}


impl AlmanacMaps {

    fn maps(&self) -> [&Map; 7] {
        [
            &self.seed_to_soil,
            &self.soil_to_fertilizer,
            &self.fertilizer_to_water,
            &self.water_to_light,
            &self.light_to_temperature,
            &self.temperature_to_humidity,
            &self.humidity_to_location,
        ]
    }

    #[allow(dead_code)]
    fn get_map(&self, name: &str) -> &Map {
        match name {
            "seed-to-soil" => &self.seed_to_soil,
            "soil-to-fertilizer" => &self.soil_to_fertilizer,
            "fertilizer-to-water" => &self.fertilizer_to_water,
            "water-to-light" => &self.water_to_light,
            "light-to-temperature" => &self.light_to_temperature,
            "temperature-to-humidity" => &self.temperature_to_humidity,
            "humidity-to-location" => &self.humidity_to_location,
            _ => panic!("Unknown map name: {}", name),
        }
    }

    fn seed_to_location(&self, seed: usize) -> usize {
        let mut location = seed;
        for map in self.maps().iter() {
            location = map.get(location);
        }
        location
        // alternative:
        // return self.maps().iter().fold(seed, |location, map| map.get(location));
    }

    fn new() -> AlmanacMaps {
        AlmanacMaps {
            seeds: Vec::new(),
            seed_to_soil: Map::new(),
            soil_to_fertilizer: Map::new(),
            fertilizer_to_water: Map::new(),
            water_to_light: Map::new(),
            light_to_temperature: Map::new(),
            temperature_to_humidity: Map::new(),
            humidity_to_location: Map::new(),
        }
    }

    fn set_map(&mut self, name: &str, map: Map) {
        match name {
            "seed-to-soil" => self.seed_to_soil = map,
            "soil-to-fertilizer" => self.soil_to_fertilizer = map,
            "fertilizer-to-water" => self.fertilizer_to_water = map,
            "water-to-light" => self.water_to_light = map,
            "light-to-temperature" => self.light_to_temperature = map,
            "temperature-to-humidity" => self.temperature_to_humidity = map,
            "humidity-to-location" => self.humidity_to_location = map,
            _ => panic!("Unknown map name: {}", name),
        }
    }

}

impl Map {
    fn new() -> Map {
        Map(Vec::new())
    }

    fn get(&self, key: usize) -> usize {
        for (src, dest, range_lenght) in self.0.iter() {
            
            if key >= *src && key < *src + *range_lenght {
                return dest + key - src;
            }
        }
        key
    }

    
    
    fn insert_range(&mut self, src: usize, dest: usize, range_lenght: usize) {
        self.0.push((src, dest, range_lenght))
    }
}


impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for AlmanacMaps {
    fn eq(&self, other: &Self) -> bool {
        self.seeds == other.seeds
        && self.seed_to_soil == other.seed_to_soil
        && self.soil_to_fertilizer == other.soil_to_fertilizer
        && self.fertilizer_to_water == other.fertilizer_to_water
        && self.water_to_light == other.water_to_light
        && self.light_to_temperature == other.light_to_temperature
        && self.temperature_to_humidity == other.temperature_to_humidity
        && self.humidity_to_location == other.humidity_to_location
    }
}

fn parse_almanac(input: &str) -> AlmanacMaps {
    let mut almanac = AlmanacMaps::new(); 

    let mut lines = input.lines();
    let seeds_line = lines.next().expect("there should be a seeds line");
    let seeds_raw = seeds_line.split_whitespace().skip(1);
    let seeds_nums: Vec::<usize> = seeds_raw.map(|seed| seed.parse().expect("should be a number")).collect();

    let seeds_range: Vec<(usize, usize)> = seeds_nums.chunks(2).map(|chunk| (chunk[0], chunk[0] + chunk[1])).collect();
    for (seed_start, seed_end) in seeds_range {
        for seed in seed_start..seed_end {
            almanac.seeds.push(seed);
        }
    }

    
    let mut curr_map_name: Option<&str> = None;
    let mut curr_map: Option<Map> = None;
    for line in lines {
        if line.is_empty() {
            if curr_map_name.is_some() && curr_map.is_some() {
                almanac.set_map(curr_map_name.unwrap(), curr_map.unwrap());
            }
            curr_map_name = None;
            curr_map = None;
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.last() == Some(&"map:") {
            curr_map_name = Some(parts.first().expect("there should be a map name"));
            curr_map = Some(Map::new());
        }
        else if let Some(curr_map) = curr_map.borrow_mut() {
            if parts.len() != 3 {
                panic!("should be 3 parts");
            }
            let dest: usize = parts[0].parse().expect("should be a number");
            let src: usize = parts[1].parse().expect("should be a number");
            let range: usize = parts[2].parse().expect("should be a number");

            curr_map.insert_range(src, dest, range);
        }
    }

    if curr_map_name.is_some() && curr_map.is_some() {
        almanac.set_map(curr_map_name.unwrap(), curr_map.unwrap());
    }

    almanac
}


#[allow(dead_code)]
fn example_almanac() -> AlmanacMaps {
    let mut almanac = AlmanacMaps::new();
    
    for (seed_start, lenght) in [(79, 14), (55, 13)] {
        for seed in seed_start..seed_start + lenght {
            almanac.seeds.push(seed);
        }
    }

    almanac.seed_to_soil.insert_range(98, 50, 2);
    almanac.seed_to_soil.insert_range(50, 52, 48);

    almanac.soil_to_fertilizer.insert_range(15, 0, 37);
    almanac.soil_to_fertilizer.insert_range(52, 37, 2);
    almanac.soil_to_fertilizer.insert_range(0, 39, 15);

    almanac.fertilizer_to_water.insert_range(53, 49, 8);
    almanac.fertilizer_to_water.insert_range(11, 0, 42);
    almanac.fertilizer_to_water.insert_range(0, 42, 7);
    almanac.fertilizer_to_water.insert_range(7, 57, 4);

    almanac.water_to_light.insert_range(18, 88, 7);
    almanac.water_to_light.insert_range(25, 18, 70);

    almanac.light_to_temperature.insert_range(77, 45, 23);
    almanac.light_to_temperature.insert_range(45, 81, 19);
    almanac.light_to_temperature.insert_range(64, 68, 13);

    almanac.temperature_to_humidity.insert_range(69, 0, 1);
    almanac.temperature_to_humidity.insert_range(0, 1, 69);

    almanac.humidity_to_location.insert_range(56, 60, 37);
    almanac.humidity_to_location.insert_range(93, 56, 4);
    almanac
}


fn part2(input: &str) -> usize {
    println!("Parsing input... ");
    let almanac = parse_almanac(input);
    println!("Parsing done");

    let total = almanac.seeds.len();
    let steps = total / 1000;
    let progress = Mutex::new(Progress::new());
    let bar = progress.lock().unwrap().bar(total, "finding best seed");

    let result = almanac.seeds.par_iter().enumerate().map(|(i, seed)| {
        let location = almanac.seed_to_location(*seed);

        if i % steps == 0 {
            progress.lock().unwrap().inc_and_draw(&bar, steps);
        }
        location
    })
    .min().expect("there should be a location for each seed");
    
    progress.lock().unwrap().set_and_draw(&bar, total);
    return result;

}

#[cfg(test)]
mod tests {


    #[test]
    fn it_works() {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

        let result = super::part2(input);
        assert_eq!(result, 46);
    }


    #[test]
    fn parsing_almanac() {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

        let parsed_almanac = super::parse_almanac(input);
        let expected_almanac = super::example_almanac();
        
        assert_eq!(parsed_almanac.seeds, expected_almanac.seeds, "seeds");
        assert_eq!(parsed_almanac.seed_to_soil, expected_almanac.seed_to_soil, "seed_to_soil");
        assert_eq!(parsed_almanac.soil_to_fertilizer, expected_almanac.soil_to_fertilizer, "soil_to_fertilizer");
        assert_eq!(parsed_almanac.fertilizer_to_water, expected_almanac.fertilizer_to_water, "fertilizer_to_water"); 
        assert_eq!(parsed_almanac.water_to_light, expected_almanac.water_to_light, "water_to_light"); 
        assert_eq!(parsed_almanac.light_to_temperature, expected_almanac.light_to_temperature, "light_to_temperature"); 
        assert_eq!(parsed_almanac.temperature_to_humidity, expected_almanac.temperature_to_humidity, "temperature_to_humidity"); 
        assert_eq!(parsed_almanac.humidity_to_location, expected_almanac.humidity_to_location, "humidity_to_location"); 


    }
        
}
