use std::collections::HashMap;

fn part2(input: &str) -> u32 {
    let lines: Vec<String> = input.lines()
        .map(|l| l.to_string())
        .collect();
    let mut hash: HashMap<String, u32> = HashMap::new();
    
    for (index, line) in lines.iter().enumerate() {
        let mut count = 0;
        let card_name = line.split_once(": ").unwrap().0.trim();
        
        // Adds value to hash if doesn't exist
        match hash.get(card_name) {
            Some(_) => {},
            None => {
                hash.insert(card_name.to_string(), 1);
                ()
            },
        }
        
        let winning: Vec<&str> = line.split_once(':').unwrap().1
            .split_once('|').unwrap().0
            .split(' ')
            .filter(|l| !l.is_empty())
            .collect();
        let hand: Vec<&str> = line.split_once(':').unwrap().1
            .split_once('|').unwrap().1
            .split(' ')
            .filter(|l| !l.is_empty())
            .collect();

        for number in &hand {
            if winning.contains(&number) {
                count += 1;
            }
        }

        for _repeat in (0..*hash.get(card_name).unwrap()) {
            for next_line_i in index..index+count {
                if let Some(value) = lines.get(next_line_i+1) {
                    let next_card_name = value.split_once(": ").unwrap().0.trim();
                    
                    match hash.get(next_card_name) {
                        Some(x) => hash.insert(next_card_name.to_string(), x + 1),
                        None => hash.insert(next_card_name.to_string(), 2)
                    };
                }
            }
        }
    }

    let values: Vec<_> = hash.values()
        .map(|x| *x)
        .collect();
    
    values.iter().sum()
}

fn main() {
    let input = include_str!("input.txt");

    let output = part2(&input);
    println!("Output: {output}");
}
