use std::{fs::File, io};
use std::io::BufRead;
use std::io::Read;

pub enum Instructions {  /// From wikipedia
    MR, // > 	Increment the data pointer by one (to point to the next cell to the right).
    ML, // < 	Decrement the data pointer by one (to point to the next cell to the left).
    IP, // + 	Increment the byte at the data pointer by one.
    DP, // - 	Decrement the byte at the data pointer by one.
    OP, // . 	Output the byte at the data pointer.
    RP, // , 	Accept one byte of input, storing its value in the byte at the data pointer.
    JF, // [ 	If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.
    JB, // ] 	If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.[a]
}

pub struct Program {
    pub instructions: Vec<Instructions>,
    memory: [u8; 30000],
    dp: i32,
    pc: i32,
    output: String,
    counter: i32,
}

impl Program {
    pub fn load(path: String) -> Program {
        let file = match File::open(&path){
            Ok(f) => f,
            Err(e) => {
                panic!("Error while reading file {}: {}", path, e.to_string());
            }
        };

        let mut instructions = Vec::new();
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            for ch in line.expect("Unable to read line").chars() {
                match ch {
                    '>' => instructions.push(Instructions::MR),
                    '<' => instructions.push(Instructions::ML),
                    '+' => instructions.push(Instructions::IP),
                    '-' => instructions.push(Instructions::DP),
                    '.' => instructions.push(Instructions::OP),
                    ',' => instructions.push(Instructions::RP),
                    '[' => instructions.push(Instructions::JF),
                    ']' => instructions.push(Instructions::JB),
                    _ => continue
                }
            }
        }

        Program {
            instructions,
            memory: [0; 30000],
            dp: 0,
            pc: 0,
            output: String::new(),
            counter: 0
        }
    }

    pub fn jump_forward(&mut self) -> i32 {
        let mut val = 1;
        let mut index = self.pc;
        while val > 0 {
            index += 1;
            if let Some(inst) = self.instructions.get(index as usize) {
                match inst {
                    Instructions::JF => val += 1,
                    Instructions::JB => val -= 1,
                    _ => continue
                }
            }
        }
        index + 1
    }

    pub fn jump_backwards(&mut self) -> i32 {
        let mut val = 1;
        let mut index = self.pc;
        while val > 0 {
            index -= 1;
            if let Some(inst) = self.instructions.get(index as usize) {
                match inst {
                    Instructions::JF => val -= 1,
                    Instructions::JB => val += 1,
                    _ => continue
                }
            }
        }

        index + 1
    }

    pub fn step(&mut self) -> Result<(), String> {
        self.counter += 1;
        let instruction = self.instructions.get(self.pc as usize);
        if let Some(inst) = instruction {

            self.pc = match inst {
                Instructions::MR => {
                    self.dp += 1;
                    self.pc + 1
                },
                Instructions::ML => {
                    self.dp -= 1;
                    self.pc + 1
                },
                Instructions::IP => {
                    self.memory[self.dp as usize] = (self.memory[self.dp as usize] as u16 + 1) as u8;
                    self.pc + 1
                },
                Instructions::DP => {
                    match self.memory[self.dp as usize].checked_sub(1) {
                        Some(val) => self.memory[self.dp as usize] = val,
                        None => self.memory[self.dp as usize] = 255
                    }
                    self.pc + 1
                },
                Instructions::OP => {
                    self.output = format!("{}{}", self.output, self.memory[self.dp as usize] as char);
                    // TODO: Check if running interactively or not
                    print!("{}", self.memory[self.dp as usize] as char);
                    self.pc + 1
                },
                Instructions::RP => {
                    if let Some(input) = std::io::stdin()
                        .bytes()
                        .next()
                        .and_then(|result| result.ok()) {
                            self.memory[self.dp as usize] = input;
                    }
                    self.pc + 1
                },
                Instructions::JF => {
                    if self.memory[self.dp as usize] == 0 {
                        self.jump_forward()
                    } else {
                        self.pc + 1
                    }
                },
                Instructions::JB => {
                    if self.memory[self.dp as usize] != 0 {
                        self.jump_backwards()
                    } else {
                        self.pc + 1
                    }
                },
            };

            Ok(())
        } else {
            Err("No more instruction left".to_string())
        }

    }

    pub fn run(&mut self) -> Result<(), String> {
        loop {
            if let Err(e) = self.step() {
                break Err(e);
            }
            if self.pc as usize >= self.instructions.len() {
                println!("Interpreted {} instruction(s)", self.counter);
                break Ok(());
            }
        }
    }
}