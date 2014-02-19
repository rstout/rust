// dfa.rs
// Binary DFAs

use lexer::LexCmd;
use lexer::symbol;
use scanner::TokenScanner;
use token::Token;
mod token;
mod lexer;
mod scanner;

#[deriving(Clone)]
enum MyToken {
    Plus,
    Minus,
}

impl ToStr for MyToken {
    fn to_str(&self) -> ~str {
        match self {
            &Plus => ~"Plus",
            &Minus => ~"Minus"
        }
    }
}

fn main() {
    let cmds = ~[symbol('p'), symbol('l'), symbol('u'), symbol('s')];
    let node = lex_cmds_to_nodes((cmds, Plus));
    let o = node.parse_token(~['p', 'l', 'u', 's']);
    match o {
        Some(tok) => println!("{}", tok.to_str()),
        None => println!("None")
    }
    //    node.print()
}

struct LexDFA<T> {
    nodes: ~[~LexNode<T>],
    //input: ~Iterator<char>
}

// TODO
impl<T> LexDFA<T> {
    fn lex(&self, scanner: TokenScanner<T>) -> Option<~[Token<T>]> {
        None
    }
}

// A LexNode is either a transition node or an accept node
enum LexNode<T> {
    Trans(LexCmd, ~LexNode<T>),
    Accept(T)
}

impl<T: Clone> LexNode<T> {
    pub fn tokenize(&self, scanner: TokenScanner<T>) -> TokenScanner<T> {
        let scanner1 = match self {
            &Accept(ref t) => {
                scanner.update_token(t.clone()).reset()
            }
        }

        // reset index explicitly?
        scanner1.reset_index()
    }

    pub fn parse_token(&self, input: &[char]) -> Option<T> {
        match self {
            &Accept(ref tok) => { Some(tok.clone()) }
            &Trans(ref cmd, ~ref node) if input.len() > 0 => {
                if cmd.accepts(*input.head()) {
                    node.parse_token(input.tail())
                } else { None }
            }
            _ => None
        }
    }
}

impl<T> LexNode<T> {
    pub fn print(self) {
        match self {
            Trans(cmd, ~node) => { cmd.print(); node.print() }
            Accept(_) => { println!("Accept") }
        }
    }
}

// Want to use foldl, no mutability
// Create a node path to ONE token
fn lex_cmds_to_nodes<T>((cmds, tok): (~[LexCmd], T)) -> ~LexNode<T> {
    cmds.iter().invert().fold(~Accept(tok), |node, &cmd| ~Trans(cmd, node))
}

// TODO: test lex_cmds_to_nodes
