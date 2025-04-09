use std::collections::HashMap;

fn handle_maps(){
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    access_map_values(&scores);
}

fn access_map_values(&scores: HashMap<String, i32>) -> i32{

    let team_name = String::from("Blue");
    let score = *scores.get(&team_name).copied().unwrap_or(0);
    // The get method returns an Option<&V>; if there’s no value for that key in the hash map, 
    // get will return None. This program handles the Option by calling copied to get an Option<i32> 
    // rather than an Option<&i32>, then unwrap_or to set score to zero if scores doesn’t have an entry for the key.

    println!("The score for {} is {}", team_name, score);
    score
}

fn itterate_over_map(&scores: HashMap<String, i32>) {
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

fn update_map(){
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");     // This code will print {"Blue": 25}. The original value of 10 has been overwritten.


    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // The or_insert method on Entry is defined to return a mutable reference to the 
    // value for the corresponding Entry key if that key exists, and if not, it inserts the parameter 
    // as the new value for this key and returns a mutable reference to the new value.
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);       // The value for the key "Blue" is not changed because it already has a value 
                                            // (entry returned value thus .or_insert() isn't called -> implies return of the scores.entry is an Option<T> type).

    println!("{scores:?}");


    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}