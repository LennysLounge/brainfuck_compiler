#[allow(unused)]
mod input;

use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use clap::{builder::OsStr, Parser, ValueEnum};

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
    #[arg(default_value_os_t = PathBuf::from("out.exe"))]
    out_file: PathBuf,
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

    let rs_file = transpile(&args.file, &args.out_file)?;
    compile(&rs_file, &args.out_file)?;
    if let RunMode::Run = args.run_mode {
        run(&args.out_file);
    }
    Ok(())
}

fn transpile(source: &Path, dest: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let rs_file =
        PathBuf::from(dest.file_stem().unwrap_or(&OsStr::from("out"))).with_extension("rs");
    let mut output = File::create(&rs_file)?;
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
    Ok(rs_file)
}

fn compile(rs_file: &Path, dest: &Path) -> Result<(), Box<dyn Error>> {
    let status = Command::new("rustc")
        .arg(rs_file)
        .arg("-o")
        .arg(dest)
        .status()
        .expect("Failed to execute rustc");
    if status.success() {
        Ok(())
    } else {
        Err("Compiling rust output failed".into())
    }
    // std::io::stdout().write_all(&output.stdout).unwrap();
    // std::io::stderr().write_all(&output.stderr).unwrap();
}

fn run(executable: &Path) {
    Command::new(PathBuf::from("./").join(executable))
        .arg(executable)
        .status()
        .expect("Failed to execute executable");
}
