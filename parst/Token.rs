use std::to_str::ToStr;

//trait DFA {}
struct DFA;

struct Token<T> {
    kind: ~T,
    val: ~str,
    lex: ~LexCmd
}

// Some of these functions should be defined in terms of the others, and
// can thus be the same for all LexCmds
struct LexCmd {
    val: ~str,
    done: bool,
    failed: bool,
    accepts: fn(char) -> bool,
    // Can implement many and many1 here: if self.accepts, return self with
    // updated val, else return next LexCmd in the chain
    consume: fn(&self, char) -> Option<~LexCmd>,
    // Use if we've consumed something and we're not done
    get_next: fn(&self) -> ~LexCmd,
    // Used for (>>) ?
    set_next: fn(&self)
}

fn symbol(c: char) -> ~LexCmd {
    // use closure here
    fn accepts(c2: char) -> bool {
        if c == c2 {
            true
        } else {
            false
        }
    }

    fn consume(self: ~LexCmd, c: char) -> Option<~LexCmd> {
        if self.accepts(c) {
            Some(LexCmd { val: c, done: true,
                          accepts: fn (c: char) -> bool { false },
                          consume: fn(&self, char) -> Option<~LexCmd>
                          { None }
                })
        } else {
            None
        }
    }
}

/*
fn: >> (~LexCmd) -> ~LexCmd {

}
*/

impl<T> Token<T> {
    fn new(k: ~T, lex: ~LexCmd) -> Token<T> {
        Token { kind: k, val: ~"", lex: lex }
    }
}

enum MyTokenType {
    Plus,
    Minus,
    Int
}

struct Lexer<T> {
    kind: T
}

/*
impl<T> Lexer<T> {
    fn new(dict: ~Hashmap<LexCmd, T>) -> Lexer<T> {
        // undefined
    }

    fn lex(filepath: ~str) -> Stream<Token<T>> {

    }
}
*/

/*
fn getGeneric<T: Token<T>>(a: T) -> ~str {
    a.get()
}
*/

// under the hood
/*
struct LexCmd {
    state: ~State;
    accept: fn; // update DFA?
    dfa: DFA; // Need way combine DFAs/LexCmds
    kind: int
}
*/

fn main() {
    //let t: Token<MyTokenType> = Token { kind: Plus, value: ~"" };
    //let t: Token<MyTokenType> = Token::new(~Plus);
}


/*
let mut map = HashMap::<LexCmd, MyTokenType>::new();
map.insert(~"a key", ~[~"value"]);
map.insert(~"z key", ~[~"value"]);
*/

enum Directions {
    North,
    East,
    South,
    West
}

impl ToStr for Directions {
    fn to_str(&self) -> ~str {
        match *self {
            North => ~"North",
            East  => ~"East",
            South => ~"South",
            West  => ~"West"
        }
    }
}
