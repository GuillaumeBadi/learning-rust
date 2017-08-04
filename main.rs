
use std::ops::{Add,Shr};

/**
 * Streams
 */
#[derive(Clone, Copy)]
struct StreamT {
    iterable: &'static str,
    line: usize,
    column: usize,
    position: usize,
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

/**
 * Parser
 */

struct ParserT<T>(T);

trait Parser {
    fn get_name(&self) -> String;
    fn run(&self, mut stream: StreamT) -> (Option<String>, StreamT);
}

impl<T, U> Shr<T> for ParserT<T> where T: Parser {
    type Output = ParserT<Sequence<T, U>>;
    fn shr(self, other: ParserT<T>) -> Self::Output {
        println!("Ok");
        self
    }
}

/**
 * Sequence
 */
struct Sequence<T: Parser, U: Parser> {
    first: T,
    second: U,
}

fn sequence<T: Parser, U: Parser>(first: T, second: U) -> Sequence<T, U> {
    Sequence { first: first, second: second }
}

impl<T, U> Parser for Sequence<T, U> where T: Parser, U: Parser {
    fn get_name(&self) -> String {
        format!(
            "Sequence({}, {})",
            self.first.get_name(),
            self.second.get_name(),
        )
    }

    fn run(&self, mut s1: StreamT) -> (Option<String>, StreamT) {
        let (e1, s2) = self.first.run(s1);
        if e1 != None { return (e1, s1) }
        let (e2, s3) = self.second.run(s2);
        if e2 != None { return (e2, s1) }
        (None, s3)
    }
}

/**
 * Chars
 */
struct Char {
    character: char,
}

fn ch(c: char) -> Char {
    Char { character: c }
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

/**
 * Symbols
 */
struct Symbol {
    string: &'static str,
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

fn main() {
    let mut stream = create_stream("He\nllo World");
    let parser = symbol("He\nllo");
    println!("{}", parser.get_name());
}
