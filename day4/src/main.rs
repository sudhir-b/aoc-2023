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

fn part2() {
    let input = include_str!("./input.txt");
    let line_count = input.lines().count();

    let mut card_counts = vec![1; line_count];

    for (i, line) in input.lines().enumerate() {
        let card_number_split: Vec<_> = line.split(":").collect();
        let numbers_split: Vec<_> = card_number_split[1].split("|").collect();

        let winning_numbers = numbers_split[0].split_whitespace();
        let playing_numbers = numbers_split[1].split_whitespace();

        let winning_numbers_set = winning_numbers.collect::<HashSet<_>>();
        let playing_numbers_set = playing_numbers.collect::<HashSet<_>>();

        let intersection = winning_numbers_set.intersection(&playing_numbers_set);

        let winning_numbers: Vec<_> = intersection.collect();

        let winning_numbers_count = winning_numbers.len();
        if winning_numbers_count == 0 {
            continue;
        }

        let start_index = i + 1;
        let end_index = start_index + winning_numbers_count;

        for _ in 0..card_counts[i] {
            for j in start_index..end_index {
                card_counts[j] += 1;
            }
        }
    }

    println!("part 2: {}", card_counts.iter().sum::<usize>());
}

fn main() {
    part1();
    part2();
}
