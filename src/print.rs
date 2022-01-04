use std::env;
use std::fs::File;
use std::io::BufReader;

use synacor_challenge::error::missing_required_argument::MissingRequiredArgument;
use synacor_challenge::process_input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut argv = env::args();
    let input_file = argv
        .nth(1)
        .ok_or_else(|| MissingRequiredArgument::new("challenge.bin"))?;
    let pretty = argv.next().unwrap_or_else(|| "".to_string()) == *"pretty";

    let file = File::open(&input_file)?;
    let memory = process_input(BufReader::new(file))?;

    if pretty {
        print(memory)?;
    } else {
        for (idx, nr) in memory.iter().enumerate() {
            println!("{:04x}:  {:04x}", idx, nr);
        }
    }

    Ok(())
}

fn print(memory: Vec<u16>) -> Result<(), Box<dyn std::error::Error>> {
    let mut memory: Vec<(usize, u16)> = memory.iter().copied().enumerate().rev().collect();
    while let Some((idx, nr)) = memory.pop() {
        print!("{:04x}:  ", idx);
        match nr {
            0x0000 => println!("halt"),
            0x0001 => println!(
                "set {} {}",
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
            ),
            0x0002 => println!("push {}", print_val(memory.pop().unwrap().1)),
            0x0003 => println!("pop {}", print_val(memory.pop().unwrap().1)),
            0x0004 => println!(
                "eq {} {} {}",
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
            ),
            0x0005 => println!(
                "gt {} {} {}",
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
            ),
            0x0006 => println!("jmp {}", print_val(memory.pop().unwrap().1)),
            0x0007 => println!(
                "jt {} {}",
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
            ),
            0x0008 => println!(
                "jf {} {}",
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
            ),
            0x0009 => println!(
                "add {} {} {}",
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
            ),
            0x000a => println!(
                "mult {} {} {}",
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
            ),
            0x000b => println!(
                "mod {} {} {}",
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
            ),
            0x000c => println!(
                "and {} {} {}",
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
            ),
            0x000d => println!(
                "or {} {} {}",
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
            ),
            0x000e => println!(
                "not {} {}",
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
            ),
            0x000f => println!(
                "rmem {} {}",
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
            ),
            0x0010 => println!(
                "wmem {} {}",
                print_val(memory.pop().unwrap().1),
                print_val(memory.pop().unwrap().1),
            ),
            0x0011 => println!("call {}", print_val(memory.pop().unwrap().1)),
            0x0012 => println!("ret"),
            0x0013 => {
                let mut output = String::new();
                loop {
                    match memory.pop().unwrap().1 {
                        0x000a => {
                            output.push('\n');
                            break;
                        }
                        0x8000 => output.push_str("<a>"),
                        0x8001 => output.push_str("<b>"),
                        0x8002 => output.push_str("<c>"),
                        0x8003 => output.push_str("<d>"),
                        0x8004 => output.push_str("<e>"),
                        0x8005 => output.push_str("<f>"),
                        0x8006 => output.push_str("<g>"),
                        0x8007 => output.push_str("<h>"),
                        v => output.push((v as u8) as char),
                    }
                    if let Some((_, v)) = memory.last() {
                        if *v == 0x0013 {
                            memory.pop();
                        } else {
                            break;
                        }
                    }
                }
                if !output.is_empty() {
                    println!("out \"{}\"", output.replace("\n", "\\n"));
                }
            }
            0x0014 => println!("in {}", print_val(memory.pop().unwrap().1)),
            0x0015 => println!("noop"),
            _ => println!("{}", print_val(nr)),
        }
    }
    Ok(())
}

fn print_val(nr: u16) -> String {
    match nr {
        0..=32767 => format!("{:04x}", nr),
        32768 => "<a>".to_string(),
        32769 => "<b>".to_string(),
        32770 => "<c>".to_string(),
        32771 => "<d>".to_string(),
        32772 => "<e>".to_string(),
        32773 => "<f>".to_string(),
        32774 => "<g>".to_string(),
        32775 => "<h>".to_string(),
        _ => panic!("Invalid value: {}", nr),
    }
}
