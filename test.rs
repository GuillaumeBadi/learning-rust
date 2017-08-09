
mod parser {

    /**
     * Streams
     */
    #[derive(Clone, Copy, Debug)]
    pub struct StreamT {
        pub iterable: &'static str,
        pub line: usize,
        pub column: usize,
        pub position: usize,
    }

    impl StreamT {
        pub fn print(&self) {
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
                None => '\0',
                Some(ch) => ch,
            }
        }
    }

    pub fn create_stream(iterable: &'static str) -> StreamT {
        StreamT { iterable: iterable, line: 1, column: 1, position: 0 }
    }

    #[derive(Debug)]
    enum Error {
        UnexpectedToken(String, String),
        UnexpectedEndOfInput(String),
        ExpectedEndOfInput(String),
    }

    #[derive(Debug)]
    enum Result<T> {
        Success {
            stream: StreamT,
            output: T,
        },
        Error(Error),
        Nothing,
    }

    pub trait Parser<T> {
        fn name(&self) -> String;
        fn run(&self, StreamT) -> Result<T>;
    }

    fn ok<T>(r: T, stream: StreamT) -> Result<T> {
        Result::Success { stream: stream, output: r }
    }

    fn ko<T>(error: Error, stream: StreamT) -> Result<T> {
        Result::Error(error)
    }

    fn nothing<T>(stream: StreamT) -> Result<T> {
        Result::Nothing
    }

    pub struct Char { ch: char }
    pub struct Maybe<T> { parser: T }

    impl Parser<char> for Char {
        fn name(&self) -> String { format!("ch(\"{}\")", self.ch) }

        fn run(&self, stream: StreamT) -> Result<char> {
            let first = stream.get_first_char();
            if first == self.ch {
                ok(self.ch, stream.consume(self.ch))
            } else {
                ko(Error::UnexpectedToken(first.to_string(), self.ch.to_string()), stream)
            }
        }
    }

    impl<T, U> Parser<U> for Maybe<T> where T: Parser<U> {
        fn name(&self) -> String {
            format!("maybe({})", self.parser.name())
        }

        fn run(&self, stream: StreamT) -> Result<U> {
            let result = self.parser.run(stream);
            match result {
                Result::Success { output, stream } => ok(output, stream),
                Result::Error(e) => nothing(stream),
                Result::Nothing => nothing(stream),
            }
        }
    }

    pub fn ch(c: char) -> Char {
        Char { ch: c }
    }

    pub fn maybe<T>(parser: T) -> Maybe<T> {
        Maybe { parser: parser }
    }

}

use parser::*;

fn main() {
    let stream = create_stream("rello World");
    let parser = maybe(ch('H'));
    let result = parser.run(stream);
    println!("{:?}", result);
}
