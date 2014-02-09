// What is "returned" by the Lexer
pub struct Token<T> {
    kind: T,
    val: ~str
}

impl<T> Token<T> {
    fn new(kind: T, val: ~str) -> ~Token<T> {
        ~Token { kind: kind, val: val }
    }
}
