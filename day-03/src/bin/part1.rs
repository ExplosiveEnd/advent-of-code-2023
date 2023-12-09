struct Wrapper {
    prev_specials: Option<Vec<usize>>,
    current_nums: Vec<(usize, String)>,
    next_specials: Option<Vec<usize>>,
    current_specials: Option<Vec<usize>>
}

impl Wrapper {
    fn new(prev_specials: Option<Vec<usize>>, current_nums: Vec<(usize, String)>, next_specials: Option<Vec<usize>>, current_specials: Option<Vec<usize>>) -> Self {
        Self {prev_specials, current_nums, next_specials, current_specials}
    }

    fn is_possible(&self) -> Vec<String>{
        let mut possible: Vec<String> = Vec::new();
        let mut used: Vec<usize> = Vec::new();

        for (index, item) in &self.current_nums {
            let len = item.len();

            // Previous
            match &self.prev_specials {
                Some(nums) => {
                    for num in nums {
                        match index {
                            0 => {
                                if num <= &(index + len) && !used.contains(index) {
                                    possible.push(item.clone());
                                    used.push(*index);
                                }
                            },
                            _ => {
                                if num >= &(index - 1) && num <= &(index + len) && !used.contains(index) {
                                    possible.push(item.clone());
                                    used.push(*index);
                                }
                            }
                        }
                    }
                },
                None => {}
            }

            // Current row
            match &self.current_specials {
                Some(nums) => {
                    for num in nums {
                        match index {
                            0 => {
                                if num == &(index + len) && !used.contains(index) {
                                    possible.push(item.clone());
                                    used.push(*index);
                                }
                            },
                            _ => {
                                if (num == &(index - 1) || num == &(index + len)) && !used.contains(index) {
                                    possible.push(item.clone());
                                    used.push(*index);
                                }
                            }
                        }
                    }
                },
                None => {}
            }


            // Next row
            match &self.next_specials {
                Some(nums) => {
                    for num in nums {
                        match index {
                            0 => {
                                if num <= &(index + len) && !used.contains(index) {
                                    possible.push(item.clone());
                                    used.push(*index);
                                }
                            },
                            _ => {
                                if num >= &(index-1) && num <= &(index + len) && !used.contains(index) {
                                    possible.push(item.clone());
                                    used.push(*index);
                                }
                            }
                        }
                    }
                },
                None => {}
            }
        }
        possible 
    }

}

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

fn part2(input: &Vec<String>) -> u32 {
    0
}

fn part1(input: &Vec<String>) -> u32 {
    let mut wrappers: Vec<Wrapper> = Vec::new();
    //let lines: Vec<String> = input.lines().map(|x| x.to_string()).collect();

    for (index, line) in input.iter().enumerate() {
        let mut prev_specials: Vec<usize> = vec![];
        let mut next_specials: Vec<usize> = vec![];

        //let line = p_line;

        if index != 0 {
            // Prev_specials
            match input.get(index -1) {
                Some(prev_line) => {
                    prev_specials = prev_line.chars()
                        .enumerate()
                        .filter(|(_,x)| !x.is_alphanumeric() && *x != '.')
                        .map(|(i, _)| i)
                        .collect::<Vec<usize>>();
    
                },
                None => {}
            }
        }

        if index != input.len() - 1 {
            // Next_specials
            match input.get(index+1) {
                Some(next_line) => {
                    next_specials = next_line.chars()
                        .enumerate()
                        .filter(|(_, x)| !x.is_alphanumeric() && *x != '.')
                        .map(|(i, _)| i)
                        .collect::<Vec<usize>>();
                },
                None => {}
            }
        }

        let nums = parse_nums(line);


        // Current_specials
        let current_specials: Vec<usize> = line.chars()
            .enumerate()
            .filter(|(_, x)| !x.is_numeric() && *x != '.')
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        wrappers.push(Wrapper::new(Some(prev_specials), nums, Some(next_specials), Some(current_specials)));
    }

    let values: Vec<_> = wrappers.iter().map(|x| x.is_possible()).flatten().map(|x| x.parse::<u32>().unwrap_or(0)).collect();


    values.iter().sum()
}

fn main() {
    let input = include_str!("input.txt");
    let x = input.lines().map(|x| ".".to_owned() + x + ".").collect::<Vec<String>>();
    let output = part1(&x);
    println!("Output: {output}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn easy_parse() {
        let inp = "5..5";
        let output = parse_nums(&inp.to_string());
        assert_eq!(output, vec![(0usize, String::from("5")), (3usize, String::from("5"))]);
    }
    #[test]
    fn rand_parse() {
        let inp = ".789...42.288...#.......226".to_string();
        let output = parse_nums(&inp);
        assert_eq!(output, vec![ (1usize, String::from("789")), 
                                (7usize, String::from("42")), 
                                (10usize, String::from("288")),
                                (24usize, String::from("226"))
                               ]);
    }

    #[test]
    fn from_start() {
        let inp = "777*241.".to_string();
        let output = parse_nums(&inp);
        assert_eq!(output, vec![ (0usize, String::from("777")),
                                (4usize, String::from("241")) ]);
    }

    #[test]
    fn from_end() {
        let inp = "...777*241".to_string();
        let output = parse_nums(&inp);
        assert_eq!(output, vec![ (3usize, String::from("777")),
                                (7usize, String::from("241")) ]);
    }

    #[test]
    fn negative() {
        let inp = "-777.235.-3".to_string();
        let output = parse_nums(&inp);
        assert_eq!(output, vec![ (1usize, String::from("777")),
                                (5usize, String::from("235")),
                                (10usize, String::from("3"))
                               ]);
    }
}
