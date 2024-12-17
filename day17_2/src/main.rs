use ibig::ubig;

fn main() {
    // Program: 2,4,1,1,7,5,1,5,4,0,0,3,5,5,3,0
    // 2, 4: b := a % 8
    // 1, 1: b := b ^ 1
    // 7, 5: c := a >> b
    // 1, 5: b := b ^ 5
    // 4, 0: b := b ^ c
    // 0, 3: a := a >> 3
    // 5, 5: out b % 8
    // 3, 0: if a != 0 goto 0
    let expected_output = vec![2, 4, 1, 1, 7, 5, 1, 5, 4, 0, 0, 3, 5, 5, 3, 0];
    let result = rec(&expected_output, expected_output.len() - 1, 0);
    if let Some(result) = result {
        let mut res = ubig!(0);
        for a in result.iter().rev() {
            res = res * 8 + a;
        }
        println!("{}", res);
    } else {
        println!("No solution found");
    }
}

fn rec(expected_output: &Vec<i32>, index: usize, prev: i32) -> Option<Vec<i32>> {
    let base = prev << 3;
    for a in 0..8 {
        let output = check_output(base + a);
        if output == expected_output[index] {
            // println!("{}: {} -> {}", index, base + a, output);
            if index == 0 {
                return Some(vec![a]);
            }
            if let Some(mut result) = rec(expected_output, index - 1, (base + a) % 1024) {
                result.push(a);
                return Some(result);
            }
        }
    }
    None
}

fn check_output(a: i32) -> i32 {
    let b = a % 8;
    let b = b ^ 1;
    let c = a >> b;
    let b = b ^ 5;
    (b ^ c) % 8
}
