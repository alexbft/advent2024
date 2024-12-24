use rand::Rng;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> String {
    let parts: Vec<_> = input.splitn(2, "\n\n").collect();

    let mut values = HashMap::new();
    for line in parts[0].lines() {
        let line_parts: Vec<_> = line.splitn(2, ": ").collect();
        let gate = line_parts[0];
        values.insert(gate, gate.to_string());
    }

    let mut wires = HashMap::new();
    let wire_regex = Regex::new(r"([a-z0-9]+) (AND|OR|XOR) ([a-z0-9]+) -> ([a-z0-9]+)").unwrap();
    for line in parts[1].lines() {
        let (_, [input1, func, input2, output]) = wire_regex.captures(line).unwrap().extract();
        wires.insert(
            output,
            Wire {
                input1: input1.to_string(),
                input2: input2.to_string(),
                func: func.to_string(),
            },
        );
    }

    // Trial and error! Find the combinations that work.
    swap_wires(&mut wires, "z13", "vcv");
    swap_wires(&mut wires, "z19", "vwp");
    swap_wires(&mut wires, "z25", "mps");
    swap_wires(&mut wires, "vjv", "cqm");
    
    let mut answer = vec!["z13", "vcv", "z19", "vwp", "z25", "mps", "vjv", "cqm"];
    answer.sort();
    println!("Answer: {}", answer.join(","));

    let mut oks = 0;
    loop {
        let xs = random_xs();
        let ys = random_xs();
        let zs = add(xs, ys);
        let swaps = test(&wires, &xs, &ys, &zs);
        if let Some(swaps) = swaps {
            if swaps.len() > 0 && swaps.len() <= 4 {
                println!("Swaps: {:?}", swaps);
                break;
            }
            oks = 0;
        } else {
            oks += 1;
            if oks > 1000 {
                println!("Verified!");
                break;
            }
        }
    }

    "Done".to_string()
}

fn random_xs() -> [u32; 45] {
    let mut rng = rand::rng();
    let mut xs = [0; 45];
    for i in 0..45 {
        xs[i] = rng.random_range(0..=1);
    }
    xs
}

fn add(xs: [u32; 45], ys: [u32; 45]) -> [u32; 46] {
    let mut zs = [0; 46];
    let mut carry = 0;
    for i in 0..45 {
        let sum = xs[i] + ys[i] + carry;
        zs[i] = sum % 2;
        carry = sum / 2;
    }
    zs[45] = carry;
    zs
}

fn get_value<'a>(
    gate: &'a str,
    wires: &'a HashMap<&'a str, Wire>,
    values: &mut HashMap<String, u32>,
) -> u32 {
    if let Some(value) = values.get(gate) {
        return *value;
    }

    let gate_s = gate.to_string();

    if gate.starts_with("x") || gate.starts_with("y") {
        values.insert(gate_s, 0);
        return 0;
    }

    values.insert(gate_s.clone(), 2);

    let wire = wires.get(gate).unwrap();
    let input1 = get_value(&wire.input1, wires, values);
    let input2 = get_value(&wire.input2, wires, values);
    let result = if input1 == 2 || input2 == 2 {
        2
    } else {
        match wire.func.as_str() {
            "AND" => input1 & input2,
            "OR" => input1 | input2,
            "XOR" => input1 ^ input2,
            _ => panic!("Unknown function: {}", wire.func),
        }
    };
    values.insert(gate_s, result);
    result
}

fn do_test(wires: &HashMap<&str, Wire>, xs: &[u32; 45], ys: &[u32; 45], zs: &[u32; 46]) -> bool {
    let mut values: HashMap<String, u32> = HashMap::new();
    for (i, &x) in xs.iter().enumerate() {
        values.insert(format!("x{:02}", i), x);
    }
    for (i, &y) in ys.iter().enumerate() {
        values.insert(format!("y{:02}", i), y);
    }
    for i in 0..46 {
        let actual = get_value(&format!("z{:02}", i), wires, &mut values);
        if zs[i] != actual {
            return false;
        }
    }
    true
}

fn test<'a>(
    wires: &HashMap<&'a str, Wire>,
    xs: &[u32; 45],
    ys: &[u32; 45],
    zs: &[u32; 46],
) -> Option<HashSet<(&'a str, &'a str)>> {
    if do_test(wires, xs, ys, zs) {
        return None;
    }
    let mut swaps_found = HashSet::new();
    for &i in wires.keys() {
        for &j in wires.keys() {
            if i == j {
                continue;
            }
            let mut new_wires = wires.clone();
            *(new_wires.get_mut(i).unwrap()) = wires[j].clone();
            *(new_wires.get_mut(j).unwrap()) = wires[i].clone();
            if do_test(&new_wires, xs, ys, zs) {
                if i < j {
                    swaps_found.insert((i, j));
                } else {
                    swaps_found.insert((j, i));
                }
            }
        }
    }
    Some(swaps_found)
}

fn swap_wires(wires: &mut HashMap<&str, Wire>, a: &str, b: &str) {
    let wire_a = wires[a].clone();
    let wire_b = wires[b].clone();
    *(wires.get_mut(a).unwrap()) = wire_b;
    *(wires.get_mut(b).unwrap()) = wire_a;
}

#[derive(Debug, Clone)]
struct Wire {
    input1: String,
    input2: String,
    func: String,
}
