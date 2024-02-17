use std::thread::{self, sleep};
fn main() {
    println!("Program started");
    let thread_one = thread::spawn(|| {
        sleep(std::time::Duration::from_millis(200));
        println!("The long running task finished last");
    });
    let thread_two = thread::spawn(|| {
        println!("We can chain callbacks");
        let thread_three = thread::spawn(|| {
            sleep(std::time::Duration::from_millis(50));
            println!("... like this");
        });
        thread_three.join().unwrap();
    });
    println!("The tasks run concurrently");
    thread_one.join().unwrap();
    thread_two.join().unwrap();
}
