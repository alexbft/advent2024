pub fn solve(input: &str) -> usize {
    let parts: Vec<_> = input.splitn(2, "\n\n").collect();
    let mut grid: Vec<Vec<char>> = parts[0]
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|c| match c {
                    '#' => vec!['#', '#'],
                    '.' => vec!['.', '.'],
                    '@' => vec!['@', '.'],
                    'O' => vec!['[', ']'],
                    _ => panic!("Invalid character: {}", c),
                })
                .collect()
        })
        .collect();
    let moves: String = parts[1].lines().collect();

    let (mut px, mut py) = find_start_pos(&grid);
    grid[py as usize][px as usize] = '.';
    for m in moves.chars() {
        let (dx, dy) = match m {
            '^' => (0, -1),
            'v' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => panic!("Invalid move: {}", m),
        };
        let (nx, ny) = (px + dx, py + dy);
        let nc = grid[ny as usize][nx as usize];
        match nc {
            '.' => {
                px = nx;
                py = ny;
            }
            '[' | ']' => {
                if try_move_box(&mut grid, nx, ny, dx, dy) {
                    px = nx;
                    py = ny;
                }
            }
            _ => {}
        }
    }
    let mut result = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == '[' {
                result += y * 100 + x;
            }
        }
    }
    result
}

fn find_start_pos(grid: &[Vec<char>]) -> (i32, i32) {
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == '@' {
                return (x as i32, y as i32);
            }
        }
    }
    panic!("No start position found");
}

fn try_move_box(grid: &mut Vec<Vec<char>>, x: i32, y: i32, dx: i32, dy: i32) -> bool {
    if dy == 0 {
        try_move_box_horizontal(grid, x, y, dx)
    } else {
        try_move_box_vertical(grid, x, y, dy)
    }
}

fn try_move_box_horizontal(grid: &mut Vec<Vec<char>>, x: i32, y: i32, dx: i32) -> bool {
    let mut nx = x;
    loop {
        nx += dx;
        let nc = grid[y as usize][nx as usize];
        if nc == '#' {
            return false;
        }
        if nc == '.' {
            break;
        }
    }
    grid[y as usize][x as usize] = '.';
    let mut write_x = x;
    let mut write_c = if dx > 0 { '[' } else { ']' };
    while write_x != nx {
        write_x += dx;
        grid[y as usize][write_x as usize] = write_c;
        write_c = if write_c == '[' { ']' } else { '[' };
    }
    true
}

fn try_move_box_vertical(grid: &mut Vec<Vec<char>>, x: i32, y: i32, dy: i32) -> bool {
    if can_move_box_vertical(grid, x, y, dy) {
        move_box_vertical(grid, x, y, dy);
        return true;
    }
    false
}

fn can_move_box_vertical(grid: &mut Vec<Vec<char>>, x: i32, y: i32, dy: i32) -> bool {
    move_box_vertical_rec(grid, x, y, dy, false)
}

fn move_box_vertical(grid: &mut Vec<Vec<char>>, x: i32, y: i32, dy: i32) {
    move_box_vertical_rec(grid, x, y, dy, true);
}

fn move_box_vertical_rec(
    grid: &mut Vec<Vec<char>>,
    x1: i32,
    y: i32,
    dy: i32,
    write: bool,
) -> bool {
    let c = grid[y as usize][x1 as usize];
    let x2 = if c == '[' { x1 + 1 } else { x1 - 1 };
    let ny = y + dy;
    for x in vec![x1, x2] {
        let nc = grid[ny as usize][x as usize];
        match nc {  
            '#' => return false,
            '.' => {},
            '[' | ']' => {
                if !move_box_vertical_rec(grid, x, ny, dy, write) {
                    return false;
                }
            },
            _ => panic!("Invalid character: {}", nc),
        }
    }
    if write {
        grid[ny as usize][x1 as usize] = grid[y as usize][x1 as usize];
        grid[ny as usize][x2 as usize] = grid[y as usize][x2 as usize];
        grid[y as usize][x1 as usize] = '.';
        grid[y as usize][x2 as usize] = '.';
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            ##########
            #..O..O.O#
            #......O.#
            #.OO..O.O#
            #..O@..O.#
            #O#..O...#
            #O..O..O.#
            #.OO.O.OO#
            #....O...#
            ##########
            
            <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
            vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
            ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
            <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
            ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
            ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
            >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
            <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
            ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
            v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
        "};
        assert_eq!(solve(input), 9021);
    }
}
