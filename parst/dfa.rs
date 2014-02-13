// dfa.rs
// Binary DFAs

fn main() {}

// add functions for querying state
// make a trait?
trait DFA<T, U> {
    fn consume(T);
    fn get_start_node() -> ~Node<T, U>;
    fn get_current_node() -> ~Node<T, U>;
}


// T is input type, U is metadata type
// We can save Node links in a struct that impls this trait, and then
// fn transition() can case on its input an return one of the saved Node links
trait Node<T, U> {
    fn transition(&self, T) -> ~Node<T, U>;
    fn get_metadata(&self) -> Option<U>;
    fn is_accept(&self) -> bool;
}


// Should be put in lexer.rs
struct LexNode<T> {
    token_kind: T,
    is_accept: bool, // Is an accepting state
    lex_cmd: ~LexCmd, // Has an accept() function
    success_node: ~LexNode<T>,
    fail_node: ~LexNode<T> // Make an option instead of start node?
}

impl<T> Node<char, T> for LexNode<T> {
    fn transition(&self, c: char) -> ~LexNode<T> {
        if self.lex_cmd.accept(c) {
            // But this is an owned pointer, can we hand it out?
            self.success_node
        } else {
            self.fail_node
        }
    }

    fn get_metadata(&self) -> Option<T> {
        Some(self.token_kind)
    }

    fn is_accept(&self) -> bool {
        self.is_accept
    }
}























/*
// This is what we might want for Tokens, but what about in the
// abstract case?
enum Result<T> {
    Failure,
    Success(T),
    Consuming
}

trait DFA<T> {
    fn consume(T) -> Result<T>;
}

// Or maybe an Enum?
enum State {
    Start, // Needed? Won't this be a field of DFA?
    Transition,
    Accept
}

// Input: chars, Output: Tokens
struct DFA<In, State, Result> {
    input: In,
    state: State
    output: V,
    current_node: mut ~Node,
    // Updates self.current_node, and returns whether the input matched
    // If match, we can add the input to the collected
    a: fn(&self, U) -> bool,
    update: fn(In) -> State,
    ret: fn(State) -> Result
}

trait Accepts<T> {
    fn accepts(T) -> bool
}

trait Node<T> {

}

struct Node<T> {
    // node to transition to if success
    success_node: ~Node,
    // node to transition to on failure
    fail_node: ~Node,
    accepts: fn (T) -> bool,
    is_accept_state: bool
}

impl Node {
    // next is either successNode or failNode?
    fn transition(&self, next: ~Node) -> ~Node {
        // Modify next then return it
        next
    }
}
*/
