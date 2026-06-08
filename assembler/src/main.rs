mod ast;
mod compiler;
mod lexer;
mod parser;

use std::path::Path;

use clap::Parser;
use compiler::compile;
use lexer::lex_source;
use parser::build_ast;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// source code path
    source: String,

    /// compiled binary output directory path
    #[arg(short, long, default_value = "output/")]
    output_dir: String,
}

fn main() {
    let args = Args::parse();
    let source = std::fs::read_to_string(args.source).expect("Failed to read program");

    let output_path = Path::new(&args.output_dir).join("bin.ch8");

    let tokens = lex_source(&source);
    let instructions = build_ast(tokens);
    compile(instructions, output_path);
}
