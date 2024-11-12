use std::num::Wrapping;

#[derive(Clone)]
pub enum Instruction {
    Add { value: Wrapping<i8> },
    Move { value: i32 },
    Loop { nodes: Vec<Instruction> },
    Write,
    Read,

    // Optimization nodes
    Set { value: u8 },
}

pub fn parse(str: &str) -> Result<Vec<Instruction>, &str> {
    let mut nodes = Vec::new();
    let mut stack = Vec::new();

    for char in str.chars() {
        match char {
            '+' => {
                nodes.push(Instruction::Add { value: Wrapping(1) });
            }
            '-' => {
                nodes.push(Instruction::Add {
                    value: Wrapping(-1),
                });
            }
            '>' => {
                nodes.push(Instruction::Move { value: 1 });
            }
            '<' => {
                nodes.push(Instruction::Move { value: -1 });
            }
            '.' => {
                nodes.push(Instruction::Write);
            }
            ',' => {
                nodes.push(Instruction::Read);
            }
            '[' => {
                stack.push(nodes);
                nodes = vec![]
            }
            ']' => {
                match stack.pop() {
                    Some(mut parent) => {
                        parent.push(Instruction::Loop { nodes });
                        nodes = parent;
                    }
                    None => return Err("No matching ["),
                };
            }
            _ => {
                // Comment
            }
        }
    }

    Ok(nodes)
}

pub fn indented(instrs: &Vec<Instruction>, indent: usize) -> String {
    let mut result = String::new();

    for instr in instrs {
        result.push_str(&" ".repeat(indent));
        match instr {
            Instruction::Add { value } => {
                result.push_str(&format!("Add {}\n", value.0));
            }
            Instruction::Move { value } => {
                result.push_str(&format!("Move {}\n", value));
            }
            Instruction::Loop { nodes } => {
                result.push_str("Loop\n");
                result.push_str(&indented(nodes, indent + 4));
            }
            Instruction::Write => {
                result.push_str("Write\n");
            }
            Instruction::Read => {
                result.push_str("Read\n");
            }
            Instruction::Set { value } => {
                result.push_str(&format!("Set {}\n", value));
            }
        }
    }

    result
}
