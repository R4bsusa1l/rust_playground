fn handle_strings(){
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    // This line creates a new, empty string called s, into which we can then load data. Often, we’ll have some initial data with which we want to start the string. 
    // For that, we use the to_string method, which is available on any type that implements the Display trait, as string literals do. Listing 8-12 shows two examples.

    let s = String::from("initial contents"); -> // String::from is a function that creates a String from a string literal. This is the most common way to create a String.    
    // does the same as .to_string() but is more explicit about the conversion.

    let mut s = String::from("foo");
    s.push_str("bar");  // append a string slice to a String (s must be mutable)

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');    // s will contain 'lol'

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // The + operator takes ownership of s1 and appends a copy of s2 to it. the + operator uses the add method; signature: fn add(self, s: &str) -> String {
    // -> the seccond string mus be a reference. 

    // But wait—the type of &s2 is &String, not &str, as specified in the second parameter to add
    // The reason we’re able to use &s2 in the call to add is that the compiler can coerce the &String argument into a &str. When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..]


    // Format macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    // Slices

    // Here, s will be a &str that contains the first four bytes of the string. Earlier, we mentioned that each of these characters was two bytes, which means s will be Зд.
    let hello = "Здравствуйте";
    let s = &hello[0..4];

    // You should use caution when creating string slices with ranges, because doing so can crash your program.
}

fn itterate_over_string(){
    for c in "Зд".chars() {     // will return З; д
        println!("{c}");
    }

    for b in "Зд".bytes() {     // will return 208; 151; 208; 180       // keep in mind valid Unicode scalar values may be made up of more than one byte
        println!("{b}");
    }
}