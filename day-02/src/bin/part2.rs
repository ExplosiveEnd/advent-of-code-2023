fn part2(input: &str) -> u32{
    let mut values: Vec<u32> = vec![];

    for line in input.lines() {
        let mut maximum = (0, 0, 0);
        // Removes "Game x:"
        let content = line.split(':').collect::<Vec<_>>()[1];

        // Splits into 2D vector of [value, color] 
        let content_list = content.split([',',';']).map(|x| x.strip_prefix(' ').unwrap().split(' ').collect::<Vec<_>>()).collect::<Vec<_>>();

        for pair in content_list {
            let value: u32 = pair[0].parse::<u32>().unwrap();
            match pair[1] {
                "red" => {
                    if value > maximum.0 {
                        maximum.0 = value;
                    }
                },
                "green" => {
                    if value > maximum.1 {
                        maximum.1 = value;
                    }
                },
                "blue" => {
                    if value > maximum.2 {
                        maximum.2 = value;
                    }
                }
                _ => unreachable!()
            }
        }
        values.push(maximum.0*maximum.1*maximum.2);
    }

    values.iter().sum()
}

fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    println!("Output: {output}");
}
