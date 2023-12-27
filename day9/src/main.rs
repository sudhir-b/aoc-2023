use std::vec;

fn get_differences(numbers: Vec<isize>) -> Vec<isize> {
    let mut differences = vec![];
    for i in 1..numbers.len() {
        differences.push(numbers[i] - numbers[i - 1]);
    }
    differences
}

fn part1() {
    let input = include_str!("./input.txt");

    let mut sum_of_extrapolated_numbers = 0;

    for line in input.lines() {
        let numbers: Vec<isize> = line.split(" ").map(|x| x.parse().unwrap()).collect();

        let mut last_elements = vec![numbers.last().unwrap().clone()];

        let mut current_differences: Vec<isize> = get_differences(numbers);
        let mut zero_differences = current_differences.iter().all(|&x| x == 0);

        last_elements.push(current_differences.last().unwrap().clone());

        while !zero_differences {
            current_differences = get_differences(current_differences);
            last_elements.push(current_differences.last().unwrap().clone());
            zero_differences = current_differences.iter().all(|&x| x == 0);
        }

        last_elements.reverse();

        let mut carry: isize = 0;
        for i in 1..last_elements.len() {
            carry += last_elements[i];
        }

        sum_of_extrapolated_numbers += carry;
    }

    println!(
        "Sum of extrapolated numbers: {}",
        sum_of_extrapolated_numbers
    );
}

fn main() {
    part1();
}
