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
/*
    let cmds = ~[symbol('p'), symbol('l'), symbol('u'), symbol('s')];
    let node = lex_cmds_to_nodes((cmds, Plus));
    let o = node.parse_token(~['p', 'l', 'u', 's']);
    match o {
        Some(tok) => println!("{}", tok.to_str()),
        None => println!("None")
    }
*/
    //    node.print()
    let u = 8u8;
}

struct LexDFA<T> {
    branches: ~[~LexNode<T>],
    //input: ~Iterator<char>
}

// TODO: better names than scanner2?
impl<T: Clone> LexDFA<T> {
    // Creates a new TokenScanner and calls lex
    /*
    fn run(&self, input: ~Iterator<u8>) -> Option<~[Token<T>]> {
        let scanner: TokenScanner<T> = TokenScanner::new(input);
        let opt_tokens = self.lex(scanner);
        match opt_tokens {
            Some(tokens) => Some(tokens.reverse()),
            None => None
        }
    }
     */

    // TODO: doc
    // TODO: note that if the char stream is not completely successfully
    // tokenize the input u8 stream, None is returned
    // ** Callers need to reverse the returned value
    fn lex(&self, scanner: TokenScanner<T>) -> Option<~[Token<T>]> {
        // Fold over branches. If the resultant TokenScanner has opt_token
        // equal to Some(token), flush the TokenScanner and call the
        // recursion. If the result is None, return None. If the result is
        // Some(tok_array), append token to the front, and return
        // Some(new_tok_array).
        // If the resultant TokenScanner is None, if TokenScanner is
        // exhausted return Some([]), else return None
        let scanner2 = self.fold_branches(scanner);
        let scanner2_token = scanner2.token.clone();
        match scanner2_token {
            Some(token) => {
                // ? Reminder: exhausted check somewhere here
                let scanner3 = scanner2.flush();
                match self.lex(scanner3) {
                    Some(mut tokens) => {
                        tokens.push(token);
                        Some(tokens)
                    }
                    None => None
                }
            }
            // If we failed to tokenize input, fail by returning None
            None => {
                // TODO: make doc better here
                // If there is no more input to tokenize and there is no
                // leftover input after exhaustion, return Some(~[]). Else
                // We failed to parse the current input (in scanner2.buf)
                if scanner2.exhausted && scanner2.len == 0 {
                    Some(~[])
                } else {
                    None
                }
            }
        }
    }

    // TODO: doc
    fn fold_branches(&self, scanner: TokenScanner<T>) -> TokenScanner<T> {
        self.branches.iter().fold(scanner, |scanner2, node| node.tokenize(scanner2))
    }
}

// A LexNode is either a transition node or an accept node
enum LexNode<T> {
    Trans(LexCmd, ~LexNode<T>),
    Accept(T)
}

impl<T: Clone> LexNode<T> {
    // TODO: doc
    pub fn tokenize(&self, scanner: TokenScanner<T>) -> TokenScanner<T> {
        match self {
            &Accept(ref t) => scanner.update_token(t.clone()).reset(),
            &Trans(ref cmd, ~ref node) => {
                let (opt_next, scanner2) = scanner.next_char();
                match opt_next {
                    Some(next) if cmd.accepts(next) => {
                        node.tokenize(scanner2)
                    }
                    _ => scanner2.reset()
                }
            }
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

// Test template
#[cfg(test)]
mod test {
    #[test]
    fn test_fn() {

    }
}
