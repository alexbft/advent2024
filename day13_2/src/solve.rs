use regex::Regex;

const PRICE_A: i64 = 3;
const PRICE_B: i64 = 1;

pub fn solve(input: &str) -> i64 {
    solve_n(input, 10000000000000)
}

fn solve_n(input: &str, add_n: i64) -> i64 {
    let lines: Vec<_> = input.lines().collect();
    let blocks = lines.split(|line| line.is_empty());
    let regex_a = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let regex_b = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let regex_prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    let mut result = 0;
    for block in blocks {
        let (_, [ax, ay]) = regex_a.captures(block[0]).unwrap().extract();
        let (_, [bx, by]) = regex_b.captures(block[1]).unwrap().extract();
        let (_, [px, py]) = regex_prize.captures(block[2]).unwrap().extract();
        let exercise = Exercise {
            a1: ax.parse().unwrap(),
            a2: ay.parse().unwrap(),
            b1: bx.parse().unwrap(),
            b2: by.parse().unwrap(),
            c1: px.parse::<i64>().unwrap() + add_n,
            c2: py.parse::<i64>().unwrap() + add_n,
        };
        if let Some(cost) = solve_exercise(exercise) {
            result += cost;
        }
    }
    result
}

fn solve_exercise(exercise: Exercise) -> Option<i64> {
    let (a1, a2, b1, b2, c1, c2) = (
        exercise.a1,
        exercise.a2,
        exercise.b1,
        exercise.b2,
        exercise.c1,
        exercise.c2,
    );
    let det = a1 * b2 - a2 * b1;
    if det == 0 {
        return None;
    }
    let fx = c1 * b2 - c2 * b1;
    if fx % det != 0 {
        return None;
    }
    let x = fx / det;
    let fy = a1 * c2 - a2 * c1;
    if fy % det != 0 {
        return None;
    }
    let y = fy / det;
    if x < 0 || y < 0 {
        return None;
    }
    Some(PRICE_A * x + PRICE_B * y)
}

struct Exercise {
    a1: i64,
    a2: i64,
    b1: i64,
    b2: i64,
    c1: i64,
    c2: i64,
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
        assert_eq!(solve_n(input, 0), 480i64);
    }
}
