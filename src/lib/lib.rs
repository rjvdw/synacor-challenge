use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::{env, fs, io};

use chrono::Utc;

use crate::error::empty_stack::EmptyStack;
use crate::error::invalid_op_code::InvalidOpCode;
use crate::error::invalid_register::InvalidRegister;

pub mod error;

pub fn run(mut memory: Vec<u16>) -> Result<(), Box<dyn std::error::Error>> {
    let mut input_buffer = String::new();
    let mut input_chars = vec![];
    let stdin = io::stdin();

    let mut seen = HashSet::new();
    let mut program_counter = 0;
    let mut register = [0; 8];
    let mut stack = vec![];
    let mut p = register[7];

    'program_loop: loop {
        if p != register[7] {
            println!("[debug] 8th register has value {}", register[7]);
            p = register[7];
        }
        let op_code = memory[program_counter];
        seen.insert(program_counter);
        match op_code {
            0x0000 => {
                // halt
                break;
            }
            0x0001 => {
                // set a b
                let a = memory[program_counter + 1];
                let a = reg(a)?;
                let b = memory[program_counter + 2];
                let b = val(b, &mut register);
                register[a] = b;
                program_counter += 3;
            }
            0x0002 => {
                // push a
                let a = memory[program_counter + 1];
                let a = val(a, &mut register);
                stack.push(a);
                program_counter += 2;
            }
            0x0003 => {
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
            0x0004 => {
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
            0x0005 => {
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
            0x0006 => {
                // jmp a
                let a = memory[program_counter + 1];
                let a = val(a, &mut register);
                program_counter = a as usize;
            }
            0x0007 => {
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
            0x0008 => {
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
            0x0009 => {
                // add a b c
                let a = memory[program_counter + 1];
                let a = reg(a)?;
                let b = memory[program_counter + 2];
                let b = val(b, &mut register);
                let c = memory[program_counter + 3];
                let c = val(c, &mut register);
                register[a] = (b + c) % 0x8000;
                program_counter += 4;
            }
            0x0a => {
                // mult a b c
                let a = memory[program_counter + 1];
                let a = reg(a)?;
                let b = memory[program_counter + 2];
                let b = val(b, &mut register);
                let c = memory[program_counter + 3];
                let c = val(c, &mut register);
                register[a] = ((b as u64 * c as u64) % 0x8000) as u16;
                program_counter += 4;
            }
            0x0b => {
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
            0x0c => {
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
            0x0d => {
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
            0x0e => {
                // not a b
                let a = memory[program_counter + 1];
                let a = reg(a)?;
                let b = memory[program_counter + 2];
                let b = val(b, &mut register);
                register[a] = b ^ 0x7fff;
                program_counter += 3;
            }
            0x0f => {
                // rmem a b
                let a = memory[program_counter + 1];
                let a = reg(a)?;
                let b = memory[program_counter + 2];
                let b = val(b, &mut register);
                register[a] = memory[b as usize];
                program_counter += 3;
            }
            0x0010 => {
                // wmem a b
                let a = memory[program_counter + 1];
                let a = val(a, &mut register);
                let b = memory[program_counter + 2];
                let b = val(b, &mut register);
                memory[a as usize] = b;
                program_counter += 3;
            }
            0x0011 => {
                // call a
                let a = memory[program_counter + 1];
                let a = val(a, &mut register);
                stack.push(program_counter as u16 + 2);
                program_counter = a as usize;
            }
            0x0012 => {
                // ret
                match stack.pop() {
                    Some(a) => {
                        program_counter = a as usize;
                    }
                    None => {
                        break;
                    }
                }
            }
            0x0013 => {
                // out a
                let a = memory[program_counter + 1];
                let a = val(a, &mut register);
                print!("{}", (a as u8) as char);
                program_counter += 2;
            }
            0x0014 => {
                // in a
                while input_chars.is_empty() {
                    stdin.read_line(&mut input_buffer)?;
                    println!();
                    if input_buffer.is_empty() {
                        println!("[debug] empty input received, user probably pressed ^D, halting");
                        break 'program_loop;
                    } else if input_buffer == *"MEM DUMP\n" {
                        dump_memory(&memory, &register)?;
                    } else if input_buffer == *"HALT\n" {
                        break 'program_loop;
                    } else {
                        if input_buffer == *"help\n" {
                            println!("HALT");
                            println!("  [debug] Immediately stops the execution of the program.");
                            println!("MEM DUMP");
                            println!("  [debug] Writes a dump of the memory and the registers to the disk.");
                        }
                        for ch in input_buffer.chars().rev() {
                            input_chars.push(ch);
                        }
                    }
                    input_buffer = String::new();
                }

                let a = memory[program_counter + 1];
                let a = reg(a)?;
                let v = input_chars.pop().unwrap();
                register[a] = (v as u8) as u16;
                program_counter += 2;
            }
            0x0015 => {
                // noop
                program_counter += 1;
            }
            _ => {
                return Err(InvalidOpCode::new(op_code).into());
            }
        }
    }

    println!("[debug] a total of {} instructions were seen", seen.len());

    Ok(())
}

fn reg(number: u16) -> Result<usize, InvalidRegister> {
    if (0x8000..0x8008).contains(&number) {
        Ok((number - 0x8000) as usize)
    } else {
        Err(InvalidRegister::new(number))
    }
}

fn val(number: u16, register: &mut [u16; 8]) -> u16 {
    if (0x8000..0x8008).contains(&number) {
        register[(number - 0x8000) as usize]
    } else {
        number
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

fn dump_memory(memory: &[u16], register: &[u16]) -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now();
    let mut tmp_dir = env::temp_dir();
    tmp_dir.push("synacor-challenge-memory-dumps");

    let mut mem_dump_file = tmp_dir.clone();
    mem_dump_file.push(format!("memory-{:?}.txt", now));
    let mem_dump_file = mem_dump_file.to_str().unwrap();

    let mut register_dump_file = tmp_dir.clone();
    register_dump_file.push(format!("registers-{:?}.txt", now));
    let register_dump_file = register_dump_file.to_str().unwrap();

    if !tmp_dir.exists() {
        fs::create_dir(tmp_dir)?;
    }

    let mut file = File::create(mem_dump_file)?;
    for number in memory {
        file.write_all(format!("{:04x}\n", number).as_bytes())?;
    }
    println!("[debug] memory dump was written to {}", mem_dump_file);

    let mut file = File::create(register_dump_file)?;
    for number in register {
        file.write_all(format!("{:04x}\n", number).as_bytes())?;
    }
    println!(
        "[debug] register dump was written to {}",
        register_dump_file
    );
    Ok(())
}
