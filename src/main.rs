#[allow(unused)]
mod input;

use std::{
    error::Error,
    fs::{self, File},
    io::Write,
};

fn main() -> Result<(), Box<dyn Error>> {
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

    let program = fs::read_to_string("hello_world.bf")?;
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
