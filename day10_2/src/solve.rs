use std::collections::HashMap;

type Pos = (i32, i32);

pub fn solve(input: &str) -> usize {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut scores: HashMap<Pos, usize> = HashMap::new();
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;
    let mut result = 0usize;
    for y in 0..height {
        for x in 0..width {
            if get_at(&grid, x, y) == Some(0) {
                result += calc_score(&grid, &mut scores, x, y);
            }
        }
    }
    result
}

fn get_at(grid: &Vec<Vec<u32>>, x: i32, y: i32) -> Option<u32> {
    grid.get(y as usize)
        .and_then(|row| row.get(x as usize))
        .copied()
}

fn calc_score(grid: &Vec<Vec<u32>>, scores: &mut HashMap<Pos, usize>, x: i32, y: i32) -> usize {
    ensure_reach(grid, scores, x, y);
    scores.get(&(x, y)).copied().unwrap()
}

fn ensure_reach(grid: &Vec<Vec<u32>>, scores: &mut HashMap<Pos, usize>, x: i32, y: i32) {
    if scores.contains_key(&(x, y)) {
        return;
    }
    let cell_value_opt = get_at(grid, x, y);
    if cell_value_opt == None {
        scores.insert((x, y), 0usize);
        return;
    }
    let cell_value = cell_value_opt.unwrap();
    if cell_value == 9 {
        scores.insert((x, y), 1usize);
        return;
    }
    let mut score = 0usize;
    for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter() {
        let (nx, ny) = (x + dir.0, y + dir.1);
        if get_at(grid, nx, ny) == Some(cell_value + 1) {
            score += calc_score(grid, scores, nx, ny);
        }
    }
    scores.insert((x, y), score);
}

#[cfg(test)]
mod tests {
    use crate::solve::solve;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            89010123
            78121874
            87430965
            96549874
            45678903
            32019012
            01329801
            10456732
        "};
        assert_eq!(solve(input), 81usize);
    }
}
