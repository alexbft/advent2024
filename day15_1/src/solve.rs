pub fn solve(input: &str) -> usize {
    let parts: Vec<_> = input.splitn(2, "\n\n").collect();
    let mut grid: Vec<Vec<char>> = parts[0]
        .lines()
        .map(|line| line.chars().collect())
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
            'O' => {
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
            if cell == 'O' {
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
    let mut nx = x;
    let mut ny = y;
    loop {
        nx += dx;
        ny += dy;
        let nc = grid[ny as usize][nx as usize];
        if nc == '#' {
            return false;
        }
        if nc == '.' {
            grid[ny as usize][nx as usize] = 'O';
            grid[y as usize][x as usize] = '.';
            return true;
        }
    }
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
        assert_eq!(solve(input), 10092);
    }
}
