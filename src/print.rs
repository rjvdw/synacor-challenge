use std::env;
use std::fs::File;
use std::io::BufReader;

use synacor_challenge::error::missing_required_argument::MissingRequiredArgument;
use synacor_challenge::{print, process_input};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = env::args()
        .nth(1)
        .ok_or_else(|| MissingRequiredArgument::new("challenge.bin"))?;

    let file = File::open(&input_file)?;
    let memory = process_input(BufReader::new(file))?;
    print(memory)?;

    Ok(())
}
