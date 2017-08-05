
mod parser {
    pub enum Error {
        UnexpectedEndOfInput(&'static str),
        UnexpectedToken(&'static str, &'static str),
        ExpectedEndOfInput(&'static str),
    }

    pub enum Result<T> {
        Right(T),
        Left(Error),
    }

    pub fn formatError(error: Error) -> String {
        match error {
            Error::UnexpectedToken(u, e) => format!("Unexpected token {}, expected {}", u, e),
            Error::UnexpectedEndOfInput(s) => format!("Unexpected end of input, expected {}", s),
            Error::ExpectedEndOfInput(s) => format!("Unexpected {}, expected end of input", s)
        }
    }

    /**
     * Streams
     */
    #[derive(Clone, Copy)]
    pub struct StreamT {
        pub iterable: &'static str,
        pub line: usize,
        pub column: usize,
        pub position: usize,
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
                None => '\0',
                Some(ch) => ch,
            }
        }
    }

    pub fn create_stream(iterable: &'static str) -> StreamT {
        StreamT { iterable: iterable, line: 1, column: 1, position: 0 }
    }

    /**
     * Parser
     */

    pub trait Parser {
        fn get_name(&self) -> String;
        fn run(&self, mut stream: StreamT) -> (Option<String>, StreamT);
    }

    /**
     * Many
     */
    #[derive(Clone, Copy)]
    pub struct Many<T> {
        parser: T,
    }

    pub fn many<T: Parser>(parser: T) -> Many<T> {
        Many { parser: parser }
    }

    impl<T: Parser> Parser for Many<T> {
        fn get_name(&self) -> String {
            format!("many({})", self.parser.get_name())
        }

        fn run(&self, stream: StreamT) -> (Option<String>, StreamT) {
            let mut st = stream;
            while true {
                let (err, s) = self.parser.run(st);
                if err != None {
                    return (None, st)
                }
                st = s;
            }
            (None, st)
        }
    }

    pub fn many1<T: Parser + Copy>(parser: T) -> Seq<T, Many<T>> {
        seq(parser, many(parser))
    }

    /**
     * Maybe
     */
    #[derive(Clone, Copy)]
    pub struct Maybe<T> {
        parser: T,
    }

    pub fn maybe<T: Parser>(parser: T) -> Maybe<T> {
        Maybe { parser: parser }
    }

    impl<T: Parser> Parser for Maybe<T> {
        fn get_name(&self) -> String {
            format!("maybe({})", self.parser.get_name())
        }

        fn run(&self, stream: StreamT) -> (Option<String>, StreamT) {
            let (_, s) = self.parser.run(stream);
            (None, s)
        }
    }

    /**
     * Choice
     */
    #[derive(Clone, Copy)]
    pub struct Choice<T: Parser, U: Parser> {
        first: T,
        second: U,
    }

    pub fn choice<T: Parser, U: Parser>(first: T, second: U) -> Choice<T, U> {
        Choice { first: first, second: second }
    }

    impl<T, U> Parser for Choice<T, U> where T: Parser, U: Parser {
        fn get_name(&self) -> String {
            format!(
                "choice({}, {})",
                self.first.get_name(),
                self.second.get_name(),
            )
        }

        fn run(&self, mut s1: StreamT) -> (Option<String>, StreamT) {
            let (e1, s2) = self.first.run(s1);
            if e1 == None { return (None, s2) }
            let (e2, s3) = self.second.run(s1);
            if e2 == None { return (None, s3) }
            let error = Some(format!("Cannot match {} with {:?}", self.get_name(), s1.iterable.chars().next()));
            (error, s1)
        }
    }

    /**
     * Seq
     */
    #[derive(Clone, Copy)]
    pub struct Seq<T: Parser, U: Parser> {
        first: T,
        second: U,
    }

    pub fn seq<T: Parser, U: Parser>(first: T, second: U) -> Seq<T, U> {
        Seq { first: first, second: second }
    }

    impl<T, U> Parser for Seq<T, U> where T: Parser, U: Parser {
        fn get_name(&self) -> String {
            format!(
                "seq({}, {})",
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
    #[derive(Clone, Copy)]
    pub struct Char {
        character: char,
    }

    pub fn ch(c: char) -> Char {
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
    #[derive(Clone, Copy)]
    pub struct Symbol {
        string: &'static str,
    }

    impl Parser for Symbol {

        fn get_name(&self) -> String {
            format!("Symbol {}", self.string)
        }

        fn run(&self, mut stream: StreamT) -> (Option<String>, StreamT) {
            if stream.iterable.starts_with(self.string) {
                (None, stream.consume_string(self.string))
            } else {
                let error = format!("Cannot match {} with {}", stream.iterable, self.string);
                (Some(error), stream)
            }
        }
    }

    pub fn symbol(string: &'static str) -> Symbol {
        Symbol { string: string }
    }
}

#[cfg(test)]
mod test {

    use parser::{ch,symbol,create_stream,Parser};

    fn should_consume_all<T: Parser>(s: &'static str, parser: T) {
        let stream = create_stream(s);
        let (error, s) = parser.run(stream);
        assert!(error == None);
        assert!(s.iterable.len() == 0);
    }

    fn should_consume<T: Parser>(s: &'static str, parser: T) {
        let stream = create_stream(s);
        let (error, _) = parser.run(stream);
        assert!(error == None);
    }

    fn should_fail<T: Parser>(s: &'static str, parser: T) {
        let stream = create_stream(s);
        println!("Hello");
        let (error, _) = parser.run(stream);
        assert!(error != None);
    }

    #[test]
    fn test_char() {
        should_fail("a", ch('b'));
        should_fail("ba", ch('a'));
        should_consume_all("a", ch('a'));
        should_fail("", ch('1'));
    }

    #[test]
    fn test_symbol() {
        should_fail("ab", symbol("abc"));
        should_consume("ab ba", symbol("ab"));
        should_consume_all("ab ba", symbol("ab ba"));
    }
}
