use std::fs::File;
use std::io::ErrorKind;


mod propagation; 

//The main function may return any types that implement the 
//std::process::Termination trait, which contains a function report that returns an ExitCode
fn main() -> Result<(), std::io::Error> {
    let _greeting_file = File::open("hello.txt")?;

    Ok(())
}

/*
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/panic`
thread 'main' panicked at src/main.rs:2:5:
crash and burn
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
*/


// handling errors with Result

#[derive()]
enum _MyResult<T, E> {
    Ok(T),
    Err(E),
}

fn _read_file() {
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}

// handling errors with match to divise error types
fn _match_error_kind() {
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
}

// match_error_kind with closure (same func written in different way)
// this is a more idiomatic way to handle errors in Rust
fn _error_closure() {
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}

// small error with unwrap

fn _error_unwrap() {
    let _greeting_file = File::open("hello.txt").unwrap();
}

/*  Error message if "hello.txt" does not exist
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/panic`
thread 'main' panicked at src/main.rs:4:49:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
*/

fn _use_expect() {
    let _greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}