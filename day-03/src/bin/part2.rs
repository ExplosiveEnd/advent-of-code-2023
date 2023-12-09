fn parse_nums(input: &String) -> Vec<(usize, String)> {
    let mut hold: Vec<(usize, String)> = Vec::new();
    let mut temp = String::new();
    let mut current: (usize, String) = (0, String::new());
    let mut check: bool = false;
    for (i, c) in input.chars().enumerate(){
        if c.to_digit(10).is_some() && !check {
            temp.push(c);
            current.0 = i;
            check = true;
        } 
        else if c.to_digit(10).is_some() && check {
            temp.push(c);
        }
        else {
            if temp.len() > 0 {
                current.1 = temp;
                hold.push(current.clone());
                temp = String::new();
                check = false;
            }
        }
    }
    if temp.len() > 0 {
        current.1 = temp;
        hold.push(current);
    }

    hold
}

struct Wrapper {
    prev_nums: Vec<(usize, String)>,
    current_stars: Vec<usize>,
    current_nums: Vec<(usize, String)>,
    next_nums: Vec<(usize, String)>,
}

impl Wrapper {
    fn new(prev_nums: Vec<(usize, String)>, current_stars: Vec<usize>, current_nums: Vec<(usize, String)>, next_nums: Vec<(usize, String)>) -> Self {
        Self {prev_nums, current_stars, current_nums, next_nums}
    }

    
    fn is_possible(&self) -> u32{
        let mut possible: Vec<&String> = Vec::new();
        let mut vals: Vec<u32> = Vec::new();
        
        for  v in self.current_stars.iter() {
            // Previous
            for (loc, num) in &self.prev_nums {
                if (loc+num.len()) >= *v && loc <= &(v+1){
                    possible.push(num);
                }
            }

            // Current
            for (loc, num) in &self.current_nums {
                if loc == &(v+1) || loc == &(v-num.len()) {
                    possible.push(num);
                }
            }
            
            // Next
            for (loc, num) in &self.next_nums {
                if (loc+num.len()) >= *v && loc <= &(v+1){
                    possible.push(num);
                }
            }
            
            // Ensures only two gears are found
            if possible.len() != 2 {
                vals.push(0);
                possible = vec![];
            }
            else {
                let prod = possible.iter()
                    .map(|x| x.parse::<u32>().unwrap())
                    .fold(1, |acc, ele| acc * ele);
                vals.push(prod);
                possible = vec![];
            }
        }
        vals.iter().sum()
    }

}


fn part2(input: &Vec<String>) -> u32 {
    let mut wrappers: Vec<Wrapper> = Vec::new();

    for (index, line) in input.iter().enumerate() {
        let mut prev_nums = Vec::new();
        let mut next_nums = Vec::new();

        // Previous
        if index != 0 {
            prev_nums = parse_nums(&input.get(index-1).unwrap());
        }

        // Current stars
        let current_stars: Vec<_> = line.chars()
            .enumerate()
            .filter(|(_, c)| *c == '*')
            .map(|(i, _)| i)
            .collect();

        // Current nums
        let current_nums = parse_nums(&line);

        // Next
        if index != input.len() - 1 {
            next_nums = parse_nums(&input.get(index+1).unwrap());
        }
        wrappers.push(Wrapper::new(prev_nums, current_stars, current_nums, next_nums));
    }
    let test = wrappers.iter().map(|w| w.is_possible()).collect::<Vec<_>>();

    test.iter().sum()
}

fn main() {
    let input = include_str!("input.txt");
    let x = input.lines().map(|x| ".".to_owned() + x + ".").collect::<Vec<String>>();
    let output = part2(&x);
    println!("Output: {output}");
}
