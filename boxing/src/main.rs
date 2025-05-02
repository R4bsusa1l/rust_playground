use crate::List::{Cons, Nil};

// reccursive data structure
enum List {
    Cons(i32, Box<List>), // this instead of Cons(i32, List); Cons(i32, List) would be infinitely large
    Nil,                    // since sizes on the stack must be known at compile time we changed from storing the list on the stack 
                                // to storing a reference to the list (pointer)
}

fn main() {
    //let list = Cons(1, Cons(2, Cons(3, Nil)));          // alledgedly adaptation from lisp
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))); // we need to box the inner lists
}

fn use_box() {
    let b = Box::new(5);        // Box::new creates a heap-allocated integer
    println!("b = {b}");
}