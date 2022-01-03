use std::io;
use std::io::{BufReader, Read};

use crate::error::empty_stack::EmptyStack;
use crate::error::invalid_op_code::InvalidOpCode;
use crate::error::invalid_register::InvalidRegister;

pub mod error;

pub fn run(mut memory: Vec<u16>) -> Result<(), Box<dyn std::error::Error>> {
    let mut input_buffer = String::new();
    let mut input_chars = vec![];
    let stdin = io::stdin();

    let mut program_counter = 0;
    let mut register = [0; 8];
    let mut stack = vec![];

    loop {
        let op_code = memory[program_counter];
        match op_code {
            0 => {
                // halt
                return Ok(());
            }
            1 => {
                // set a b
                let a = memory[program_counter + 1];
                let a = reg(a)?;
                let b = memory[program_counter + 2];
                let b = val(b, &mut register);
                register[a] = b;
                program_counter += 3;
            }
            2 => {
                // push a
                let a = memory[program_counter + 1];
                let a = val(a, &mut register);
                stack.push(a);
                program_counter += 2;
            }
            3 => {
                // pop a
                match stack.pop() {
                    Some(v) => {
                        let a = memory[program_counter + 1];
                        let a = reg(a)?;
                        register[a] = v;
                        program_counter += 2;
                    }
                    None => {
                        return Err(EmptyStack.into());
                    }
                }
            }
            4 => {
                // eq a b c
                let a = memory[program_counter + 1];
                let a = reg(a)?;
                let b = memory[program_counter + 2];
                let b = val(b, &mut register);
                let c = memory[program_counter + 3];
                let c = val(c, &mut register);
                register[a] = if b == c { 1 } else { 0 };
                program_counter += 4;
            }
            5 => {
                // gt a b c
                let a = memory[program_counter + 1];
                let a = reg(a)?;
                let b = memory[program_counter + 2];
                let b = val(b, &mut register);
                let c = memory[program_counter + 3];
                let c = val(c, &mut register);
                register[a] = if b > c { 1 } else { 0 };
                program_counter += 4;
            }
            6 => {
                // jmp a
                let a = memory[program_counter + 1];
                let a = val(a, &mut register);
                program_counter = a as usize;
            }
            7 => {
                // jt a b
                let a = memory[program_counter + 1];
                let a = val(a, &mut register);
                let b = memory[program_counter + 2];
                let b = val(b, &mut register);
                if a != 0 {
                    program_counter = b as usize;
                } else {
                    program_counter += 3;
                }
            }
            8 => {
                // jf a b
                let a = memory[program_counter + 1];
                let a = val(a, &mut register);
                let b = memory[program_counter + 2];
                let b = val(b, &mut register);
                if a == 0 {
                    program_counter = b as usize;
                } else {
                    program_counter += 3;
                }
            }
            9 => {
                // add a b c
                let a = memory[program_counter + 1];
                let a = reg(a)?;
                let b = memory[program_counter + 2];
                let b = val(b, &mut register);
                let c = memory[program_counter + 3];
                let c = val(c, &mut register);
                register[a] = (b + c) % 32768;
                program_counter += 4;
            }
            10 => {
                // mult a b c
                let a = memory[program_counter + 1];
                let a = reg(a)?;
                let b = memory[program_counter + 2];
                let b = val(b, &mut register);
                let c = memory[program_counter + 3];
                let c = val(c, &mut register);
                register[a] = ((b as u64 * c as u64) % 32768) as u16;
                program_counter += 4;
            }
            11 => {
                // mod a b c
                let a = memory[program_counter + 1];
                let a = reg(a)?;
                let b = memory[program_counter + 2];
                let b = val(b, &mut register);
                let c = memory[program_counter + 3];
                let c = val(c, &mut register);
                register[a] = b % c;
                program_counter += 4;
            }
            12 => {
                // and a b c
                let a = memory[program_counter + 1];
                let a = reg(a)?;
                let b = memory[program_counter + 2];
                let b = val(b, &mut register);
                let c = memory[program_counter + 3];
                let c = val(c, &mut register);
                register[a] = b & c;
                program_counter += 4;
            }
            13 => {
                // or a b c
                let a = memory[program_counter + 1];
                let a = reg(a)?;
                let b = memory[program_counter + 2];
                let b = val(b, &mut register);
                let c = memory[program_counter + 3];
                let c = val(c, &mut register);
                register[a] = b | c;
                program_counter += 4;
            }
            14 => {
                // not a b
                let a = memory[program_counter + 1];
                let a = reg(a)?;
                let b = memory[program_counter + 2];
                let b = val(b, &mut register);
                register[a] = b ^ 0b111_1111_1111_1111;
                program_counter += 3;
            }
            15 => {
                // rmem a b
                let a = memory[program_counter + 1];
                let a = reg(a)?;
                let b = memory[program_counter + 2];
                let b = val(b, &mut register);
                register[a] = memory[b as usize];
                program_counter += 3;
            }
            16 => {
                // wmem a b
                let a = memory[program_counter + 1];
                let a = val(a, &mut register);
                let b = memory[program_counter + 2];
                let b = val(b, &mut register);
                memory[a as usize] = b;
                program_counter += 3;
            }
            17 => {
                // call a
                let a = memory[program_counter + 1];
                let a = val(a, &mut register);
                stack.push(program_counter as u16 + 2);
                program_counter = a as usize;
            }
            18 => {
                // ret
                match stack.pop() {
                    Some(a) => {
                        program_counter = a as usize;
                    }
                    None => {
                        return Ok(());
                    }
                }
            }
            19 => {
                // out a
                let a = memory[program_counter + 1];
                let a = val(a, &mut register);
                print!("{}", (a as u8) as char);
                program_counter += 2;
            }
            20 => {
                // in a
                if input_chars.is_empty() {
                    stdin.read_line(&mut input_buffer)?;
                    for ch in input_buffer.chars().rev() {
                        input_chars.push(ch);
                    }
                }

                let a = memory[program_counter + 1];
                let a = reg(a)?;
                let v = input_chars.pop().unwrap();
                register[a] = (v as u8) as u16;
                program_counter += 2;
            }
            21 => {
                // noop
                program_counter += 1;
            }
            _ => {
                return Err(InvalidOpCode::new(op_code).into());
            }
        }
    }
}

fn reg(number: u16) -> Result<usize, InvalidRegister> {
    if (32768..=32775).contains(&number) {
        Ok((number - 32768) as usize)
    } else {
        Err(InvalidRegister::new(number))
    }
}

fn val(number: u16, register: &mut [u16; 8]) -> u16 {
    if (32768..=32775).contains(&number) {
        register[(number - 32768) as usize]
    } else {
        number
    }
}

pub fn print(memory: Vec<u16>) -> Result<(), Box<dyn std::error::Error>> {
    let mut memory: Vec<(usize, u16)> = memory.iter().copied().enumerate().rev().collect();
    while let Some((idx, nr)) = memory.pop() {
        print!("{:6}:  ", idx);
        match nr {
            0 => println!("halt"),
            1 => println!(
                "set {} {}",
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
            ),
            2 => println!("push {}", print_val(memory.pop().unwrap().1),),
            3 => println!("pop {}", print_val(memory.pop().unwrap().1),),
            4 => println!(
                "eq {} {} {}",
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
            ),
            5 => println!(
                "gt {} {} {}",
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
            ),
            6 => println!("jmp {}", print_val(memory.pop().unwrap().1),),
            7 => println!(
                "jt {} {}",
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
            ),
            8 => println!(
                "jf {} {}",
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
            ),
            9 => println!(
                "add {} {} {}",
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
            ),
            10 => println!(
                "mult {} {} {}",
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
            ),
            11 => println!(
                "mod {} {} {}",
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
            ),
            12 => println!(
                "and {} {} {}",
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
            ),
            13 => println!(
                "or {} {} {}",
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
            ),
            14 => println!(
                "not {} {}",
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
            ),
            15 => println!(
                "rmem {} {}",
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
            ),
            16 => println!(
                "wmem {} {}",
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
            ),
            17 => println!("call {}", print_val(memory.pop().unwrap().1),),
            18 => println!("ret"),
            19 => println!("out {}", print_val(memory.pop().unwrap().1),),
            20 => println!("in {}", print_val(memory.pop().unwrap().1),),
            21 => println!("noop"),
            _ => println!("{}", print_val(nr)),
        }
    }
    Ok(())
}

fn print_val(nr: u16) -> String {
    match nr {
        0..=32767 => format!("{}", nr),
        32768 => "a".to_string(),
        32769 => "b".to_string(),
        32770 => "c".to_string(),
        32771 => "d".to_string(),
        32772 => "e".to_string(),
        32773 => "f".to_string(),
        32774 => "g".to_string(),
        32775 => "h".to_string(),
        _ => panic!("Invalid value: {}", nr),
    }
}

pub fn process_input<R: Read>(
    mut reader: BufReader<R>,
) -> Result<Vec<u16>, Box<dyn std::error::Error>> {
    let mut buffer = [0; 4096];
    let mut numbers = vec![];
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            return Ok(numbers);
        }
        for i in (0..bytes_read).step_by(2) {
            numbers.push(u16::from_le_bytes([buffer[i], buffer[i + 1]]));
        }
    }
}
