pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: usize) -> usize {
    a + 2
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    fn another() {
        panic!("Make this test fail");
    }
    
    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

// Some command line options go to cargo test, and some go to the resultant test binary. 
// To separate these two types of arguments, you list the arguments that go to cargo test 
// followed by the separator -- and then the ones that go to the test binary. 
// Running cargo test --help displays the options you can use with cargo test, 
// and running cargo test -- --help displays the options you can use after the separator

// $ cargo test -- --test-threads=1     -> this will run all tests in a single thread   -> TE is by default multithreaded
// this means that tests that interfere with each other can cause problems. Thus we make sure all tests are 
// executed sequentially.
// $ cargo test -- --ignored           -> this will run all tests that are marked as ignored
// $ cargo test -- --show-output       -> this will show the output of all tests    
// function output of println!() and eprintln!() for example is then visible in the test-report
// $ cargo test it_works                -> this will run only the test it_works
// $ cargo test it                      -> this will run the tests it_works and it_adds_two. 
// this is the standard way to specify 2 tests to run by name. 
// We take the common prefix of the test names and the compiler will run any test that matches the prefix.

// After #[test], we add the #[ignore] line to the test we want to exclude, in this case it_adds_two. 
// Now when we run our tests, it_works runs, but it_adds_two doesn’t
// cargo test it -- --include-ignored   -> will run both it_works and it_adds_two again as ignored tessts are