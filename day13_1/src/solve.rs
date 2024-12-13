use regex::Regex;

const PRICE_A: usize = 3;
const PRICE_B: usize = 1;

pub fn solve(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();
    let blocks = lines.split(|line| line.is_empty());
    let regex_a = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let regex_b = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let regex_prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    let mut result = 0usize;
    for block in blocks {
        let (_, [ax, ay]) = regex_a.captures(block[0]).unwrap().extract();
        let (_, [bx, by]) = regex_b.captures(block[1]).unwrap().extract();
        let (_, [px, py]) = regex_prize.captures(block[2]).unwrap().extract();
        let exercise = Exercise {
            ax: ax.parse().unwrap(),
            ay: ay.parse().unwrap(),
            bx: bx.parse().unwrap(),
            by: by.parse().unwrap(),
            px: px.parse().unwrap(),
            py: py.parse().unwrap(),
        };
        if let Some((a, b)) = solve_exercise(exercise) {
            result += a * PRICE_A + b * PRICE_B;
        }
    }
    result
}

fn solve_exercise(exercise: Exercise) -> Option<(usize, usize)> {
    let mut min_cost = usize::MAX;
    let mut result = None;
    for a in 0..=100 {
        for b in 0..=100 {
            let target_x = a * exercise.ax + b * exercise.bx;
            let target_y = a * exercise.ay + b * exercise.by;
            if target_x == exercise.px && target_y == exercise.py {
                let cost = a * PRICE_A + b * PRICE_B;
                if cost < min_cost {
                    min_cost = cost;
                    result = Some((a, b));
                }
            }
        }
    }
    result
}

struct Exercise {
    ax: usize,
    ay: usize,
    bx: usize,
    by: usize,
    px: usize,
    py: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400
            
            Button A: X+26, Y+66
            Button B: X+67, Y+21
            Prize: X=12748, Y=12176
            
            Button A: X+17, Y+86
            Button B: X+84, Y+37
            Prize: X=7870, Y=6450
            
            Button A: X+69, Y+23
            Button B: X+27, Y+71
            Prize: X=18641, Y=10279
        "};
        assert_eq!(solve(input), 480usize);
    }
}
