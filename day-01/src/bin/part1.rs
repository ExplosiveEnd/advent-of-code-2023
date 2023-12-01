fn part1(input: &str) -> u32 {
    let mut values: Vec<u32> = vec![];

    for line in input.lines() {
        let nums = line.chars().filter(|x| x.is_digit(10)).collect::<Vec<_>>();
        let agg = format!("{}{}", nums.first().unwrap(), nums.last().unwrap());
        values.push(agg.parse::<u32>().unwrap());
    }

    values.iter().sum()
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input); 
    println!("{}", output);
}
