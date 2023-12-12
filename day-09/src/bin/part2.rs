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

        let mut histories: Vec<Vec<_>> = vec![values.clone()];

        // Array_windows are NIGHTLY and default to size 2
        while !histories.last().unwrap().iter().all(|&x| x == 0) {
            histories.push(histories.last().unwrap().array_windows().map(|[x, y]| y - x).collect::<Vec<_>>());
        }

        // Inserts values into beginning of each subvector
        for index in (0..histories.len()-1).rev() {
            let temp = histories.clone();
            histories[index].insert(0, *temp[index].first().unwrap() - (*temp[index+1].first().unwrap()));
        }

        ends.push(*histories[0].first().unwrap());
    }

    ends.iter().sum()
}

fn main() {
    let input = include_str!("input.txt");

    let output = solution(input);
    println!("Output: {output}");
}
