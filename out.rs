#![allow(unused_mut)]

struct Input {
    buffer: Vec<char>,
    index: usize,
    handle: std::io::Stdin,
}
impl Input {
    #[allow(unused)]
    fn read(&mut self) -> std::num::Wrapping<u8> {
        if self.index >= self.buffer.len() {
            let mut input = String::new();
            self.handle
                .read_line(&mut input)
                .expect("Invalid stdin stream");
            input = input.replace("\r\n", "\n");
            self.buffer = input.chars().collect();
            self.index = 0;
        }
        // If the buffer is still to small that means we have reached EOF in stdin.
        if self.index >= self.buffer.len() {
            return std::num::Wrapping(0);
        }
        self.index += 1;
        std::num::Wrapping(self.buffer[self.index - 1] as u8)
    }
}

fn main() {
    let mut tape = [std::num::Wrapping(0u8); 30_000];
    let mut pointer = 0usize;
    #[allow(unused)]
    let mut input = Input {
        buffer: Vec::new(),
        index: 0,
        handle: std::io::stdin(),
    };
    tape[pointer] += 1;
    tape[pointer] += 1;
    tape[pointer] += 1;
    tape[pointer] += 1;
    tape[pointer] += 1;
    tape[pointer] += 1;
    tape[pointer] += 1;
    tape[pointer] += 1;
    while tape[pointer].0 != 0 {
        pointer += 1;
        tape[pointer] += 1;
        tape[pointer] += 1;
        tape[pointer] += 1;
        tape[pointer] += 1;
        while tape[pointer].0 != 0 {
            pointer += 1;
            tape[pointer] += 1;
            tape[pointer] += 1;
            pointer += 1;
            tape[pointer] += 1;
            tape[pointer] += 1;
            tape[pointer] += 1;
            pointer += 1;
            tape[pointer] += 1;
            tape[pointer] += 1;
            tape[pointer] += 1;
            pointer += 1;
            tape[pointer] += 1;
            pointer -= 1;
            pointer -= 1;
            pointer -= 1;
            pointer -= 1;
            tape[pointer] -= 1;
        }
        pointer += 1;
        tape[pointer] += 1;
        pointer += 1;
        tape[pointer] += 1;
        pointer += 1;
        tape[pointer] -= 1;
        pointer += 1;
        pointer += 1;
        tape[pointer] += 1;
        while tape[pointer].0 != 0 {
            pointer -= 1;
        }
        pointer -= 1;
        tape[pointer] -= 1;
    }
    pointer += 1;
    pointer += 1;
    print!("{}", tape[pointer].0 as char);
    pointer += 1;
    tape[pointer] -= 1;
    tape[pointer] -= 1;
    tape[pointer] -= 1;
    print!("{}", tape[pointer].0 as char);
    tape[pointer] += 1;
    tape[pointer] += 1;
    tape[pointer] += 1;
    tape[pointer] += 1;
    tape[pointer] += 1;
    tape[pointer] += 1;
    tape[pointer] += 1;
    print!("{}", tape[pointer].0 as char);
    print!("{}", tape[pointer].0 as char);
    tape[pointer] += 1;
    tape[pointer] += 1;
    tape[pointer] += 1;
    print!("{}", tape[pointer].0 as char);
    pointer += 1;
    pointer += 1;
    print!("{}", tape[pointer].0 as char);
    pointer -= 1;
    tape[pointer] -= 1;
    print!("{}", tape[pointer].0 as char);
    pointer -= 1;
    print!("{}", tape[pointer].0 as char);
    tape[pointer] += 1;
    tape[pointer] += 1;
    tape[pointer] += 1;
    print!("{}", tape[pointer].0 as char);
    tape[pointer] -= 1;
    tape[pointer] -= 1;
    tape[pointer] -= 1;
    tape[pointer] -= 1;
    tape[pointer] -= 1;
    tape[pointer] -= 1;
    print!("{}", tape[pointer].0 as char);
    tape[pointer] -= 1;
    tape[pointer] -= 1;
    tape[pointer] -= 1;
    tape[pointer] -= 1;
    tape[pointer] -= 1;
    tape[pointer] -= 1;
    tape[pointer] -= 1;
    tape[pointer] -= 1;
    print!("{}", tape[pointer].0 as char);
    pointer += 1;
    pointer += 1;
    tape[pointer] += 1;
    print!("{}", tape[pointer].0 as char);
    pointer += 1;
    tape[pointer] += 1;
    tape[pointer] += 1;
    print!("{}", tape[pointer].0 as char);
}
