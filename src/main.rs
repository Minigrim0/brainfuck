use clap::Parser;

mod cli;
mod program;

use cli::Arguments;
use program::Program;

fn main() {
    let args = Arguments::parse();
    let mut program = Program::load(args.file);
    let _ = program.run();
    println!();
}
