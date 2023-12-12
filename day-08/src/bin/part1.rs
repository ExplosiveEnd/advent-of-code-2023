use std::collections::HashMap;

fn part1(input: &str) -> usize {
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

    let mut steps = 0;

    let mut current: String = String::from("AAA");

    while current != String::from("ZZZ") {
        for letter in instructions.chars() {
            let (left, right) = hash.get(&current).unwrap();
            
            if letter == 'L' {
                current = left.to_string();
            } 
            else if letter == 'R' {
                current = right.to_string();
            }
            
            steps += 1;
        }
    }
    
    steps
}

fn main() {
    let input = include_str!("input.txt");

    let output = part1(input);
    println!("Output: {output}");
}
