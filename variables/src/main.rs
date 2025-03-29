// Variables and Mutability
// Variables are immutable by default in Rust. To make a variable mutable, we use the `mut` keyword.
// This means that we can change the value of a variable after it has been declared.
// However, we cannot change the type of a variable after it has been declared.
// For example, we can change the value of a mutable variable, but we cannot change the type of a variable.
// In Rust, we can also shadow a variable by declaring a new variable with the same name.
// This means that we can create a new variable with the same name as an existing variable, and the new variable will shadow the old variable.
// This is useful when we want to change the value of a variable, but we don't want to change the type of the variable.
// In this example, we will create a mutable variable, change its value, and then shadow it with a new variable.
// We will also create a new variable with the same name as an existing variable, and then shadow it with a new variable.

mod loops;


fn main() {
    
    let x = 5;
    /* 
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    */
    let mode = "run";

    if mode == "debug"{
        shadow();
        floating_point();
        numeric_operators();
        bool_type();
        tuples();
        arrays();
        print_labeled_measurement(x, 'm');
        expressions_vs_statements();
        loops::looper();
    }
    if mode == "run"{
        use_functions();
        loops::loop_with_returnvalue();
        loops::selective_loop();
        loops::while_loop();
        loops::for_loop();
    }
}

/*
Length	Signed	Unsigned
8-bit	i8	    u8
16-bit	i16	    u16
32-bit	i32	    u32
64-bit	i64	    u64
128-bit	i128	u128
arch	isize	usize
*/

fn shadow() {
    let x = 5;

    let x = x + 1;
    println!("The value of x is: {x}");
    let hello = "hello";
    println!("The value of hello is: {hello}");
    
    {
        let hello = hello.len();
        println!("The value of hello is: {hello}");
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn floating_point() {
    let x = 2.0; // f64
    print!("The value of x is: {x}");

    let y: f32 = 3.0; // f32
    print!("The value of y is: {y}");
}

fn numeric_operators() {
    // addition
    let sum = 5 + 10;
    print!("The sum of 5 and 10 is: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    print!("The difference of 95.5 and 4.3 is: {difference}");

    // multiplication
    let product = 4 * 30;
    print!("The product of 4 and 30 is: {product}");

    // division
    let quotient = 56.7 / 32.2;
    print!("The quotient of 56.7 and 32.2 is: {quotient}");
    let truncated = -5 / 3; // Results in -1
    print!("The truncated value of -5 / 3 is: {truncated}");

    // remainder
    let remainder = 43 % 5;
    print!("The remainder of 43 / 5 is: {remainder}");
}

fn bool_type() {
    let t = true;
    let f: bool = false; // with explicit type annotation
    if t{
        println!("The value of t is: {t}");
    } else {
        println!("The value of t is: {t}");
    }
    if f{
        println!("The value of f is: {f}");
    } else {
        println!("The value of f is: {f}");
    }
}

// immutable collection of variable types

fn tuples() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x, y, z is: {x}, {y}, {z}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    print!("five_hundred: {five_hundred}, six_point_four: {six_point_four}, one: {one}");
}

// Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.

fn arrays() {
    let a: [u8; 5] = [1, 2, 3, 4, 5];
    // let a = [3; 5]; == let a = [3, 3, 3, 3, 3];

    let first: u8 = a[0];
    print!("The first element of the array is: {first}");
    let second: u8 = a[1];
    print!("The second element of the array is: {second}");
}

// In function signatures, you must declare the type of each parameter. 
// This is a deliberate decision in Rust’s design: requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere in the code to figure out what type you mean

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn expressions_vs_statements() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

}

// funtction with return type signed 32 bit

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn use_functions(){
    print!("{}", plus_one(five()));
}
