use std::to_str::ToStr;

//trait DFA {}
struct DFA;

struct Token<T> {
    kind: ~T,
    val: ~str,
    lex: ~LexCmd
}

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

}

impl<T> Lexer<T> {
    fn new(dict: ~Hashmap<LexCmd, T>) -> Lexer<T> {
        // undefined
    }

    fn lex(filepath: ~str) -> Stream<Token<T>> {

    }
}

/*
fn getGeneric<T: Token<T>>(a: T) -> ~str {
    a.get()
}
*/

// under the hood
struct LexCmd {
    /*
    state: ~State;
    accept: fn; // update DFA?
    dfa: DFA; // Need way combine DFAs/LexCmds
    */
    kind: int
}

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
