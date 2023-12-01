fn part2(input: &str) -> u32 {
    let mut values: Vec<u32> = vec![];

    for line in input.lines() {
        // Contains first letter + digit + last letter for overlapping cases.
        let new_line = line.replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");
        let nums = new_line.chars().filter(|x| x.is_digit(10)).collect::<Vec<_>>();
        let agg = format!("{}{}", nums.first().unwrap(), nums.last().unwrap());
        values.push(agg.parse().unwrap());

    }

    values.iter().sum()
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input); 
    println!("{}", output);
}
