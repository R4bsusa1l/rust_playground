
// endless loop can be interruped in terminal with ctrl+c
// or in the code with a break statement
// or by using a return statement
// or by using a panic statement
// or by using a loop statement
// or by using a continue statement

pub fn looper(){
    loop{
        println!("looping");
    }
}

pub fn loop_with_returnvalue() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

pub fn selective_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

pub fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

pub fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }


    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
    
}