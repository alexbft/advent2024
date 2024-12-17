use regex::Regex;

pub fn solve(input: &str) -> String {
    let parts: Vec<_> = input.splitn(2, "\n\n").collect();
    let register_regex =
        Regex::new(r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)").unwrap();
    let register_caps = register_regex.captures(parts[0].trim()).unwrap();
    let registers: Vec<i32> = [&register_caps[1], &register_caps[2], &register_caps[3]]
        .iter()
        .map(|value_s| value_s.parse().unwrap())
        .collect();
    let program_regex = Regex::new(r"Program: (.+)").unwrap();
    let program_s = program_regex.captures(parts[1].trim()).unwrap();
    let program: Vec<i32> = program_s[1]
        .split(",")
        .map(|value_s| value_s.parse().unwrap())
        .collect();
    let mut interpreter = Interpreter {
        registers,
        program,
        ip: 0,
        output: Vec::new(),
    };
    interpreter.run();
    interpreter
        .output
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

struct Interpreter {
    registers: Vec<i32>,
    program: Vec<i32>,
    ip: i32,
    output: Vec<i32>,
}

impl Interpreter {
    fn run(&mut self) {
        while self.ip >= 0 && self.ip < self.program.len() as i32 {
            if self.next_instruction() == None {
                break;
            }
        }
    }

    fn get_operand(&self, combo_operand: i32) -> Option<i32> {
        match combo_operand {
            0..=3 => Some(combo_operand),
            4..=6 => Some(self.registers[(combo_operand - 4) as usize]),
            _ => None,
        }
    }

    fn next_instruction(&mut self) -> Option<()> {
        let opcode = self.program[self.ip as usize];
        let operand = self.program.get((self.ip + 1) as usize)?;
        match opcode {
            0 => {
                // adv
                let combo_operand = self.get_operand(*operand)?;
                let num = self.registers[0];
                self.registers[0] = num >> combo_operand;
                self.ip += 2;
            }
            1 => {
                // bxl
                self.registers[1] = self.registers[1] ^ *operand;
                self.ip += 2;
            }
            2 => {
                // bst
                let combo_operand = self.get_operand(*operand)?;
                self.registers[1] = combo_operand % 8;
                self.ip += 2;
            }
            3 => {
                // jnz
                if self.registers[0] != 0 {
                    self.ip = *operand;
                } else {
                    self.ip += 2;
                }
            }
            4 => {
                // bxc
                self.registers[1] = self.registers[1] ^ self.registers[2];
                self.ip += 2;
            }
            5 => {
                // out
                let combo_operand = self.get_operand(*operand)?;
                self.output.push(combo_operand % 8);
                self.ip += 2;
            }
            6 => {
                // bdv
                let combo_operand = self.get_operand(*operand)?;
                let num = self.registers[0];
                self.registers[1] = num >> combo_operand;
                self.ip += 2;
            }
            7 => {
                // cdv
                let combo_operand = self.get_operand(*operand)?;
                let num = self.registers[0];
                self.registers[2] = num >> combo_operand;
                self.ip += 2;
            }
            _ => panic!("Unknown opcode: {}", opcode),
        }
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            Register A: 729
            Register B: 0
            Register C: 0
            
            Program: 0,1,5,4,3,0
        "};
        assert_eq!(solve(input), "4,6,3,5,6,3,5,2,1,0");
    }
}
