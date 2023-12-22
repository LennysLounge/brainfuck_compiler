struct Input {
    buffer: Vec<char>,
    index: usize,
    handle: std::io::Stdin,
}
impl Input {
    #[allow(unused)]
    fn read(&mut self) -> std::num::Wrapping<u8> {
        if self.index >= self.buffer.len() {
            {
                let mut input = String::new();
                self.handle
                    .read_line(&mut input)
                    .expect("Invalid stdin stream");
                input = input.replace("\r\n", "\n");
                self.buffer = input.chars().collect();
                self.index = 0;
            }
        }
        // If the buffer is still to small that means we have reached EOF in stdin.
        if self.index >= self.buffer.len() {
            return std::num::Wrapping(0);
        }
        self.index += 1;
        std::num::Wrapping(self.buffer[self.index - 1] as u8)
    }
    fn new() -> Self {
        Self {
            buffer: Vec::new(),
            index: 0,
            handle: std::io::stdin(),
        }
    }
}
