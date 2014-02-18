use std::str::from_chars;
use token::Token;
mod token;

fn main() {}

// TODO: doc
pub struct TokenScanner<T> {
    input: ~Iterator<u8>,
    buf: ~[char],
    len: uint, // number of currently parsed chars for the current token
    token: Option<Token<T>>,
    exhausted: bool
}

// Before creating methods, design DFA arch

impl<T: Clone> TokenScanner<T> {

    fn set_token(self, t: T) -> TokenScanner<T> {
        let s = from_chars(self.buf.slice_to(self.len));
        let token = Token::new(t, s);
        let mut scanner = self;
        scanner.token = Some(token);
        scanner
    }

    fn get_token(self) -> Option<Token<T>> {
        self.token
    }

    pub fn update_token(self, t: T) -> TokenScanner<T> {
    //pub fn update_token(&self, t: T) -> Option<T> {
        // Don't set token if there is a longer token already set
        let token = self.token.clone();
        match token {
            Some(prev_token) => {
                // Given two tokens, we take the longest
                if self.len > prev_token.val.len() {
                    self.set_token(t)
                } else {
                    self
                }
            }
            // Only set a non-zero length token
            None if self.len > 0 => { self.set_token(t) }
            _ => { self }
        }
    }
}

/*

Future Note:

    pub fn update_token(self, t: T) -> TokenScanner<T> {
        let len = self.len;
        let ref token = self.token;
        self
    }

fails, but

    pub fn update_token(self, t: T) -> Option<T> {
        let len = self.len;
        let ref token = self.token;
        None
    }

*/
