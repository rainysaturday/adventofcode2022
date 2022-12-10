
fn main() {
    let instrs = include_str!("../../../input10")
        .lines()
        .map(|l| {
            let parts = l.split(" ").collect::<Vec<&str>>();
            match parts[0] {
                "noop" => Instr::NOOP,
                "addx" => Instr::ADDX(parts[1].parse::<i64>().expect("Must be parseable number")),
                other => panic!("unknown instruction {}", other)
            }
        })
        .collect::<Vec<Instr>>();

    let mut vm = VM::from(instrs.clone());
    let boundaries: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
    let mut reg_x_values = Vec::new();
    for _ in 0..300 {
        let x_during = vm.reg_x;
        if let Some(is_lit) = vm.run_cycle() {
            print!("{}", is_lit);
            if vm.current_cycle % 40 == 0 {
                println!("");
            }
        } else {
            break;
        }
        if boundaries.contains(&vm.current_cycle) { 
            reg_x_values.push(x_during);
        }
    }

    println!("Part1: sum {}", reg_x_values.iter()
        .zip(boundaries.iter())
        .map(|(x, b)| x * (*b as i64))
        .sum::<i64>());
}


#[derive(Clone, Debug)]
enum Instr {
    NOOP,
    ADDX(i64),
}

impl Instr {
    fn cycle_to_complete(&self) -> usize {
        match self {
            Self::NOOP => 1,
            Self::ADDX(_) => 2,
        }
    }
}


struct VM {
    program: Vec<Instr>,
    pc: usize,
    current_cycle: usize,
    inter_instr_cycle: usize,
    reg_x: i64,
}

impl VM {
    fn from(program: Vec<Instr>) -> VM {
        VM {
            program,
            pc: 0,
            current_cycle: 0,
            inter_instr_cycle: 0,
            reg_x: 1,
        }
    }

    fn run_cycle(&mut self) -> Option<char> {
        if self.pc >= self.program.len() {
            return None;
        }

        let crt_h_pos = (self.current_cycle % 40) as i64;
        let is_lit = if crt_h_pos >= self.reg_x - 1 && crt_h_pos <= self.reg_x + 1 { '#' } else { '.' };
        
        let instr = &self.program[self.pc];
        self.current_cycle += 1;
        self.inter_instr_cycle += 1;
        if self.inter_instr_cycle >= instr.cycle_to_complete() {
            // Done executing current instruction
            self.pc += 1;
            self.inter_instr_cycle = 0; // Reset inter instr counter
            // Perform action of instruction
            match instr {
                Instr::NOOP => { },
                Instr::ADDX(val) => self.reg_x += val,
            }
        }

        Some(is_lit)
    }
}