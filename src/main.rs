#[allow(unused)]
mod input;

use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// What mode to run this program in
    #[arg(value_enum)]
    run_mode: RunMode,

    /// The Brainfuck file to compile
    file: PathBuf,

    /// Name of the output file
    #[arg(short, long)]
    #[arg(default_value_t = String::from("out"))]
    out_file: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum RunMode {
    /// Compile the program and run it
    Run,
    /// Compile the program
    Build,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    transpile(&args.file)?;
    // compile
    if let RunMode::Run = args.run_mode {
        // run
    }
    Ok(())
}

fn transpile(source: &Path) -> Result<(), Box<dyn Error>> {
    let mut output = File::create("out.rs")?;
    write!(
        output,
        r#"#![allow(unused_mut)]
{input}
fn main() {{
    let mut tape = [std::num::Wrapping(0u8); 30_000];
    let mut pointer = 0usize;
    #[allow(unused)]
    let mut input = Input::new();
"#,
        input = include_str!("input.rs"),
    )?;

    let program = fs::read_to_string(source)?;
    for char in program.chars() {
        match char {
            '>' => writeln!(output, r#"    pointer += 1;"#),
            '<' => writeln!(output, r#"    pointer -= 1;"#),
            '+' => writeln!(output, r#"    tape[pointer] += 1;"#),
            '-' => writeln!(output, r#"    tape[pointer] -= 1;"#),
            '[' => writeln!(output, r#"    while tape[pointer].0 != 0 {{"#),
            ']' => writeln!(output, r#"    }}"#),
            '.' => writeln!(output, r#"    print!("{{}}", tape[pointer].0 as char);"#),
            ',' => writeln!(output, r#"    tape[pointer] = input.read();"#),
            '#' => writeln!(output, r#"    print!("{{}}", tape[pointer]);"#),
            x => writeln!(output, "// unimplemented: {x}"),
        }?
    }

    writeln!(output, "}}")?;
    Ok(())
}
