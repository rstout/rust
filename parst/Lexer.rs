//use std::comm::Chan;

fn main() {

}

enum C0Tokens {
    Plus,
    Digit
}

/*
fn symbol(c: char) -> ~LexCmd {
    ~LexCmd
}
*/

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

struct Lexer<T> {
    cmds: ~[(~LexCmd, T)]
}

impl<T> Lexer<T> {
    fn new(cmds: ~[(~LexCmd, T)]) -> ~Lexer<T> {
        ~Lexer { cmds: cmds }
    }

    // Want this to return a Token<T> channel, and maybe spawn a lexing
    // task that writes to the channel. For now, just return a list of tokens
    fn run(&self, file: ~str) -> ~[~Token<T>] {
        ~[]
    }
}
