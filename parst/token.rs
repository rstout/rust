// What is "returned" by the Lexer
#[deriving(Clone)]
pub struct Token<T> {
    kind: T,
    val: ~str
}

impl<T> Token<T> {
    pub fn new(kind: T, val: ~str) -> Token<T> {
        Token { kind: kind, val: val }
    }
}
