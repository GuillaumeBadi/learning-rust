
#[derive(Clone, Copy)]
struct StreamT {
    iterable: &'static str,
    line: usize,
    column: usize,
    position: usize,
}

struct Char {
    character: char,
}

struct Symbol {
    string: &'static str,
}

trait Parser {
    fn get_name(&self) -> String;
    fn run(&self, mut stream: StreamT) -> (Option<String>, StreamT);
}

impl Parser for Symbol {
    fn get_name(&self) -> String {
        format!("Symbol {}", self.string)
    }

    fn run(&self, mut stream: StreamT) -> (Option<String>, StreamT) {
        if (stream.iterable.starts_with(self.string)) {
            (None, stream.consume_string(self.string))
        } else {
            let error = format!("Cannot match {} with {}", stream.iterable, self.string);
            (Some(error), stream)
        }
    }
}
fn symbol(string: &'static str) -> Symbol {
    Symbol { string: string }
}

impl Parser for Char {
    fn get_name(&self) -> String {
        format!("Char {}", self.character)
    }

    fn run(&self, mut stream: StreamT) -> (Option<String>, StreamT) {
        let ch = stream.get_first_char();
        if self.character == ch {
            (None, stream.consume(ch))
        } else {
            let error = format!("Couldn't match character {} with {}", ch, self.character);
            (Some(error), stream)
        }
    }
}

fn ch(c: char) -> Char {
    Char { character: c }
}

impl StreamT {
    fn print(&self) {
        println!("Iterable: {}", self.iterable);
        println!("Column: {}", self.column);
        println!("Line: {}", self.line);
    }

    fn consume(mut self, c: char) -> StreamT {
        if c == '\n' {
            self.line += 1;
            self.column = 1;
            self.position += 1;
        } else {
            self.column += 1;
            self.position += 1;
        }
        self.iterable = &self.iterable[1..];
        self
    }

    fn consume_string(mut self, string: &str) -> StreamT {
        let lines = string.lines();

        let last = lines.last();
        match last {
            Some(l) => {
                self.column = l.len();
                self.line += string.matches('\n').count();
            },
            None => panic!("Error")
        }

        self.iterable = &self.iterable[string.len()..];
        self
    }

    fn get_first_char(&self) -> char {
        match self.iterable.chars().next() {
            None => panic!("wow"),
            Some(ch) => ch
        }
    }
}

fn create_stream(iterable: &'static str) -> StreamT {
    StreamT { iterable: iterable, line: 1, column: 1, position: 0 }
}

fn main() {
    let mut stream = create_stream("He\nllo World");
    let parser = symbol("He\nllo");
    println!("{}", parser.get_name());
}
