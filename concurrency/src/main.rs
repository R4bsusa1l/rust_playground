use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
    alternation_with_wait_for_execution();
}

use std::thread;
use std::time::Duration;

// if we place the handle.join() at the beginning of the main thread: 
// The main thread will wait for the spawned thread to finish and then run its for loop, 
// so the output won’t be interleaved anymore, as shown here:
/* 
        hi number 1 from the spawned thread!
        hi number 2 from the spawned thread!
        hi number 3 from the spawned thread!
        hi number 4 from the spawned thread!
        hi number 5 from the spawned thread!
        hi number 6 from the spawned thread!
        hi number 7 from the spawned thread!
        hi number 8 from the spawned thread!
        hi number 9 from the spawned thread!
        hi number 1 from the main thread!
        hi number 2 from the main thread!
        hi number 3 from the main thread!
        hi number 4 from the main thread!
*/
fn alternation_with_wait_for_execution() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    //handle.join().unwrap();   -> this will block the main thread until the spawned thread finishes as in example above

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

use std::thread;

fn use_thread_with_closure_values() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}

