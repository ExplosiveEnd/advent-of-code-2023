use std::collections::HashMap;

fn a_to_b(lines: &mut Vec<String>, needle: &str) -> HashMap<(usize, usize), usize> {
    let mut hash: HashMap<(usize, usize), usize> = HashMap::new();
    let mut index = 0;

    if !lines.is_empty() {
        lines.remove(0);
    }

    while lines.get(index).is_some() && lines.get(index).unwrap() != needle {
        let split: Vec<usize> = lines.get(index).unwrap()
            .split(' ')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        hash.insert((split[1], split[2]-1), split[0]);
        index += 1;
    }

    let _ = lines.drain(..index).collect::<Vec<_>>();

    hash
}

fn part1(input: &str) -> usize {
    let mut lines: Vec<String> = input.lines()
        .map(|l| l.to_string())
        .filter(|l| !l.is_empty())
        .collect();

    let seeds_str: String = lines.drain(..1).collect();
    let seeds: Vec<usize> = seeds_str.split_once(':').unwrap().1
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut result: Vec<usize> = Vec::with_capacity(seeds.len());
    
    // Chain of map creations
    let to_soil = a_to_b(&mut lines, "soil-to-fertilizer map:");
    let to_fert = a_to_b(&mut lines, "fertilizer-to-water map:");
    let to_water = a_to_b(&mut lines, "water-to-light map:");
    let to_light = a_to_b(&mut lines, "light-to-temperature map:");
    let to_temperature = a_to_b(&mut lines, "temperature-to-humidity map:");
    let to_humidity = a_to_b(&mut lines, "humidity-to-location map:");
    let to_location = a_to_b(&mut lines, "");
    

    for seed in &seeds {
        // Chain of conversions
        let soil = conversion(*seed, &to_soil);
        let fert = conversion(soil, &to_fert);
        let water = conversion(fert, &to_water);
        let light = conversion(water, &to_light);
        let temperature = conversion(light, &to_temperature);
        let humidity = conversion(temperature, &to_humidity);
        let location = conversion(humidity, &to_location);

        result.push(location);
    }
    
    *result.iter().min().unwrap()
}

fn conversion(seed: usize, map: &HashMap<(usize, usize), usize>) -> usize {
    let mut soil = 0;
    let mut found = false;

    for (key, value) in map.iter() {
        if seed >= key.0 && seed <= key.0+key.1 {
            soil = value+seed-key.0;
            found = true;
        }
    }

    if !found {
        soil = seed;
    }
    soil
}

fn main() {
    let input = include_str!("input.txt");

    let output = part1(&input);
    println!("Output: {output}");
}
