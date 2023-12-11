fn part1() {
    let input = include_str!("input.txt");

    let mut id_sum = 0u32;

    for line in input.lines() {
        let id_data: Vec<&str> = line.split(':').collect();
        let game_and_id: Vec<&str> = id_data[0].split_whitespace().collect();
        let id = game_and_id[1].parse::<u32>().unwrap();

        let observations: Vec<&str> = id_data[1].split(';').collect();
        let mut game_possible = true;
        for observation in observations {
            let cube_colour_amounts: Vec<&str> = observation.split(',').collect();
            for cube_colour_amount in cube_colour_amounts {
                let cube_colour_amount: Vec<&str> = cube_colour_amount.split_whitespace().collect();
                let cube_amount = cube_colour_amount[0].parse::<u32>().unwrap();
                let cube_colour = cube_colour_amount[1];
                if cube_colour == "red" && cube_amount > 12 {
                    game_possible = false;
                } else if cube_colour == "green" && cube_amount > 13 {
                    game_possible = false;
                } else if cube_colour == "blue" && cube_amount > 14 {
                    game_possible = false;
                }
            }
        }
        if game_possible {
            id_sum += id;
        }
    }

    println!("Part 1: {}", id_sum);
}

fn part2() {
    let input = include_str!("input.txt");

    let mut sum = 0u32;

    for line in input.lines() {
        let id_data: Vec<&str> = line.split(':').collect();
        let mut minimum_red = 0u32;
        let mut minimum_green = 0u32;
        let mut minimum_blue = 0u32;
        let observations: Vec<&str> = id_data[1].split(';').collect();

        for observation in observations {
            let cube_colour_amounts: Vec<&str> = observation.split(',').collect();
            for cube_colour_amount in cube_colour_amounts {
                let cubes: Vec<&str> = cube_colour_amount.split_whitespace().collect();
                let cube_amount = cubes[0].parse::<u32>().unwrap();
                let cube_colour = cubes[1];
                if cube_colour == "red" && cube_amount > minimum_red {
                    minimum_red = cube_amount;
                } else if cube_colour == "green" && cube_amount > minimum_green {
                    minimum_green = cube_amount;
                } else if cube_colour == "blue" && cube_amount > minimum_blue {
                    minimum_blue = cube_amount;
                }
            }
        }

        let power = minimum_red * minimum_green * minimum_blue;
        sum += power;
    }

    println!("Part 2: {}", sum);
}

fn main() {
    part1();
    part2();
}
