//use token::Token;

pub enum LexCmd {
    Symbol(char),
}

impl LexCmd {
    pub fn accepts(self, c: char) -> bool {
        match self {
            Symbol(sym) => c == sym,
        }
    }

    pub fn print(self) {
        match self {
            Symbol(c) => println!("Symbol({})", c)
        }
    }
}

pub fn symbol(sym: char) -> LexCmd {
    Symbol(sym)
}

/*
// Don't want this to be public
pub struct Lexer<T> {
    cmds: ~[(LexCmd, T)]
}

impl<T> Lexer<T> {
    pub fn new(cmds: ~[(LexCmd, T)]) -> ~Lexer<T> {
        ~Lexer { cmds: cmds }
    }

    // Want this to return a Token<T> channel, and maybe spawn a lexing
    // task that writes to the channel. For now, just return a list of tokens
    fn run(&self, file: ~str) -> ~[~Token<T>] {
        ~[]
    }
}
*/
