use std::collections::HashMap;

pub enum InstructionError {
    RegisterNotExist,
}

pub enum Argument {
    Register(char),
    Value(i32),
}

pub struct Instruction {
    reg: char,
    op: Operation,
}

impl Instruction {
    fn new(register: char, operation: Operation) -> Instruction {
        Instruction {
            reg: register,
            op: operation,
        }
    }
}

pub enum Operation {
    Inp(),
    Add(Argument),
    Mul(Argument),
    Div(Argument),
    Mod(Argument),
    Eql(Argument),
}

pub struct Monad {
    registers: HashMap<char, i32>,
    input_nums: Vec<i32>,
    i: usize,
}

impl Monad {
    fn new(nums: Vec<i32>) -> Monad {
        let mut registers: HashMap<char, i32> = HashMap::new();
        registers.insert('w', 0);
        registers.insert('x', 0);
        registers.insert('y', 0);
        registers.insert('z', 0);
        Monad {
            registers: registers,
            input_nums: nums,
            i: 0,
        }
    }

    fn get_value(&self, argument: &Argument) -> i32 {
        match argument {
            Argument::Value(value) => *value,
            Argument::Register(reg) => *self.registers.get(reg).unwrap(),
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        let val = match instruction.op {
            Operation::Inp() => {
                let val = self.input_nums[self.i];
                self.i += 1;
                val
            },
            Operation::Add(arg) => self.registers[&instruction.reg] + self.get_value(&arg),
            Operation::Div(arg) => self.registers[&instruction.reg] / self.get_value(&arg),
            Operation::Eql(arg) => {
                let is_equal: bool = self.registers[&instruction.reg] == self.get_value(&arg);
                is_equal as i32
            }
            Operation::Mod(arg) => self.registers[&instruction.reg] % self.get_value(&arg),
            Operation::Mul(arg) => self.registers[&instruction.reg] % self.get_value(&arg),
        };
        self.registers.insert(instruction.reg, val);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_inp_instruction() {
        let mut vm = Monad::new(vec![1, 2, 3]);
        vm.execute(Instruction::new('z', Operation::Inp()));
        assert_eq!(vm.registers[&'z'], 1)
    }

    #[test]
    fn test_inp_instruction_many_times() {
        let mut vm = Monad::new(vec![1, 2, 3]);
        vm.execute(Instruction::new('z', Operation::Inp()));
        vm.execute(Instruction::new('z', Operation::Inp()));
        vm.execute(Instruction::new('w', Operation::Inp()));
        assert_eq!(vm.registers[&'w'], 3);
        assert_eq!(vm.registers[&'z'], 2);
    }
}
