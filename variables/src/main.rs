const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    shadow();
}

fn shadow() {
    let x = 5;

    let x = x + 1;
    println!("The value of x is: {x}");
    let hello = "hello";
    println!("The value of hello is: {hello}");
    
    {
        let hello = spaces.len();
        println
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}