fn part1(input: &str) -> u32 {
    let lines = input.lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let mut values: Vec<u32> = Vec::new();
    

    for line in lines {
        let mut count = 0;
        let mut score = 0;
        
        let card = line.split_once('|').unwrap();
        let winning = card.0.split_once(':').unwrap().1.split(' ').filter(|l| !l.is_empty()).collect::<Vec<_>>();
        let hand = card.1.split(' ')
            .filter(|l| !l.is_empty())
            .collect::<Vec<_>>();
        
        for number in hand {
            if winning.contains(&number) {
                match count {
                    0 => score += 1,
                    _ => score *= 2
                }
                count += 1;
            }
        }
        
        values.push(score)
    }
    
    values.iter().sum()
}

fn main() {
    let input = include_str!("input.txt");

    let output = part1(&input);
    println!("{}", output);
}
