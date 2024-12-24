use regex::Regex;
use std::collections::HashMap;

pub fn solve(input: &str) -> u64 {
    let parts: Vec<_> = input.splitn(2, "\n\n").collect();

    let mut values = HashMap::new();
    for line in parts[0].lines() {
        let line_parts: Vec<_> = line.splitn(2, ": ").collect();
        let gate = line_parts[0];
        let value: u32 = line_parts[1].parse().unwrap();
        values.insert(gate, value);
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

    let mut result: u64 = 0;
    let mut zs: Vec<_> = wires.keys().filter(|&x| x.starts_with("z")).collect();
    zs.sort();
    for &&z in zs.iter().rev() {
        let value = get_value(z, &wires, &mut values);
        result = result * 2 + value as u64;
    }
    result
}

fn get_value<'a>(
    gate: &'a str,
    wires: &'a HashMap<&'a str, Wire>,
    values: &mut HashMap<&'a str, u32>,
) -> u32 {
    if let Some(value) = values.get(gate) {
        return *value;
    }

    let wire = wires.get(gate).unwrap();
    let input1 = get_value(&wire.input1, wires, values);
    let input2 = get_value(&wire.input2, wires, values);
    let result = match wire.func.as_str() {
        "AND" => input1 & input2,
        "OR" => input1 | input2,
        "XOR" => input1 ^ input2,
        _ => panic!("Unknown function: {}", wire.func),
    };
    values.insert(gate, result);
    result
}

struct Wire {
    input1: String,
    input2: String,
    func: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            x00: 1
            x01: 0
            x02: 1
            x03: 1
            x04: 0
            y00: 1
            y01: 1
            y02: 1
            y03: 1
            y04: 1
            
            ntg XOR fgs -> mjb
            y02 OR x01 -> tnw
            kwq OR kpj -> z05
            x00 OR x03 -> fst
            tgd XOR rvg -> z01
            vdt OR tnw -> bfw
            bfw AND frj -> z10
            ffh OR nrd -> bqk
            y00 AND y03 -> djm
            y03 OR y00 -> psh
            bqk OR frj -> z08
            tnw OR fst -> frj
            gnj AND tgd -> z11
            bfw XOR mjb -> z00
            x03 OR x00 -> vdt
            gnj AND wpb -> z02
            x04 AND y00 -> kjc
            djm OR pbm -> qhw
            nrd AND vdt -> hwm
            kjc AND fst -> rvg
            y04 OR y02 -> fgs
            y01 AND x02 -> pbm
            ntg OR kjc -> kwq
            psh XOR fgs -> tgd
            qhw XOR tgd -> z09
            pbm OR djm -> kpj
            x03 XOR y03 -> ffh
            x00 XOR y04 -> ntg
            bfw OR bqk -> z06
            nrd XOR fgs -> wpb
            frj XOR qhw -> z04
            bqk OR frj -> z07
            y03 OR x01 -> nrd
            hwm AND bqk -> z03
            tgd XOR rvg -> z12
            tnw OR pbm -> gnj
        "};
        assert_eq!(solve(input), 2024);
    }
}
