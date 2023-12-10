use std::iter::zip;

fn part1(input: &str) -> u32 {
    let mut values: Vec<u32> = Vec::new();

    let lines: Vec<String> = input.lines()
        .map(|l| l.to_string())
        .collect();

    let time: u64 = lines.get(0).unwrap()
        .split_once(':').unwrap().1
        .split(' ')
        .filter(|v| !v.is_empty())
        .collect::<String>()
        .parse().unwrap();

    println!("Time: {time}");

    let distance: u64 = lines.get(1).unwrap()
        .split_once(':').unwrap().1
        .split(' ')
        .filter(|v| !v.is_empty())
        .collect::<String>()
        .parse().unwrap();

    println!("Distance: {distance}");

    let mut count = 0;
    for speed in 0..=time {
        let remaining = time-speed;
        let mut traveled = 0;
        for _ in 0..remaining {
            traveled += speed;
        }

        if traveled > distance {
            count += 1;
        }
    }

    values.push(count);

    values.iter().fold(1, |acc, ele| acc * ele)
}

fn main() {
    let input = include_str!("input.txt");

    let output = part1(input);
    println!("Output: {output}");
}
