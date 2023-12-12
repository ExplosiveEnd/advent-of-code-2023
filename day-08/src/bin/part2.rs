use std::collections::HashMap;
use num::integer::lcm;

fn part2(input: &str) -> usize {
    let mut lines: Vec<String> = input.lines()
        .map(|l| l.to_string())
        .filter(|l| !l.is_empty())
        .collect();
    
    let instructions: String = lines.drain(..1).collect();

    let mut hash: HashMap<String, (String, String)> = HashMap::new();

    for line in lines {
        let split: (&str, &str) = line.split_once('=').unwrap();

        let left: String = split.0.trim().to_string();

        let right = split.1.split_once(' ').unwrap().1.replace(')', " ").replace('(', " ");

        let nodes = right.split_once(',').unwrap();

        let pairs: (String, String) = (nodes.0.trim().to_string(), nodes.1.trim().to_string());

        hash.insert(left, pairs);
    }

    let mut steps: usize;
    let current: Vec<String> = hash.clone().into_iter()
        .filter(|(k, _)| k.contains("A"))
        .map(|(k, _)| k)
        .collect();
    let mut locations: Vec<usize> = Vec::new();

    for item in current {
        steps = 0;
        let mut temp = item;
        while !temp.contains("Z") {
            for letter in instructions.chars() {
                let (left, right) = hash.get(&temp).unwrap();
                
                if letter == 'L' {
                    temp = left.to_string();
                }
                else {
                    temp = right.to_string();
                }
                
                steps += 1;
                if temp.contains("Z") {
                    break;
                }
            }
            
        }
        locations.push(steps);
    }

    // Finds LCM
    let first = locations.iter().fold(1, |acc, ele| lcm(acc, *ele));

    first
}

fn main() {
    let input = include_str!("input.txt");

    let output = part2(input);
    println!("Output: {output}");
}
