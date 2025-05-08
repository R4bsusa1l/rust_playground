enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn create_depending_lists() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    // -.-.-.-.-.-.-.-.-.-.-.-.-

    let a1 = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a1));
    let b1 = Cons(3, Rc::clone(&a1));
    println!("count after creating b = {}", Rc::strong_count(&a1));
    {
        let c1 = Cons(4, Rc::clone(&a1));
        println!("count after creating c = {}", Rc::strong_count(&a1));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a1));
}