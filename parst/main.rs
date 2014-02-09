use lexer::symbol;
use lexer::Lexer;
use lexer::many1;
use lexer::range;

mod token;
mod lexer;

enum C0Tokens {
    Plus,
    Digit
}

fn main() {
    let plus = symbol('+');
    let digit = many1(range('0', '9'));
    let toks = ~[(plus , Plus),
                 (digit, Digit)
                 ];

    // Should put new behind a function/interface
    let lexer: ~Lexer<C0Tokens> = lexer::Lexer::new(toks);
}
