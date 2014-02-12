// dfa.rs
// Binary DFAs

fn main() {}

// add functions for querying state
// make a trait?
trait DFA<T> {
    consume: fn(T)
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
