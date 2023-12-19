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

fn part2() {
    let input = include_str!("./input.txt");

    let instructions_and_nodes: Vec<_> = input.split("\n").collect();

    let raw_instructions = instructions_and_nodes[0];
    let instructions = raw_instructions.chars().collect::<Vec<_>>();

    let raw_nodes = &instructions_and_nodes[2..instructions_and_nodes.len() - 1];

    let mut nodes = HashMap::new();
    let mut current_nodes: Vec<&str> = vec![];
    for raw_node in raw_nodes {
        let current = &raw_node[0..3];
        let left = &raw_node[7..10];
        let right = &raw_node[12..15];

        nodes.insert(current, (left, right));
        if current.ends_with("A") {
            current_nodes.push(current);
        }
    }

    println!("Starting nodes: {:?}", current_nodes);

    let mut step_counts = vec![0u64; current_nodes.len()];
    for i in 0..current_nodes.len() {
        for instruction in instructions.iter().cycle() {
            let (left, right) = nodes.get(current_nodes[i]).unwrap();
            if *instruction == 'L' {
                current_nodes[i] = left;
            } else if *instruction == 'R' {
                current_nodes[i] = right;
            }

            step_counts[i] += 1;

            if current_nodes[i].ends_with("Z") {
                break;
            }
        }
    }

    // calculate lowest common multiple of the step_counts
    println!("Part 2: {:?}", lcm_vector(step_counts));
}

fn lcm_vector(v: Vec<u64>) -> u64 {
    let mut result = v[0];
    for v in v.into_iter().skip(1) {
        result = lcm(result, v);
    }
    result
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    part1();
    part2();
}
