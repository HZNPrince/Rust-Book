use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    // First thread
    thread::spawn(move || {
        let val = vec![
            String::from("Hi"),
            String::from("From"),
            String::from("The"),
            String::from("Thread"),
        ];
        for text in val {
            thread::sleep(Duration::from_millis(1000));
            tx.send(text).unwrap();
        }
    });

    // Second Thread
    thread::spawn(move || {
        let messages = vec![
            String::from("More"),
            String::from("Messages"),
            String::from("For"),
            String::from("You"),
        ];
        for message in messages {
            thread::sleep(Duration::from_millis(1000));
            tx1.send(message).unwrap();
        }
    });

    // Receiver
    for received in rx {
        println!("Got : {received}");
    }
}
