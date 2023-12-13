fn part1() {
    let input = include_str!("./input.txt");

    let groups: Vec<_> = input.split("\n\n").collect();

    let mut seed_strings: Vec<_> = groups[0].split_whitespace().collect();
    seed_strings.remove(0);

    let mut seeds: Vec<u64> = seed_strings
        .iter()
        .map(|seed| seed.parse::<u64>().unwrap())
        .collect();

    let mut maps = vec![];
    for i in 1..=7 {
        let mut split_group: Vec<_> = groups[i].split("\n").collect();
        split_group.remove(0);

        // remove empty strings
        split_group.retain(|line| line != &"");

        let map: Vec<(u64, u64, u64)> = split_group
            .iter()
            .map(|line| {
                let split: Vec<_> = line.split_whitespace().collect();
                (
                    split[0].parse::<u64>().unwrap(),
                    split[1].parse::<u64>().unwrap(),
                    split[2].parse::<u64>().unwrap(),
                )
            })
            .collect();

        maps.push(map);
    }

    for map in maps {
        for i in 0..seeds.len() {
            for mapping in &map {
                // check seed is in the range of mapping.1 to mapping.1 + mapping.2
                let source_range_start = mapping.1;
                let source_range_end = mapping.1 + mapping.2;

                let offset: Option<u64>;
                if seeds[i] >= source_range_start && seeds[i] <= source_range_end {
                    offset = Some(seeds[i] - source_range_start);
                } else {
                    offset = None;
                }

                if let Some(offset) = offset {
                    seeds[i] = mapping.0 + offset;
                    break;
                }
            }
        }
    }

    // get min seed
    let min_seed = seeds.iter().min().unwrap();
    println!("part 1: {:?}", min_seed);
}

fn part2() {
    let input = include_str!("./input.txt");
    let groups: Vec<_> = input.split("\n\n").collect();

    let mut seed_ranges_strings: Vec<_> = groups[0].split_whitespace().collect();
    seed_ranges_strings.remove(0);

    let seed_ranges: Vec<u64> = seed_ranges_strings
        .iter()
        .map(|seed| seed.parse::<u64>().unwrap())
        .collect();

    let mut maps = vec![];
    for i in 1..=7 {
        let mut split_group: Vec<_> = groups[i].split("\n").collect();
        split_group.remove(0);

        // remove empty strings
        split_group.retain(|line| line != &"");

        let map: Vec<(u64, u64, u64)> = split_group
            .iter()
            .map(|line| {
                let split: Vec<_> = line.split_whitespace().collect();
                (
                    split[0].parse::<u64>().unwrap(),
                    split[1].parse::<u64>().unwrap(),
                    split[2].parse::<u64>().unwrap(),
                )
            })
            .collect();

        maps.push(map);
    }

    let mut seeds = vec![];
    for i in (0..seed_ranges.len()).step_by(2) {
        seeds.push(seed_ranges[i]);
        seeds.push(seed_ranges[i] + seed_ranges[i + 1]);
    }

    for map in maps {
        let mut new_seeds = vec![];
        while seeds.len() > 0 {
            let end = seeds.pop().unwrap();
            let start = seeds.pop().unwrap();

            let mut range_processed = false;

            for (destination_range_start, source_range_start, range_length) in &map {
                let overlap_start = u64::max(start, *source_range_start);
                let overlap_end = u64::min(end, *source_range_start + *range_length);

                if overlap_start < overlap_end {
                    new_seeds.push(destination_range_start + (overlap_start - source_range_start));
                    new_seeds.push(destination_range_start + (overlap_end - source_range_start));

                    if start < overlap_start {
                        seeds.push(start);
                        seeds.push(overlap_start);
                    }

                    if overlap_end < end {
                        seeds.push(overlap_end);
                        seeds.push(end);
                    }

                    range_processed = true;
                    break;
                }
            }

            if !range_processed {
                new_seeds.push(start);
                new_seeds.push(end);
            }
        }
        seeds = new_seeds;
    }

    // get min seed in seeds
    let min_seed = seeds.iter().min().unwrap();
    println!("part 2: {:?}", min_seed);
}

fn main() {
    part1();
    part2();
}
