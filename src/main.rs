use std::io::{self, Read};

#[derive(Debug)]
enum Operation {
    IncrementPtr,
    DecrementPtr,
    IncrementData,
    DecrementData,
    PrintData,
    StartLoop(usize),
    EndLoop(usize),
}

struct Program {
    operations: Vec<Operation>,
    pointer: usize,
    memory: Vec<usize>,
}

impl Program {
    pub fn load_it_up(input: &str) -> Self {
        let mut operations: Vec<Operation> = Vec::new();

        let mut start_loop_positions: Vec<usize> = vec![];

        for c in input.chars() {
            let op = match c {
                '>' => Operation::IncrementPtr,
                '<' => Operation::DecrementPtr,
                '+' => Operation::IncrementData,
                '-' => Operation::DecrementData,
                '.' => Operation::PrintData,
                '[' => {
                    // push what will be the position of the start of a loop
                    start_loop_positions.push(operations.len());

                    // push placeholder value
                    Operation::StartLoop(0)
                }
                ']' => {
                    // pop start position out
                    let start_position = start_loop_positions.pop().unwrap();

                    // fix up StartLoop so that it contains the ending loop bracket
                    operations[start_position] = Operation::StartLoop(operations.len());

                    // return EndLoop with the start position
                    Operation::EndLoop(start_position)
                }
                _ => {
                    continue;
                }
            };

            operations.push(op);
        }

        Self {
            pointer: 0,
            operations,
            memory: Vec::with_capacity(30000),
        }
    }

    pub fn run(&mut self) {
        for (i, op) in self.operations.iter().enumerate() {
            println!("{:?}, {:?}", i, op);
        }

        self.memory.push(0);

        let mut instruction_pointer = 0;

        while instruction_pointer < self.operations.len() {
            match self.operations[instruction_pointer] {
                Operation::IncrementPtr => {
                    self.pointer += 1;

                    if self.pointer == self.memory.len() {
                        self.memory.push(0);
                    }
                }
                Operation::DecrementPtr => self.pointer -= 1,
                Operation::IncrementData => {
                    println!(
                        "doing it data: {:?}, location: {:?}",
                        self.memory[self.pointer], self.pointer
                    );
                    self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1);
                }
                Operation::DecrementData => {
                    self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1);
                }
                Operation::PrintData => {
                    print!("{}", self.memory[self.pointer] as u8 as char);
                }
                Operation::StartLoop(end_index) => {
                    if self.memory[self.pointer] == 0 {
                        instruction_pointer = end_index;
                        continue;
                    }
                }
                Operation::EndLoop(start_index) => {
                    if self.memory[self.pointer] != 0 {
                        instruction_pointer = start_index;
                        continue;
                    }
                }
            }

            instruction_pointer += 1;
        }
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();

    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;

    let mut prog = Program::load_it_up(&buffer);
    prog.run();

    Ok(())
}
