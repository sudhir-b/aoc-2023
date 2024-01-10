use std::collections::HashMap;

fn arrange(springs: String, groups: Vec<u32>, cache: &mut HashMap<(String, Vec<u32>), usize>) -> usize {
    // if combination of springs and groups is in cache, return the cached value
    if let Some(&result) = cache.get(&(springs.clone(), groups.clone())) {
        return result;
    }

    // if no more groups:
    //     if no more damaged springs, we have a valid arrangement
    //     otherwise, we have an invalid arrangement
    //     store result in cache and return

    if groups.is_empty() {
        let valid: usize;
        if springs.chars().any(|c| c == '#') {
            valid = 0;
        } else {
            valid = 1;
        }

        cache.insert((springs, groups), valid);
        return valid;
    }

    // if there are groups:
    //     try to place the next group of springs
    //         if there aren't enough springs left, invalid arrangement
    //             return 0 and cache
    //         if next char is "." then chop off the first spring and recurse
    //             return result and cache
    //         if no "." in springs of length of group and the char after is not #
    //             chop off springs and group and recurse and add to running total
    //         if next char is ?, then try assuming it's "."
    //             chop off the first spring and recurse and add to running total
    //         insert running total into cache and return

    let required_space = groups.iter().sum::<u32>() + groups.len() as u32 - 1;
    if springs.len() < required_space as usize {
        cache.insert((springs, groups), 0);
        return 0;
    }

    let next_char = springs.chars().nth(0).unwrap();
    if next_char == '.' {
        let result = arrange(springs[1..].to_owned(), groups.clone(), cache);
        cache.insert((springs, groups), result);
        return result;
    }

    let group = groups[0];
    let mut running_total = 0;
    if springs[0..group as usize].chars().all(|c| c != '.')
        && springs[group as usize..].chars().nth(0) != Some('#')
    {
        let result = arrange(
            springs[group as usize + 1..].to_owned(),
            groups[1..].to_vec(),
            cache,
        );
        running_total += result;
    }

    if next_char == '?' {
        let result = arrange(springs[1..].to_owned(), groups.clone(), cache);
        running_total += result;
    }

    cache.insert((springs, groups), running_total);

    return running_total;
}

fn part1() {
    // TODO: try solving this with with finite state machines
    let input = include_str!("./input.txt");

    let mut sum = 0;
    let mut cache: HashMap<(String, Vec<u32>), usize> = HashMap::new();

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let mut springs = parts[0].to_owned();
        springs.push('.');
        let groups = parts[1].split(",")
            .map(|part| part.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        sum += arrange(springs, groups, &mut cache);
    }

    println!("Part 1: {}", sum);
}

fn part2() {
    let input = include_str!("./input.txt");

    let mut sum = 0;
    let mut cache: HashMap<(String, Vec<u32>), usize> = HashMap::new();

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let mut springs = parts[0].to_owned();
        springs.push('?');

        springs = springs.repeat(5);
        springs.pop();
        springs.push('.');

        let mut groups = parts[1].split(",")
            .map(|part| part.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        groups = groups.repeat(5);

        sum += arrange(springs, groups, &mut cache);
    }

    println!("Part 2: {}", sum);
}

fn main() {
    part1();
    part2();
}
