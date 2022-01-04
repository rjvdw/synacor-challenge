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
            println!("{:6}:  {}", idx, nr);
        }
    }

    Ok(())
}

fn print(memory: Vec<u16>) -> Result<(), Box<dyn std::error::Error>> {
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
            2 => println!("push {}", print_val(memory.pop().unwrap().1)),
            3 => println!("pop {}", print_val(memory.pop().unwrap().1)),
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
            6 => println!("jmp {}", print_val(memory.pop().unwrap().1)),
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
            17 => println!("call {}", print_val(memory.pop().unwrap().1)),
            18 => println!("ret"),
            19 => {
                let mut output = String::new();
                loop {
                    match memory.pop().unwrap().1 {
                        10 => {
                            output.push('\n');
                            break;
                        }
                        32768 => output.push_str("<a>"),
                        32769 => output.push_str("<b>"),
                        32770 => output.push_str("<c>"),
                        32771 => output.push_str("<d>"),
                        32772 => output.push_str("<e>"),
                        32773 => output.push_str("<f>"),
                        32774 => output.push_str("<g>"),
                        32775 => output.push_str("<h>"),
                        v => output.push((v as u8) as char),
                    }
                    if let Some((_, v)) = memory.last() {
                        if *v == 19 {
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
            20 => println!("in {}", print_val(memory.pop().unwrap().1)),
            21 => println!("noop"),
            _ => println!("{}", print_val(nr)),
        }
    }
    Ok(())
}

fn print_val(nr: u16) -> String {
    match nr {
        0..=32767 => format!("{}", nr),
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
