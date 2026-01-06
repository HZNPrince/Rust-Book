use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} spawned from the spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} spawned from the main thread");
        thread::sleep(Duration::from_millis(1));
    }

    let vec1 = vec![1, 2, 3];

    thread::spawn(move || {
        println!("Printing Vector from main thread : {:?}", vec1);
    });
    println!("asnf");

    handle.join().unwrap();
}
