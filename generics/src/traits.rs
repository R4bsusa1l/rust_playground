pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}



// -.-.-.-.-.-.-.-.-.-.-.-.-

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// implement of the Summary trait for the Type

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}


// -.-.-.-.-.-.-.-.-.-
fn make_tweet() -> impl Summary {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    tweet
}

println!("1 new tweet: {}", tweet.summarize());


pub fn notify(item: &impl Summary) {    // This parameter accepts any type that implements the specified trait
    println!("Breaking news! {}", item.summarize());
}

// trait bounds 

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// these two examles are equivalent

// the difference is that the first one is more readable and the second one is more flexible -> 
// notify1 takes parameters of potentiall different types that implement the same trait 
// while notify2 striclty requires the same type(that implements the summary trait) for both parameters

pub fn notify1(item1: &impl Summary, item2: &impl Summary);
pub fn notify2<T: Summary>(item1: &T, item2: &T);

// syntax for multiple trait bounds (with the + operator)
pub fn notify(item: &(impl Summary + Display));
pub fn notify<T: Summary + Display>(item: &T);


//instead of this:      -> trait with where so the method signature is more readable
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

//we can use a where clause, like this:
    
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{}