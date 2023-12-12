use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Eq, Clone)]
enum Type {
    FIVE(String, u32, Vec<u32>),
    FOUR(String, u32, Vec<u32>),
    HOUSE(String, u32, Vec<u32>),
    THREE(String, u32, Vec<u32>),
    TWO(String, u32, Vec<u32>),
    ONE(String, u32, Vec<u32>),
    HIGH(String, u32, Vec<u32>)
}

impl Ord for Type {
    fn cmp(&self, other: &Type) -> Ordering{
        match (self, other) {
            (Type::FIVE(_, _, self_hand), Type::FIVE(_, _, other_hand)) | 
            (Type::FOUR(_, _, self_hand), Type::FOUR(_, _, other_hand)) |
            (Type::HOUSE(_, _, self_hand), Type::HOUSE(_, _, other_hand)) |
            (Type::THREE(_, _, self_hand), Type::THREE(_, _, other_hand)) |
            (Type::TWO(_, _, self_hand), Type::TWO(_, _, other_hand)) |
            (Type::ONE(_, _, self_hand), Type::ONE(_, _, other_hand)) |
            (Type::HIGH(_, _, self_hand), Type::HIGH(_, _, other_hand)) => {

                for index in 0..5 {
                    if self_hand[index] > other_hand[index] {
                        return Ordering::Greater;
                    }
                    else if self_hand[index] < other_hand[index] {
                        return Ordering::Less;
                    }
                }
                return Ordering::Equal;
            },
            _ => unreachable!()
        }
    }
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Type) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Type) -> bool{
         match (self, other) {
            (Type::FIVE(_, _, self_hand), Type::FIVE(_, _, other_hand)) | 
            (Type::FOUR(_, _, self_hand), Type::FOUR(_, _, other_hand)) |
            (Type::HOUSE(_, _, self_hand), Type::HOUSE(_, _, other_hand)) |
            (Type::THREE(_, _, self_hand), Type::THREE(_, _, other_hand)) |
            (Type::TWO(_, _, self_hand), Type::TWO(_, _, other_hand)) |
            (Type::ONE(_, _, self_hand), Type::ONE(_, _, other_hand)) |
            (Type::HIGH(_, _, self_hand), Type::HIGH(_, _, other_hand)) => {
                    self_hand == other_hand
            },
            _ => unreachable!()

        }
    }
}

impl Type {
    fn new(line: &str) -> Self {
        let hold: (&str, &str) = line.split_once(' ').unwrap();
        let hand: String = hold.0.to_string();
        let bid: u32 = hold.1.parse().unwrap();

        let mut occurences: Vec<usize> = Vec::with_capacity(5);
        let mut hash: HashMap<char, u32> = HashMap::new();
        let mut values: Vec<u32> = Vec::with_capacity(5);
        
        hash.insert('A', 14);
        hash.insert('K', 13);
        hash.insert('Q', 12);
        hash.insert('J', 11);
        hash.insert('T', 10);
        hash.insert('9', 9);
        hash.insert('8', 8);
        hash.insert('7', 7);
        hash.insert('6', 6);
        hash.insert('5', 5);
        hash.insert('4', 4);
        hash.insert('3', 3);
        hash.insert('2', 2);

        for card in hand.chars() {
            values.push(*hash.get(&card).unwrap());
            occurences.push(hand.matches(card).count());
        }

        // All the same
        if occurences.contains(&5) {
            return Self::FIVE(hand, bid, values);
        }
        // 4 same, 1 different
        else if occurences.contains(&4) {
            return Self::FOUR(hand, bid, values);
        }
        // 3 same, 2 different OR 3 same, 2 same
        else if occurences.contains(&3) {
            // 2 same
            if occurences.contains(&2) {
                return Self::HOUSE(hand, bid, values);
            }
            else {
                return Self::THREE(hand, bid, values);
            }
        }
        // 2 same, 2 same, and 1 different OR 2 same, 3 different
        else if occurences.contains(&2) {
            if occurences.iter().filter(|x| **x == 1).count() == 3 {
                return Self::ONE(hand, bid, values);
            }
            else {
                return Self::TWO(hand, bid, values);
            }
        } 

        Self::HIGH(hand, bid, values)
    }
}

#[derive(Debug)]
struct Game {
    fives: Vec<Type>,
    fours: Vec<Type>,
    houses: Vec<Type>,
    threes: Vec<Type>,
    twos: Vec<Type>,
    ones: Vec<Type>,
    highs: Vec<Type>
}

impl Game {
    fn new() -> Self {
        Self {fives: vec![], fours: vec![], houses: vec![], threes: vec![], twos: vec![], ones: vec![], highs: vec![]}
    }

    fn add(&mut self, val: Type) {
        match val {
            Type::FIVE(_, _, _) => self.fives.push(val),
            Type::FOUR(_, _, _) => self.fours.push(val),
            Type::HOUSE(_, _, _) => self.houses.push(val),
            Type::THREE(_, _, _) => self.threes.push(val),
            Type::TWO(_, _, _) => self.twos.push(val),
            Type::ONE(_, _, _) => self.ones.push(val),
            Type::HIGH(_, _, _) => self.highs.push(val),
            _ => unreachable!()
        }
    }

    fn sort(&mut self) {
        self.fives.sort();
        self.fours.sort();
        self.houses.sort();
        self.threes.sort();
        self.twos.sort();
        self.ones.sort();
        self.highs.sort();
    }

    fn get_rank(&self) -> u32 {
        let mut values: Vec<u32> = Vec::new();

        for item in &self.highs {
            if let Type::HIGH(_, val, _) = item {
                values.push(*val);
            }
        }

        for item in &self.ones {
            if let Type::ONE(_, val, _) = item {
                values.push(*val);
            }
        }

        for item in &self.twos {
            if let Type::TWO(_, val, _) = item {
                values.push(*val);
            }
        }

        for item in &self.threes {
            if let Type::THREE(_, val, _) = item {
                values.push(*val);
            }
        }

        for item in &self.houses {
            if let Type::HOUSE(_, val, _) = item {
                values.push(*val);
            }
        }

        for item in &self.fours {
            if let Type::FOUR(_, val, _) = item {
                values.push(*val);
            }
        }

        for item in &self.fives {
            if let Type::FIVE(_, val, _) = item {
                values.push(*val);
            }
        }

        values.iter().enumerate().map(|(i, v)| (v*(i as u32 +1)) as u32).sum()
    }
}

fn part1(input: &str) -> u32 {

    let types: Vec<Type> = input.lines().map(Type::new).collect();

    let mut game = Game::new();
    for val in types {
        game.add(val);
    }

    game.sort();

    game.get_rank()
}

fn main() {
    let input = include_str!("input.txt");

    let output = part1(input);
    println!("Output: {output}");
}
