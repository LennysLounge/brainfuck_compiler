use std::io::Write;

struct Input {
    buffer: Vec<char>,
    index: usize,
    handle: std::io::Stdin,
}
impl Input {
    fn read(&mut self) -> u8 {
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
            return 0;
        }
        self.index += 1;
        self.buffer[self.index - 1] as u8
    }
}

fn main() {
    let mut input = Input {
        buffer: Vec::new(),
        index: 0,
        handle: std::io::stdin(),
    };

    print!("Please input some text: ");
    _ = std::io::stdout().flush();

    for _ in 0..20 {
        print!("{}", input.read());
        _ = std::io::stdout().flush();
    }
}
