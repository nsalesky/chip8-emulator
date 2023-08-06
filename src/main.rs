use std::{fs::File, path::Path, process};

use anyhow::Result;
use chip8::run;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename for the ROM file to load
    #[arg()]
    rom_file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let rom_path = Path::new(&args.rom_file);

    let file = match File::open(rom_path) {
        Err(why) => {
            eprintln!("Couldn't open {}: {}", rom_path.display(), why);
            process::exit(1);
        }
        Ok(file) => file,
    };

    run(file)?;
    Ok(())
}
