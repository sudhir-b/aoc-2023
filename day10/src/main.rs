fn part1() {
    let input = include_str!("./input.txt");

    let mut grid: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        let row: Vec<_> = line.chars().collect();
        grid.push(row);
    }

    // look for S
    // search around current position for next step in both directions
    // step through loop in both directions at the same time
    // exit when both steps are in the same grid position

    let mut starting_position = (0, 0);

    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                starting_position = (x, y);
            }
        }
    }

    let walker_starting_positions = find_connections(&grid, starting_position, None);

    let mut first_walker_previous = starting_position;
    let mut first_walker_current = walker_starting_positions[0];

    let mut second_walker_previous = starting_position;
    let mut second_walker_current = walker_starting_positions[1];

    let mut steps = 1;

    while first_walker_current != second_walker_current {
        let first_walker_connections =
            find_connections(&grid, first_walker_current, Some(first_walker_previous));

        let second_walker_connections =
            find_connections(&grid, second_walker_current, Some(second_walker_previous));

        first_walker_previous = first_walker_current;
        first_walker_current = first_walker_connections[0];

        second_walker_previous = second_walker_current;
        second_walker_current = second_walker_connections[0];

        steps += 1;
    }

    println!("Part 1: {}", steps);
}

fn find_connections(
    grid: &Vec<Vec<char>>,
    position: (usize, usize),
    previous_position: Option<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut connections: Vec<(usize, usize)> = vec![];

    let (x, y) = position;
    let current_char = grid[y][x];

    let mut up = false;
    let mut down = false;
    let mut left = false;
    let mut right = false;

    match current_char {
        'S' => {
            up = true;
            down = true;
            left = true;
            right = true;
        }
        '|' => {
            up = true;
            down = true;
        }
        '-' => {
            left = true;
            right = true;
        }
        'L' => {
            up = true;
            right = true;
        }
        'J' => {
            up = true;
            left = true;
        }
        '7' => {
            down = true;
            left = true;
        }
        'F' => {
            down = true;
            right = true;
        }
        _ => {
            panic!(
                "Not currently in a pipe: {} at {:?}",
                current_char, position
            );
        }
    }

    // check up
    if up && y > 0 {
        let up_char = grid[y - 1][x];
        match up_char {
            '|' | 'F' | '7' => {
                connections.push((x, y - 1));
            }
            _ => {}
        }
    }

    // check down
    if down && y < grid.len() - 1 {
        let down_char = grid[y + 1][x];
        match down_char {
            '|' | 'L' | 'J' => {
                connections.push((x, y + 1));
            }
            _ => {}
        }
    }

    // check left
    if left && x > 0 {
        let left_char = grid[y][x - 1];
        match left_char {
            '-' | 'L' | 'F' => {
                connections.push((x - 1, y));
            }
            _ => {}
        }
    }

    // check right
    if right && x < grid[y].len() - 1 {
        let right_char = grid[y][x + 1];
        match right_char {
            '-' | 'J' | '7' => {
                connections.push((x + 1, y));
            }
            _ => {}
        }
    }

    // remove previous position from connections
    if let Some(previous_position) = previous_position {
        connections.retain(|&x| x != previous_position);
    }

    connections
}

fn part2() {
    // go through and mark path on a copy of the grid
    // go around and fill in characters on one side in direction of travel until you hit a pipe
    // count number of filled in characters

    let input = include_str!("./input.txt");

    let mut grid: Vec<Vec<char>> = vec![];
    let mut path_grid: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        let row: Vec<_> = line.chars().collect();
        let row_length = row.len();
        grid.push(row);
        path_grid.push(vec!['.'; row_length]);
    }

    let mut starting_position = (0, 0);

    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                starting_position = (x, y);
            }
        }
    }

    path_grid[starting_position.1][starting_position.0] = 'S';

    let first_steps = find_connections(&grid, starting_position, None);

    // choose '-' instead of 'F' arbitrarily
    let mut current_position = first_steps[1];

    let mut previous_position = starting_position;

    while grid[current_position.1][current_position.0] != 'S' {
        path_grid[current_position.1][current_position.0] =
            convert_to_box_symbol(grid[current_position.1][current_position.0]);

        let connections = find_connections(&grid, current_position, Some(previous_position));

        if connections.len() == 0 {
            // probably back at the beginning
            break;
        }

        previous_position = current_position;
        current_position = connections[0];
    }

    previous_position = starting_position;
    current_position = first_steps[1];

    while path_grid[current_position.1][current_position.0] != 'S' {
        // fill grid to the right of the current pipe
        fill_grid(&mut path_grid, current_position, previous_position);

        let connections = find_connections(&grid, current_position, Some(previous_position));
        if connections.len() == 0 {
            // probably back at the beginning
            break;
        }

        previous_position = current_position;
        current_position = connections[0];
    }

    // count the 'I' characters
    let count = path_grid
        .iter()
        .flat_map(|x| x)
        .filter(|&x| *x == 'I')
        .count();

    println!("Part 2: {}", count);
}

fn fill_grid(
    path_grid: &mut Vec<Vec<char>>,
    current_position: (usize, usize),
    previous_position: (usize, usize),
) {
    // depending on the direction of travel, fill in the grid to the right of the current pipe
    // until you hit a pipe or a corner

    let (x, y) = current_position;
    let (previous_x, previous_y) = previous_position;
    let current_char = path_grid[y][x];

    let delta_x = x as isize - previous_x as isize;
    let delta_y = y as isize - previous_y as isize;

    if delta_x > 0 {
        // moving right
        // current char must be ┛, ┓, or ━
        match current_char {
            '┛' => {
                fill_down(path_grid, current_position);
                fill_right(path_grid, current_position);
            }
            '┓' => {
                fill_down(path_grid, current_position);
                fill_left(path_grid, current_position);
            }
            '━' => {
                fill_down(path_grid, current_position);
            }
            _ => panic!("Invalid current char: {}", current_char),
        }
    } else if delta_x < 0 {
        // moving left
        // current char must be ┗, ┏, or ━
        match current_char {
            '┗' => {
                fill_up(path_grid, current_position);
                fill_right(path_grid, current_position);
            }
            '┏' => {
                fill_up(path_grid, current_position);
                fill_left(path_grid, current_position);
            }
            '━' => {
                fill_up(path_grid, current_position);
            }
            _ => panic!("Invalid current char: {}", current_char),
        }
    } else if delta_y > 0 {
        // moving down
        // current char must be ┛, ┗, or ┃
        match current_char {
            '┛' => {
                fill_left(path_grid, current_position);
                fill_up(path_grid, current_position);
            }
            '┗' => {
                fill_left(path_grid, current_position);
                fill_down(path_grid, current_position);
            }
            '┃' => {
                fill_left(path_grid, current_position);
            }
            _ => panic!("Invalid current char: {}", current_char),
        }
    } else if delta_y < 0 {
        // moving up
        // current char must be ┓, ┏, or ┃
        match current_char {
            '┓' => {
                fill_right(path_grid, current_position);
                fill_up(path_grid, current_position);
            }
            '┏' => {
                fill_right(path_grid, current_position);
                fill_down(path_grid, current_position);
            }
            '┃' => {
                fill_right(path_grid, current_position);
            }
            _ => panic!("Invalid current char: {}", current_char),
        }
    }
}

fn fill_left(path_grid: &mut Vec<Vec<char>>, current_position: (usize, usize)) {
    let (x, y) = current_position;
    let mut x = x;

    while x > 0 {
        x -= 1;
        if is_path_char(path_grid[y][x]) {
            break;
        }
        path_grid[y][x] = 'I';
    }
}

fn fill_right(path_grid: &mut Vec<Vec<char>>, current_position: (usize, usize)) {
    let (x, y) = current_position;
    let mut x = x;

    while x < path_grid[y].len() - 1 {
        x += 1;
        if is_path_char(path_grid[y][x]) {
            break;
        }
        path_grid[y][x] = 'I';
    }
}

fn fill_up(path_grid: &mut Vec<Vec<char>>, current_position: (usize, usize)) {
    let (x, y) = current_position;
    let mut y = y;

    while y > 0 {
        y -= 1;
        if is_path_char(path_grid[y][x]) {
            break;
        }
        path_grid[y][x] = 'I';
    }
}

fn fill_down(path_grid: &mut Vec<Vec<char>>, current_position: (usize, usize)) {
    let (x, y) = current_position;
    let mut y = y;

    while y < path_grid.len() - 1 {
        y += 1;
        if is_path_char(path_grid[y][x]) {
            break;
        }
        path_grid[y][x] = 'I';
    }
}

fn is_path_char(c: char) -> bool {
    match c {
        '┗' | '┛' | '┓' | '┏' | '┃' | '━' | 'S' => true,
        _ => false,
    }
}

fn convert_to_box_symbol(c: char) -> char {
    match c {
        'L' => '┗',
        'J' => '┛',
        '7' => '┓',
        'F' => '┏',
        '|' => '┃',
        '-' => '━',
        _ => c,
    }
}

fn main() {
    part1();
    part2();
}
