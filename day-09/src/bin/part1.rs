#![feature(array_windows)]
fn solution(input: &str) -> i32 {
    let lines: Vec<String> = input.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect();

    let mut ends: Vec<i32> = Vec::new();

    for line in lines {
        let values: Vec<i32> = line.split_whitespace()
            .map(|l| l.parse::<i32>().unwrap())
            .collect();

        // Holds all "layers" of main vector
        let mut histories: Vec<Vec<_>> = vec![values.clone()];

        // Array_windows are NIGHTLY and default to size 2
        while !histories.last().unwrap().iter().all(|&x| x == 0) {
            histories.push(histories.last().unwrap().array_windows().map(|[x, y]| y - x).collect::<Vec<_>>());
        }

        // Appends proper value to end of each vector
        for index in (0..histories.len()-1).rev() {
            let temp = histories.clone();
            histories[index].push(*temp[index].last().unwrap() + *temp[index+1].last().unwrap());
        }

        ends.push(*histories[0].last().unwrap());
    }

    ends.iter().sum()
}

fn main() {
    let input = include_str!("input.txt");

    let output = solution(input);
    println!("Output: {output}");
}
