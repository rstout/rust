use std::str::from_chars;
use token::Token;
mod token;

fn main() {}

// TODO: doc
// Really a BufferedReader
pub struct TokenScanner<T> {
    input: ~Iterator<u8>,
    buf: ~[char],
    // TODO: better name?
    len: uint, // number of currently parsed chars for the current token
    token: Option<Token<T>>,
    exhausted: bool
}

// Before creating methods, design DFA arch

impl<T: Clone> TokenScanner<T> {
    pub fn new(input: ~Iterator<u8>) -> TokenScanner<T> {
        TokenScanner {
            input: input,
            buf: ~[],
            len: 0,
            token: None,
            exhausted: false
        }
    }

    // TODO: doc
    fn set_token(self, t: T) -> TokenScanner<T> {
        let s = from_chars(self.buf.slice_to(self.len));
        let token = Token::new(t, s);
        let mut scanner = self;
        scanner.token = Some(token);
        scanner
    }

    // TODO: doc
    pub fn update_token(self, t: T) -> TokenScanner<T> {
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
            // TODO: raise error/exception?
            None if self.len > 0 => self.set_token(t),
            _ => self
        }
    }

    // TODO: doc
    // TODO: maybe breakup into smaller functions?
    // Updates len, buf, exhausted
    pub fn next_char(self) -> (Option<char>, TokenScanner<T>) {
        if self.len < self.buf.len() {
            let mut scanner = self;
            let next = scanner.buf[scanner.len];
            scanner.len += 1;
            (Some(next), scanner)
        } else {
            let mut scanner = self;
            let opt_next = scanner.input.next();
            match opt_next {
                Some(u) => {
                    let next = u as char;
                    scanner.buf.push(next);
                    scanner.len += 1;
                    (Some(next), scanner)
                }
                None => {
                    scanner.exhausted = true;
                    (None, scanner)
                }
            }
        }
    }

    // TODO: doc
    pub fn reset(self) -> TokenScanner<T> {
        let mut scanner = self;
        scanner.len = 0;
        scanner
    }

    // TODO: impl, doc
    // Flushes the TokenScanner's buffer, zeros length, flushes token
    pub fn flush(self) -> TokenScanner<T> {
        let mut scanner = self;
        scanner.buf = scanner.buf.slice_from(scanner.len).to_owned();
        scanner.len = 0;
        scanner.token = None;
        scanner
    }
}

#[cfg(test)]
mod test {
    //use std::vec::VecIterator;
    use std::io::BufReader;
    use std::io::extensions::Bytes;
    // ByteIterator has a lifetime.
    // Send things cannot contain borrowed pointers that are not 'static
    // there's no way to fix it, that code isn't safe. You'd be returning
    // an object which has a mutable reference to the BufReader, but once
    // you return, the BufReader will be freed, thus making the reference
    // dangling.

    //fn get_iter() -> ~Iterator<u8> {
    //fn get_iter() -> ByteIterator<BufReader> {

    fn get_iter() -> Option<int> {
        let buf = ~[30u8, 31u8];
        let mut buf_reader = ~BufReader::new(buf);
        let byte_iter = Bytes::new(buf_reader);
        //let a: ~VecIterator<int> = ~[1, 2, 3, 4].iter();
        //let b: ~Iterator<&int> = a;
        //b
        //let iter: ~Iterator<u8> = byte_iter as ~Iterator<u8>;
        //iter
        //byte_iter
        None
    }
/*
    //fn f(c: char) {}
    //fn f(c: &char) {}
*/
    fn f<'a>(byte_iter: &'a Bytes<BufReader>) -> &'a Iterator<u8> {
        byte_iter as &'a Iterator<u8>
    }

    #[test]
    fn test_reset_sets_len_to_zero() {

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
succeeds
*/
