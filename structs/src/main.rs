struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main (){
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com"); // the entire instance of user1 must be mutable to mutate even one value

}

// function returns instance of this struct
// this function takes ownership of the email and username strings that then fall out of scope
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user_field_init(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// build a new instance of User from an existing instance
fn new_user_from_user() {
    // --snip--

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}

// can also use struct update syntax to create a new instance of User

fn update_user() {
    // --snip--
// implies move from all fields of user1 except the ones explicitly set. this means user1.username is no longer valid
    let user2 = User {          
        email: String::from("another@example.com"),
        ..user1     // this will copy remaining values not explicitly set from user1 into user2
    };
}

fn tuple_struct(){
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    fn (3_int_object) {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    }
}

//-.-.-.-.-.-.-.-

struct Rectangle {
    width: u32,
    height: u32,
}


fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

/*
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
*/      // -> could also be:

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}