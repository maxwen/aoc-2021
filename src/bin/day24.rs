
use aoc_2021::read_lines_as_vec;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
enum InstructionType {
    INP,
    ADD,
    MUL,
    DIV,
    MOD,
    EQL,
}

impl TryFrom<&str> for InstructionType {
    type Error = ();

    fn try_from(c: &str) -> Result<Self, Self::Error> {
        match c {
            "inp" => Ok(InstructionType::INP),
            "add" => Ok(InstructionType::ADD),
            "mul" => Ok(InstructionType::MUL),
            "div" => Ok(InstructionType::DIV),
            "mod" => Ok(InstructionType::MOD),
            "eql" => Ok(InstructionType::EQL),
            _ => Err(()),
        }
    }
}

impl Display for InstructionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionType::INP => write!(f, "inp"),
            InstructionType::ADD => write!(f, "add"),
            InstructionType::MUL => write!(f, "mul"),
            InstructionType::DIV => write!(f, "div"),
            InstructionType::MOD => write!(f, "mod"),
            InstructionType::EQL => write!(f, "eql"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Variable {
    W,
    X,
    Y,
    Z,
}

impl TryFrom<&str> for Variable {
    type Error = ();

    fn try_from(c: &str) -> Result<Self, Self::Error> {
        match c {
            "x" => Ok(Variable::X),
            "y" => Ok(Variable::Y),
            "z" => Ok(Variable::Z),
            "w" => Ok(Variable::W),
            _ => Err(()),
        }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Variable::W => write!(f, "w"),
            Variable::X => write!(f, "x"),
            Variable::Y => write!(f, "y"),
            Variable::Z => write!(f, "z"),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    instruction_type: InstructionType,
    variable_lhs: Variable,
    variable_rhs: Option<Variable>,
    value: Option<i64>,
}

impl Instruction {
    fn new_inp(variable_lhs: Variable) -> Self {
        Instruction {
            instruction_type: InstructionType::INP,
            variable_lhs,
            variable_rhs: None,
            value: None,
        }
    }
    fn new_add(variable_lhs: Variable, value: Option<i64>, variable_rhs: Option<Variable>) -> Self {
        Instruction {
            instruction_type: InstructionType::ADD,
            variable_lhs,
            variable_rhs,
            value,
        }
    }
    fn new_mul(variable_lhs: Variable, value: Option<i64>, variable_rhs: Option<Variable>) -> Self {
        Instruction {
            instruction_type: InstructionType::MUL,
            variable_lhs,
            variable_rhs,
            value,
        }
    }
    fn new_div(variable_lhs: Variable, value: Option<i64>, variable_rhs: Option<Variable>) -> Self {
        Instruction {
            instruction_type: InstructionType::DIV,
            variable_lhs,
            variable_rhs,
            value,
        }
    }
    fn new_mod(variable_lhs: Variable, value: Option<i64>, variable_rhs: Option<Variable>) -> Self {
        Instruction {
            instruction_type: InstructionType::MOD,
            variable_lhs,
            variable_rhs,
            value,
        }
    }
    fn new_eql(variable_lhs: Variable, value: Option<i64>, variable_rhs: Option<Variable>) -> Self {
        Instruction {
            instruction_type: InstructionType::EQL,
            variable_lhs,
            variable_rhs,
            value,
        }
    }

    fn execute(
        &self,
        storage: &mut VariableStorage,
        inp_values: &Vec<i32>,
        inp_value_index: usize,
    ) -> usize {
        let mut inp_value_index = inp_value_index;
        match self.instruction_type {
            InstructionType::INP => {
                storage.set_variable_value(&self.variable_lhs, inp_values[inp_value_index] as i64);
                inp_value_index += 1;

                // println!("{} {}", inp_value_index, storage)
            }
            InstructionType::ADD => {
                if self.value.is_some() {
                    let res = storage.get_variable_value(&self.variable_lhs) + self.value.unwrap();
                    storage.set_variable_value(&self.variable_lhs, res);
                } else {
                    let res = storage.get_variable_value(&self.variable_lhs)
                        + storage.get_variable_value(&self.variable_rhs.as_ref().unwrap());
                    storage.set_variable_value(&self.variable_lhs, res);
                }
            }
            InstructionType::MUL => {
                if self.value.is_some() {
                    let res = storage.get_variable_value(&self.variable_lhs) * self.value.unwrap();
                    storage.set_variable_value(&self.variable_lhs, res);
                } else {
                    let res = storage.get_variable_value(&self.variable_lhs)
                        * storage.get_variable_value(&self.variable_rhs.as_ref().unwrap());
                    storage.set_variable_value(&self.variable_lhs, res);
                }
            }
            InstructionType::DIV => {
                if self.value.is_some() {
                    let res = storage.get_variable_value(&self.variable_lhs) / self.value.unwrap();
                    storage.set_variable_value(&self.variable_lhs, res);
                } else {
                    let res = storage.get_variable_value(&self.variable_lhs) / storage.get_variable_value(&self.variable_rhs.as_ref().unwrap());
                    storage.set_variable_value(&self.variable_lhs, res);
                }
            }
            InstructionType::MOD => {
                if self.value.is_some() {
                    let res = storage
                        .get_variable_value(&self.variable_lhs)
                        .rem_euclid(self.value.unwrap());
                    storage.set_variable_value(&self.variable_lhs, res);
                } else {
                    let res = storage.get_variable_value(&self.variable_lhs).rem_euclid(
                        storage.get_variable_value(&self.variable_rhs.as_ref().unwrap()),
                    );
                    storage.set_variable_value(&self.variable_lhs, res);
                }
            }
            InstructionType::EQL => {
                if self.value.is_some() {
                    let res =
                        if storage.get_variable_value(&self.variable_lhs) == self.value.unwrap() {
                            1
                        } else {
                            0
                        };
                    storage.set_variable_value(&self.variable_lhs, res);
                } else {
                    let res = if storage.get_variable_value(&self.variable_lhs)
                        == storage.get_variable_value(&self.variable_rhs.as_ref().unwrap())
                    {
                        1
                    } else {
                        0
                    };
                    storage.set_variable_value(&self.variable_lhs, res);
                }
            }
        }
        inp_value_index
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.value.is_none() && self.variable_rhs.is_none() {
            return write!(f, "{} {}", self.instruction_type, self.variable_lhs);
        }
        if self.value.is_none() {
            return write!(
                f,
                "{} {} {}",
                self.instruction_type,
                self.variable_lhs,
                self.variable_rhs.as_ref().unwrap()
            );
        }
        write!(
            f,
            "{} {} {}",
            self.instruction_type,
            self.variable_lhs,
            self.value.as_ref().unwrap()
        )
    }
}

#[derive(Debug, Copy, Clone)]
struct VariableStorage {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl VariableStorage {
    fn get_variable_value(&self, variable: &Variable) -> i64 {
        match variable {
            Variable::W => self.w,
            Variable::X => self.x,
            Variable::Y => self.y,
            Variable::Z => self.z,
        }
    }

    fn set_variable_value(&mut self, variable: &Variable, value: i64) {
        match variable {
            Variable::W => self.w = value,
            Variable::X => self.x = value,
            Variable::Y => self.y = value,
            Variable::Z => self.z = value,
        }
    }
}

impl Display for VariableStorage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "w = {} x = {} y = {} z = {}",
            self.w, self.x, self.y, self.z
        )
    }
}

fn convert_number_to_inp_value(value: u64) -> Vec<i32> {
    value
        .to_string()
        .chars()
        .enumerate()
        .map(|(_, c)| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<_>>()
}
fn convert_inp_value_to_number(inp_values: &Vec<i32>) -> u64 {
    inp_values
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + (*x as u64) * 10_u64.pow(i as u32))

}

fn part1(lines: &[String]) -> u64 {
    let mut sub_programs = vec![];
    let mut program = vec![];

    for line in lines.iter() {
        if line.len() == 0 || line.starts_with("#") {
            continue;
        }
        let parts = line.split(" ").collect::<Vec<_>>();

        let instruction_type = InstructionType::try_from(parts[0]).unwrap();
        let variable_lhs = Variable::try_from(parts[1]).unwrap();
        let mut value: Option<i64> = None;
        let mut variable_rhs: Option<Variable> = None;

        if parts.len() == 3 {
            if let Ok(v) = parts[2].parse() {
                value = Some(v);
            } else {
                variable_rhs = Some(Variable::try_from(parts[2]).unwrap());
            }
        }
        let instruction = match instruction_type {
            InstructionType::INP => Instruction::new_inp(variable_lhs),
            InstructionType::ADD => Instruction::new_add(variable_lhs, value, variable_rhs),
            InstructionType::MUL => Instruction::new_mul(variable_lhs, value, variable_rhs),
            InstructionType::DIV => Instruction::new_div(variable_lhs, value, variable_rhs),
            InstructionType::MOD => Instruction::new_mod(variable_lhs, value, variable_rhs),
            InstructionType::EQL => Instruction::new_eql(variable_lhs, value, variable_rhs),
        };
        if instruction.instruction_type == InstructionType::INP {
            if !program.is_empty() {
                sub_programs.push(program);
            }
            program = vec![];
            program.push(instruction)
        } else {
            program.push(instruction)
        }
    }
    sub_programs.push(program);

    let mut storage = VariableStorage {
        w: 0,
        x: 0,
        y: 0,
        z: 0,
    };

    // 91897399498995
    let inp_values = vec![9, 1, 8, 9, 7, 3, 9, 9, 4, 9, 8, 9, 9, 5];
    println!("{}", convert_inp_value_to_number(&inp_values));
    // let mut inp_value_index = 0;
    // for p in sub_programs.iter() {
    //     for i in p.iter()
    //     {
    //         println!("{}", i);
    //         inp_value_index = i.execute(&mut storage, &inp_values, inp_value_index);
    //         println!("{}", storage);
    //     }
    // }
    // println!("end {}", storage);

    0
}

fn part2(lines: &[String]) -> usize {
    0
}

// https://github.com/MarcelRobitaille/2021-advent-of-code/blob/main/day_24/src/main.rs
// https://www.ericburden.work/blog/2022/01/05/advent-of-code-2021-day-24/

fn main() {
    let lines = read_lines_as_vec("input/input_day24.txt").unwrap();
    // let lines = read_lines_as_vec("input_test/input_test_day24.txt").unwrap();

    // let lines = vec![
    //     "inp w", "add z w", "mod z 2", "div w 2", "add y w", "mod y 2", "div w 2", "add x w",
    //     "mod x 2", "div w 2", "mod w 2",
    // ]
    // .iter()
    // .map(|s| s.to_string())
    // .collect::<Vec<_>>();
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}

