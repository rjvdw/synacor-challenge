use std::env;
use std::fs::File;
use std::io::BufReader;

use synacor_challenge::error::missing_required_argument::MissingRequiredArgument;
use synacor_challenge::{process_input, run};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut argv = env::args();
    let input_file = argv
        .nth(1)
        .ok_or_else(|| MissingRequiredArgument::new("challenge.bin"))?;
    let start_at = argv.next().unwrap_or_else(|| "0".to_string());
    let start_at = usize::from_str_radix(&start_at, 16)?;
    let reg8 = argv.next().unwrap_or_else(|| "0".to_string());
    let reg8 = u16::from_str_radix(&reg8, 16)?;

    let file = File::open(&input_file)?;
    let memory = process_input(BufReader::new(file))?;
    run(memory, start_at, reg8)?;

    Ok(())
}
