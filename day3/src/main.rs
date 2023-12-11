fn part1() {
    let input = include_str!("./input.txt");
    let lines: Vec<_> = input.lines().collect();
    let mut sum = 0;

    for (i, line) in lines.iter().enumerate() {
        let char_vec: Vec<char> = line.chars().collect();

        let mut number_indexes: Vec<(i32, i32)> = Vec::new();
        let mut start_index: i32 = -1;
        for index in 0..char_vec.len() {
            if char_vec[index].is_digit(10) {
                if start_index == -1 {
                    start_index = index as i32;
                }
            } else {
                if start_index != -1 {
                    number_indexes.push((start_index, index as i32));
                    start_index = -1;
                }
            }
        }

        if start_index != -1 {
            number_indexes.push((start_index, char_vec.len() as i32));
        }

        for number_index in &number_indexes {
            println!("{:?}", number_index);

            // check same line
            let number_value = char_vec[number_index.0 as usize..number_index.1 as usize]
                .iter()
                .collect::<String>()
                .parse::<i32>()
                .unwrap();

            println!("number value {}", number_value);

            let before_index;
            if number_index.0 - 1 < 0 {
                before_index = 0;
            } else {
                before_index = (number_index.0 - 1) as usize;
            }

            println!("before index {}", before_index);

            let previous_char = char_vec[before_index];
            println!("previous char {}", previous_char);
            if !previous_char.is_digit(10) && previous_char != '.' {
                println!("symbol found");
                sum += number_value;
                continue;
            }

            let after_index = number_index.1 as usize;
            if after_index != char_vec.len() {
                let next_char = char_vec[after_index];
                println!("next char {}", next_char);
                if !next_char.is_digit(10) && next_char != '.' {
                    println!("symbol found");
                    sum += number_value;
                    continue;
                }
            }

            let slice_end_index = if after_index == char_vec.len() {
                after_index
            } else {
                after_index + 1
            };

            // check previous line if this isn't the first line
            if i > 0 {
                let previous_line = lines[i - 1];
                let previous_char_vec: Vec<char> = previous_line.chars().collect();
                let previous_chars = previous_char_vec[before_index..slice_end_index]
                    .iter()
                    .collect::<String>();
                println!("previous line chars {}", previous_chars);
                // check for non-digit chars that are not periods
                if previous_chars.chars().any(|c| !c.is_digit(10) && c != '.') {
                    println!("symbol found");
                    sum += number_value;
                    continue;
                }
            }

            // check next line if this isn't the last line
            if i < lines.len() - 1 {
                let next_line = lines[i + 1];
                let next_char_vec: Vec<char> = next_line.chars().collect();
                let next_chars = next_char_vec[before_index..slice_end_index]
                    .iter()
                    .collect::<String>();
                println!("next line chars {}", next_chars);
                // check for non-digit chars that are not periods
                if next_chars.chars().any(|c| !c.is_digit(10) && c != '.') {
                    println!("symbol found");
                    sum += number_value;
                    continue;
                }
            }
        }
    }

    println!("{}", sum);
}

fn main() {
    part1();
}
