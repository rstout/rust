//use std::comm::Chan;

fn main() {

}

enum C0Tokens {
    Plus,
    Digit
}

struct HasClosure<'a> {
    closure: 'a |int| -> bool
}

fn new_eq_check<'a>(i: int) -> ~HasClosure<'a> {
    ~HasClosure { closure: |j| { i == j } }
}

/*
fn symbol<'a>(sym: char) -> ~LexCmd<'a> {
    ~LexCmd {
        kind: Symbol,
        accepts: |c| { sym == c },
        modifier: None
    }
}

// Is this needed? Doesn't having an accepts function make this obsolete?
enum LexKind {
    Symbol,
    OneOf,
    Range
}

// How do we handle multiple modifiers?
enum LexModifier {
    Many,
    Many1,
    Not
}

struct LexCmd<'a> {
    kind: LexKind,
    accepts: 'a |char| -> bool, // maybe a closure
    modifier: Option<LexModifier>
}
*/

/*
// (>>) sets the next field
struct LexCmd {
    val: ~str,
    accepts: fn (char) -> bool,
    done: bool,
    // Sets done
    // If accepts and not done, add to val and return next with the val.
    // If accepts and done, "return" val
    // If does not accept, return None (?)
    consume: fn(char) -> Option<~LexCmd>,
    next: Option<~LexCmd>
}
*/

/*
// What is "returned" by the Lexer
struct Token<T> {
    kind: T,
    val: ~str
}

impl<T> Token<T> {
    fn new(kind: T, val: ~str) -> ~Token<T> {
        ~Token { kind: kind, val: val }
    }
}

struct Lexer<'a, T> {
    cmds: ~[(~LexCmd<'a>, T)]
}

impl<'a, T> Lexer<'a, T> {
    fn new(cmds: ~[(~LexCmd<'a>, T)]) -> ~Lexer<'a, T> {
        ~Lexer { cmds: cmds }
    }

    // Want this to return a Token<T> channel, and maybe spawn a lexing
    // task that writes to the channel. For now, just return a list of tokens
    fn run(&self, file: ~str) -> ~[~Token<T>] {
        ~[]
    }
}
*/
