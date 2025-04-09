fn pub vectors(){
    let v: Vec<i32> = Vec::new();   // type annotation i32 necessary since no values are assigned at creation
    // Vec<T> is a growable array and can hold any type T

    let v = vec![1, 2, 3]; // vec! macro creates a vector and infers the type

    let mut v = Vet::new();

    v.push(5);      // update the vector (must be a mutable vector)
    v.push(6);
    v.push(7);
    v.push(8);
}

fn two_way_to_retrieve_values(){
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

}

fn itterate_over_vector(){¨
    let v = vec![100, 32, 57];      //immutable itteration
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];      //mutable itteration
    for i in &mut v {
        *i += 50;
    }
}