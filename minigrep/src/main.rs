use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];                   // we start at 1 because 0 is the name of the program (eg. /home/user/minigrep)
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");
} 