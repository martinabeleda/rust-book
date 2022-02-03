use std::thread;
use std::time::Duration;

fn main() {

    // Basic threading
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    for i in 1..5 {
        println!("Hi {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    // Using move closures with threads
    // The move closure allows you to use data from one
    // thread in another. This means that the spawned
    // thread takes ownership of the vector v.
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move ||{
        println!("Here's a vector {:?}", v);
    });
    // If we uncomment this line, we get a compiler error
    // since the main thread no longer has ownership of v
    // drop(v);
    handle.join().unwrap();
}
