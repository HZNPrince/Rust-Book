use std::{fmt::format, pin::pin, time::Duration};

use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let values = 1..10;
        let iter = values.map(|n| n * 2);
        let stream = trpl::stream_from_iter(iter);

        let mut filtered = stream.filter(|n| n % 3 == 0 || n % 5 == 0);

        while let Some(n) = filtered.next().await {
            println!("The value was : {n}");
        }

        //

        let messages = get_message().timeout(Duration::from_millis(200));
        let interval = get_interval()
            .map(|count| format!("Count : {count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));
        let merged = messages.merge(interval).take(20);
        let mut stream = pin!(merged);

        while let Some(message) = stream.next().await {
            match message {
                Ok(message) => println!("{message}"),
                Err(reason) => println!("{reason}"),
            }
        }
    })
}

fn get_message() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_err) = tx.send(format!("Message : '{message}'")) {
                eprintln!("Cannot send message '{message}': {send_err}");
                break;
            }
        }
    });
    ReceiverStream::new(rx)
}

fn get_interval() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;

        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            if let Err(send_err) = tx.send(count) {
                eprintln!("Cannot send message '{message}': {send_err}")
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}
