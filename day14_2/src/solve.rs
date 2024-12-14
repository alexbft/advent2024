use itertools::Itertools;
use regex::Regex;

pub fn solve(input: &str) -> String {
    solve_n(input, 101, 103, 10000)
}

fn solve_n(input: &str, width: i32, height: i32, time: i32) -> String {
    let robot_regex = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots: Vec<_> = input
        .lines()
        .map(|line| {
            let caps = robot_regex.captures(line).unwrap();
            let x = caps[1].parse::<i32>().unwrap();
            let y = caps[2].parse::<i32>().unwrap();
            let vx = caps[3].parse::<i32>().unwrap();
            let vy = caps[4].parse::<i32>().unwrap();
            Robot { x, y, vx, vy }
        })
        .collect();
    let mut buf = vec![];
    for i in 0..time {
        if are_robots_grouped(&robots, width, height) {
            buf.push(format!("=== TURN {} ===", i));

            let mut grid = vec![vec!['.'; width as usize]; height as usize];
            for robot in &robots {
                grid[robot.y as usize][robot.x as usize] = '#';
            }

            for row in grid {
                buf.push(row.into_iter().collect());
            }
        }

        for robot in &mut robots {
            robot.x = (robot.x + robot.vx + width) % width;
            robot.y = (robot.y + robot.vy + height) % height;
        }
    }
    buf.join("\n")
}

fn are_robots_grouped(robots: &[Robot], width: i32, height: i32) -> bool {
    let w2 = width / 2;
    let h2 = height / 2;
    let robots_num = robots.len();
    let count_by_quadrant = robots.iter().counts_by(|r| {
        let qx = r.x.cmp(&w2) as i8;
        let qy = r.y.cmp(&h2) as i8;
        (qx, qy)
    });
    let (min, max) = count_by_quadrant.into_iter().filter_map(|((qx, qy), v)| {
        if qx == 0 || qy == 0 {
            None
        } else {
            Some(v)
        }
    }).minmax().into_option().unwrap();
    (max - min) > robots_num / 2
}

struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}
