fn transpose(input_matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut output_matrix: Vec<Vec<char>> = vec![];

    for i in 0..input_matrix[0].len() {
        let mut row: Vec<char> = vec![];
        for j in 0..input_matrix.len() {
            row.push(input_matrix[j][i]);
        }
        output_matrix.push(row);
    }

    output_matrix
}

fn expand_rows(grid: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result_grid: Vec<Vec<char>> = vec![];

    // expand empty rows
    for row in grid.iter() {
        if row.iter().all(|&c| c == '.') {
            result_grid.push(vec!['.'; row.len()]);
        }
        result_grid.push(row.to_vec());
    }

    result_grid
}

fn expand_space(grid: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
    let expanded_rows = expand_rows(grid);
    let expanded_columns = expand_rows(&mut transpose(&expanded_rows));
    transpose(&expanded_columns)
}

type Position = (usize, usize);

fn get_pairs(input: Vec<Position>) -> Vec<(Position, Position)> {
    let mut result: Vec<(Position, Position)> = vec![];

    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            result.push((input[i], input[j]));
        }
    }

    result
}

fn part1() {
    let input = include_str!("./input.txt");
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let expanded = expand_space(&mut grid);

    let mut hash_positions: Vec<(usize, usize)> = vec![];

    for (i, row) in expanded.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '#' {
                hash_positions.push((i, j));
            }
        }
    }

    let mut path_sum = 0;

    for (first, second) in get_pairs(hash_positions) {
        let (x2, y2) = second;
        let (x1, y1) = first;

        let x_diff: i32 = (x2 as i32 - x1 as i32).abs();
        let y_diff: i32 = (y2 as i32 - y1 as i32).abs();

        let distance = x_diff + y_diff;
        path_sum += distance;
    }

    println!("Part 1: {}", path_sum);
}

fn part2() {
    let input = include_str!("./input.txt");
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut hash_positions: Vec<(usize, usize)> = vec![];

    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '#' {
                hash_positions.push((i, j));
            }
        }
    }

    let mut x_increments: Vec<usize> = vec![0; hash_positions.len()];
    let mut y_increments: Vec<usize> = vec![0; hash_positions.len()];

    for (i, row) in grid.iter().enumerate() {
        if row.iter().all(|&c| c == '.') {
            for (j, hash_position) in hash_positions.iter().enumerate() {
                if hash_position.0 > i {
                    x_increments[j] += 999_999;
                }
            }
        }
    }

    for (i, row) in transpose(&grid).iter().enumerate() {
        if row.iter().all(|&c| c == '.') {
            for (j, hash_position) in hash_positions.iter().enumerate() {
                if hash_position.1 > i {
                    y_increments[j] += 999_999;
                }
            }
        }
    }

    for (i, hash_position) in hash_positions.iter_mut().enumerate() {
        hash_position.0 += x_increments[i];
        hash_position.1 += y_increments[i];
    }

    let mut path_sum: usize = 0;

    for (first, second) in get_pairs(hash_positions) {
        let (x2, y2) = second;
        let (x1, y1) = first;

        let x_diff: usize = (x2 as i32 - x1 as i32).abs().try_into().unwrap();
        let y_diff: usize = (y2 as i32 - y1 as i32).abs().try_into().unwrap();

        let distance = x_diff + y_diff;
        path_sum += distance;
    }

    println!("Part 2: {}", path_sum);
}

fn main() {
    part1();
    part2();
}
