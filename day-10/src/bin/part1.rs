#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Self{
        Self {x, y}
    }
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    point: Point
}

impl Node {
    fn new(point: Point, name: String) -> Self {
        Self {name, point}
    }
}

fn sol(input: &str) -> u32{
   let lines: Vec<String> = input.lines()
        .map(|l| l.to_string())
        .filter(|l| !l.is_empty())
        .collect();

    let loc = lines.iter()
        .position(|v| v.contains('S'))
        .unwrap();
    println!("S at {loc:?}");

    let mut nested_nodes: Vec<Vec<Node>> = Vec::new();

    for (y, line) in lines.iter().enumerate() {

        let symbols: Vec<_> = line.chars()
            .filter(|&c| c != '.')
            .collect();
        let mut nodes: Vec<Node> = line.chars().enumerate()
            .filter(|&(_, c)| c != '.')
            .map(|(x, s)| Node::new(Point::new(x.try_into().unwrap(), y.try_into().unwrap()), s.to_string()))
            .collect();
        //println!("Symbols: {symbols:?}");
        //println!("Nodes: {nodes:?}");

        nested_nodes.push(nodes);
    }

    //println!("Nested Nodes: {nested_nodes:?}");
    let all_nodes: Vec<Node> = nested_nodes.into_iter().flatten().collect();
    // Adds nodes to Graph
    for node in all_nodes.iter() {
        let mut paths: Vec<Node> = Vec::new();

    }

    println!("");
    

    0
}

fn main() {
    let input = include_str!("input.txt");
    println!("{input}");

    let output = sol(input);
    println!("Output: {output}");
}
