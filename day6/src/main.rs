fn part1(input: Vec<(i32, i32)>) {
    let mut product = 1;
    for (total_time, distance_to_beat) in input {
        let mut count = 0;
        for button_time in 0..total_time {
            let distance = button_time * (total_time - button_time);
            if distance > distance_to_beat {
                count += 1;
            }
        }
        product *= count;
    }

    println!("Part 1: {}", product);
}

fn part2(total_time: u64, distance_to_beat: u64) {
    let mut count = 0u64;
    for button_time in 0..total_time {
        let distance = button_time * (total_time - button_time);
        if distance > distance_to_beat {
            count += 1;
        }
    }

    println!("Part 2: {}", count);
}

fn main() {
    let input: Vec<(i32, i32)> = vec![(38, 241), (94, 1549), (79, 1074), (70, 1091)];
    part1(input);

    part2(38947970, 241154910741091)
}
