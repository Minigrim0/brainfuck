use clap::Parser;

mod cli;
mod program;

use cli::Arguments;
use program::Program;

fn main() {
    let args = Arguments::parse();
    // Check if the user wants to run the program interactively
    let mut program = Program::load(args.file);
    if args.interactive {
        let _ = program.run_interactive();
        return;
    } else  {
        let _ = program.run();
    }
}
