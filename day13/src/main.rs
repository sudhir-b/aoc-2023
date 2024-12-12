use std::string;

fn check_for_reflections<T: AsRef<str>>(
    pattern: &[T],
    horizontal: bool,
) -> (bool, usize, Option<usize>) {
    let mut reflection_found = false;
    let mut to_add = 0;
    let mut location: Option<usize> = None;

    // check for any two neighboring lines that are the same
    for i in 0..pattern.len() - 1 {
        if pattern[i].as_ref() == pattern[i + 1].as_ref() {
            // check other lines actually do reflect
            let mut j = i;
            let mut k = i + 1;
            let mut pattern_reflection_found = true;
            while j > 0 && k < pattern.len() - 1 {
                j -= 1;
                k += 1;
                if pattern[j].as_ref() != pattern[k].as_ref() {
                    pattern_reflection_found = false;
                    break;
                }
            }

            if pattern_reflection_found {
                if horizontal {
                    to_add = 100 * (i + 1);
                } else {
                    to_add = i + 1;
                }
                reflection_found = true;
                location = Some(i);
                break;
            }
        }
    }

    (reflection_found, to_add, location)
}

fn part1() {
    let input = include_str!("./input.txt");

    let mut sum = 0;

    for pattern in input.split("\n\n") {
        // parse pattern into horizontal vecs and look for horizontal reflection
        let horizontal: Vec<_> = pattern.lines().collect();

        let (horizontal_reflection_found, to_add, _) = check_for_reflections(&horizontal, true);
        sum += to_add;

        if horizontal_reflection_found {
            continue;
        }

        // parse pattern into vertical vecs and look for vertical reflection
        let mut vertical_chars = vec![vec![]; horizontal[0].len()];
        for line in horizontal {
            for (i, c) in line.chars().enumerate() {
                vertical_chars[i].push(c);
            }
        }

        let vertical = vertical_chars
            .iter()
            .map(|chars| chars.iter().collect::<String>())
            .collect::<Vec<String>>();

        let (vertical_reflection_found, to_add, _) = check_for_reflections(&vertical, false);
        sum += to_add;

        if !vertical_reflection_found {
            panic!("no reflection found for pattern: \n{}", pattern);
        }
    }

    println!("Part 1: {}", sum);
}

fn part2() {
    let input = include_str!("./input.txt");

    let mut sum = 0;

    for pattern in input.split("\n\n") {
        let horizontal: Vec<_> = pattern
            .la ines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => '0',
                        '#' => '1',
                        _ => panic!("unexpected character: {}", c),
                    })
                    .collect::<String>()
            })
            .map(|bin_string| u64::from_str_radix(&bin_string, 2).expect("Parsing error"))
            .collect();
        println!("{:?}", horizontal);
        println!("{:?}", horizontal[0] ^ horizontal[1]);

        // let bits = horizontal.iter().map(|string| {
        //     string
        //         .chars()
        //         .map(|c| if c == '#' { 1 } else { 0 })
        //         .collect::<Vec<_>>()
        // }).collect::<Vec<_>>();
        // println!("{:?}", bits);
    }
}

fn main() {
    // part1();
    part2();
}
