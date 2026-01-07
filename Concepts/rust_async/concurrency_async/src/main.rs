use std::pin::{Pin, pin};
use std::thread;
use std::time::{Duration, Instant};

use trpl::Either;

fn main() {
    trpl::run(async {
        trpl::spawn_task(async {
            for i in 0..5 {
                println!("Hi {i} from the first task ");
                trpl::sleep(Duration::from_millis(10)).await;
            }
        });

        for i in 0..10 {
            println!("Hi {i} from the second task ");
            trpl::sleep(Duration::from_millis(10)).await;
        }

        //
        let (tx, mut rx) = trpl::channel();
        let message = String::from("Teri gaa-nd maarunga");
        tx.send(message).unwrap();

        let received = rx.recv().await.unwrap();
        println!("Message Received {}", received);

        //
        let (tx1, mut rx1) = trpl::channel();
        let tx2 = tx1.clone();
        let sending_message = pin!(async move {
            let messages = vec![
                String::from("Hi"),
                String::from("From"),
                String::from("The"),
                String::from("Future"),
            ];

            for message in messages {
                let message_clone = message.clone();
                tx2.send(message).unwrap();
                println!("Message send : {}", message_clone);
                trpl::sleep(Duration::from_millis(500)).await
            }
        });

        let receiver = pin!(async {
            while let Some(received) = rx1.recv().await {
                println!("Message received : {}", received);
            }
        });

        let sending_message1 = pin!(async move {
            let messages = vec![
                String::from("Maa"),
                String::from("From"),
                String::from("Bho"),
                String::from("AAAAAAHGGGGGGGG!!!"),
            ];

            for message in messages {
                let message_clone = message.clone();
                tx1.send(message).unwrap();
                println!("Message send : {}", message_clone);
                trpl::sleep(Duration::from_millis(500)).await
            }
        });

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> =
            vec![sending_message, receiver, sending_message1];
        trpl::join_all(futures).await;

        //

        let a = async {
            println!("a started ---");
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 40);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("a finished ---");
        };
        let b = async {
            println!("b started ---");
            trpl::yield_now().await;
            slow("b", 50);
            trpl::yield_now().await;
            slow("b", 80);
            trpl::yield_now().await;
            slow("b", 375);
            trpl::yield_now().await;
            println!("b finished ---");
        };

        trpl::race(b, a).await;

        // Benchmark
        let one_ms = Duration::from_nanos(1);
        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::sleep(one_ms).await;
            }
        }
        .await;
        let time = Instant::now() - start;

        println!("'Sleep' version completed in {}", time.as_secs_f32());

        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::yield_now().await;
            }
        }
        .await;
        let time = Instant::now() - start;
        println!("'Yeild' version completed in {}", time.as_secs_f32());

        // Timeout

        let super_slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "finally finished"
        };

        match timeout(super_slow, Duration::from_secs(2)).await {
            Ok(result) => println!("Succeeded with {result}"),
            Err(duration) => println!(" Failed after {} seconds ", duration.as_secs()),
        }
    })
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("{name} worked for {ms}");
}
async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(future_output) => Ok(future_output),
        Either::Right(_) => Err(max_time),
    }
}
