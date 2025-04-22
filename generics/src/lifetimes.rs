fn some_scope() {     // visualize the scope of the variables
    let r;          // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;   // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {r}");   //          |
}                         // ---------+


/*// Lifetime annotations are used to specify the scope of a reference
    &i32        // a reference
    &'a i32     // a reference with an explicit lifetime
    &'a mut i32 // a mutable reference with an explicit lifetime
*/

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_string() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result: &str = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}"); // result valid until this point because the shorter of the two lifetimes in 
    }
    /* // this wouldnt work however: 
    fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {result}");
}
    */
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn do_smth() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}