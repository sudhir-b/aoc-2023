use std::collections::HashSet;

fn part1() {
    let input = include_str!("./input.txt");
    let mut points_sum = 0;

    for line in input.lines() {
        let card_number_split: Vec<_> = line.split(":").collect();
        let numbers_split: Vec<_> = card_number_split[1].split("|").collect();

        let winning_numbers = numbers_split[0].split_whitespace();
        let playing_numbers = numbers_split[1].split_whitespace();

        let winning_numbers_set = winning_numbers.collect::<HashSet<_>>();
        let playing_numbers_set = playing_numbers.collect::<HashSet<_>>();

        let intersection = winning_numbers_set.intersection(&playing_numbers_set);

        let winning_numbers: Vec<_> = intersection.collect();

        let winning_numbers_count = winning_numbers.len() as u32;
        if winning_numbers_count == 0 {
            continue;
        }

        let points = u32::pow(2, winning_numbers_count - 1);

        points_sum += points;
    }

    println!("part 1: {}", points_sum);
}

fn main() {
    part1();
}
