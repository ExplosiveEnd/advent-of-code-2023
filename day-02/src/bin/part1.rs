fn part1(input: &str) -> u32{
    let mut values: Vec<u32> = vec![];
    // Red, Green, Blue
    let limits = (12, 13, 14);

    for (mut index, line) in input.lines().enumerate() {
        index += 1;
        let mut possible: bool = true;

        // Removes "Game x:"
        let content = line.split(':').collect::<Vec<_>>()[1];

        // Splits into 2D vector of [value, color] 
        let content_list = content.split([',',';']).map(|x| x.strip_prefix(' ').unwrap().split(' ').collect::<Vec<_>>()).collect::<Vec<_>>();

        for pair in content_list {
            let value: u32 = pair[0].parse::<u32>().unwrap();
            match pair[1] {
                "red" => {
                    if value > limits.0 {
                        possible = false;
                    }
                },
                "green" => {
                    if value > limits.1 {
                        possible = false;
                    }
                },
                "blue" => {
                    if value > limits.2 {
                        possible = false;
                    }
                }
                _ => unreachable!()
            }
        }
        if possible {
            values.push(index as u32);
        }
    }

    values.iter().sum()
}

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("Output: {output}");
}
