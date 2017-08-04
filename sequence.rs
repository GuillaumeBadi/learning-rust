
pub struct Sequence<T: Parser, U: Parser> {
    pub first: T,
    pub second: U,
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

    fn run(&self, mut s1: stream::StreamT) -> (Option<String>, stream::StreamT) {
        let (e1, s2) = self.first.run(s1);
        if e1 != None { return (e1, s1) }
        let (e2, s3) = self.second.run(s2);
        if e2 != None { return (e2, s1) }
        (None, s3)
    }
}
