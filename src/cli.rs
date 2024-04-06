use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[clap(author="Minigrim0", version, about="A brainfuck interpreter written in rust")]
pub struct Arguments {
    /// The input file to read
    #[arg(short, long)]
    pub file: String,

    /// Interpret interactively, allowing to inspect memory & instruction pointer
    #[arg(short, long)]
    pub interactive: bool,

    /// The output file (to write the output to)
    #[arg(short, long, default_value_t = String::from("stdout"))]
    pub output: String,
}