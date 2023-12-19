use std::collections::HashMap;

fn part1() {
    let input = include_str!("./input.txt");

    let instructions_and_nodes: Vec<_> = input.split("\n").collect();

    let raw_instructions = instructions_and_nodes[0];
    let instructions = raw_instructions.chars().collect::<Vec<_>>();

    let raw_nodes = &instructions_and_nodes[2..instructions_and_nodes.len() - 1];

    let mut nodes = HashMap::new();
    for raw_node in raw_nodes {
        let current = &raw_node[0..3];
        let left = &raw_node[7..10];
        let right = &raw_node[12..15];

        nodes.insert(current, (left, right));
    }

    let mut current_node = "AAA";
    let mut steps_count = 0u64;
    for instruction in instructions.iter().cycle() {
        let (left, right) = nodes.get(current_node).unwrap();

        if *instruction == 'L' {
            current_node = left;
        } else if *instruction == 'R' {
            current_node = right;
        }

        steps_count += 1;

        if current_node == "ZZZ" {
            break;
        }
    }

    println!("Part 1: {}", steps_count);
}

fn main() {
    part1();
}
