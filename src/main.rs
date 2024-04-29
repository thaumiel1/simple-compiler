use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let line_size: usize = args[2]
        .parse()
        .expect("Unable to parse second argument as number.");
    let line: Vec<usize> = vec![0; line_size];
    let contents = fs::read_to_string(file_path).expect("Failed to read file in argument.");
    let contents = contents.trim();
    let commands = contents.chars().collect::<Vec<char>>();
    let mut line = Line { line, pointer: 0 };
    // Allowed characters: < > + - .
    // <, move pointer left
    // >, moev pointer right
    // +, increment current index by 1
    // -, decrement current index by 1
    // ., print ascii character for the value stored in the index
    for command in commands {
        match command {
            '<' => line.left(),
            '>' => line.right(),
            '+' => line.increment(),
            '-' => line.decrement(),
            '.' => line.print(),
            _ => break,
        }
    }
}

struct Line {
    line: Vec<usize>,
    pointer: usize,
}

impl Line {
    fn left(&mut self) {
        if self.pointer == 0 {
            println!("Pointer is already as far left as possible.");
        } else {
            self.pointer -= 1;
        }
    }

    fn right(&mut self) {
        if self.pointer == self.line.len() {
            println!("Pointer is already as far right as possible.");
        } else {
            self.pointer += 1;
        }
    }

    fn increment(&mut self) {
        if self.line[self.pointer] == usize::MAX {
            println!("Value is at its maximum already.");
        } else {
            self.line[self.pointer] += 1;
        }
    }

    fn decrement(&mut self) {
        if self.line[self.pointer] == usize::MIN {
            println!("Value is at its minimum already.");
        } else {
            self.line[self.pointer] -= 1;
        }
    }

    fn print(&self) {
        print!("{}", self.line[self.pointer] as u8 as char);
    }
}
