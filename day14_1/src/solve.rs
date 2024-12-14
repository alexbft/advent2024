use regex::Regex;
use itertools::Itertools;

pub fn solve(input: &str) -> i32 {
    solve_n(input, 101, 103, 100)
}

fn solve_n(input: &str, width: i32, height: i32, time: i32) -> i32 {
    let robot_regex = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let w2 = width / 2;
    let h2 = height / 2;
    let robots = input.lines().map(|line| {
        let caps = robot_regex.captures(line).unwrap();
        let x = caps[1].parse::<i32>().unwrap();
        let y = caps[2].parse::<i32>().unwrap();
        let vx = caps[3].parse::<i32>().unwrap();
        let vy = caps[4].parse::<i32>().unwrap();
        let nx = (width + (x + vx * time) % width) % width;
        let ny = (height + (y + vy * time) % height) % height;
        (nx, ny)
    });
    let count_by_quadrant = robots.counts_by(|(x, y)| {
        let qx = x.cmp(&w2) as i8;
        let qy = y.cmp(&h2) as i8;
        (qx, qy)
    });
    count_by_quadrant.into_iter().filter_map(|((qx, qy), v)| {
        if qx == 0 || qy == 0 {
            None
        } else {
            // println!("{},{}: {}", qx, qy, v);
            Some(v)
        }
    }).product::<usize>() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            p=0,4 v=3,-3
            p=6,3 v=-1,-3
            p=10,3 v=-1,2
            p=2,0 v=2,-1
            p=0,0 v=1,3
            p=3,0 v=-2,-2
            p=7,6 v=-1,-3
            p=3,0 v=-1,-2
            p=9,3 v=2,3
            p=7,3 v=-1,2
            p=2,4 v=2,-3
            p=9,5 v=-3,-3
        "};
        assert_eq!(solve_n(input, 11, 7, 100), 12);
    }
}
