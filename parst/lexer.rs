use token::Token;

enum LexCmd {
    Symbol(char),
    OneOf(~[char]),
    Range(char, char), // Inclusive
    Modified(LexModifier, ~LexCmd)
}

impl LexCmd {
    fn accepts(&self, c: char) -> bool {
        match *self {
            Symbol(sym) => c == sym,
            OneOf(ref cs) => cs.contains(&c),
            Range(low, high) => low <= c && c <= high,
            Modified(modifier, ref cmd) => cmd.accepts(c)
        }
    }
}

pub fn symbol(sym: char) -> ~LexCmd {
    ~Symbol(sym)
}

pub fn oneOf(chars: ~[char]) -> ~LexCmd {
    ~OneOf(chars)
}

// Inclusive
pub fn range(low: char, high: char) -> ~LexCmd {
    ~Range(low, high)
}

pub fn many(cmd: ~LexCmd) -> ~LexCmd {
    ~Modified(Many, cmd)
}

pub fn many1(cmd: ~LexCmd) -> ~LexCmd {
    ~Modified(Many1, cmd)
}

// How do we handle multiple modifiers?
enum LexModifier {
    Many,
    Many1,
    Not,
    NoAdd // Don't add parsed input to the built-up string
}


// Don't want this to be public
pub struct Lexer<T> {
    cmds: ~[(~LexCmd, T)]
}

impl<T> Lexer<T> {
    pub fn new(cmds: ~[(~LexCmd, T)]) -> ~Lexer<T> {
        ~Lexer { cmds: cmds }
    }

    // Want this to return a Token<T> channel, and maybe spawn a lexing
    // task that writes to the channel. For now, just return a list of tokens
    fn run(&self, file: ~str) -> ~[~Token<T>] {
        ~[]
    }
}
