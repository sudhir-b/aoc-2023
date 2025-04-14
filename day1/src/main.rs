fn part1() {
    let input = include_str!("input.txt");
    let mut sum = 0;

    for line in input.lines() {
        let numbers: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

        // Using .unwrap() here assumes every line will have at least one digit.
        // For more robust code, consider handling the Option explicitly,
        // e.g., using `if let` or filtering lines without digits earlier.
        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();

        sum += 10 * first + last
    }

    println!("Part 1: {}", sum);
}

fn part2() {
    let input = include_str!("input.txt");
    let mut sum = 0;

    for line in input.lines() {
        let modified_line = line
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");
        let numbers: Vec<u32> = modified_line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect();

        // Using .unwrap() here assumes every processed line will have at least one digit
        // after the replacements. For more robust code, consider handling the Option
        // explicitly, similar to the note in part1.
        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();

        sum += 10 * first + last
    }

    println!("Part 2: {}", sum);
}

fn main() {
    part1();
    part2();
}
