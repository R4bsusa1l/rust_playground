use crate::List::{Cons, Nil};
use std::ops::Deref;


struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}


// -.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-



struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// reccursive data structure
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>), // this instead of Cons(i32, List); Cons(i32, List) would be infinitely large
    Nil,                    // since sizes on the stack must be known at compile time we changed from storing the list on the stack 
                                // to storing a reference to the list (pointer)
}

fn main1() {
    //let list = Cons(1, Cons(2, Cons(3, Nil)));          // alledgedly adaptation from lisp
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))); // we need to box the inner lists
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    use_box();

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

fn use_box() {
    let b = Box::new(5);        // Box::new creates a heap-allocated integer
    println!("b = {b}");
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

/*
From &T to &U when T: Deref<Target=U>
From &mut T to &mut U when T: DerefMut<Target=U>
From &mut T to &U when T: Deref<Target=U>
*/