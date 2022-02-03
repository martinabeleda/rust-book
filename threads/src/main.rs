use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Hi {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..15 {
        println!("Hi {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
}
