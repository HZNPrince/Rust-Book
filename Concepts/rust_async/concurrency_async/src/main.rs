use std::time::Duration;

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

        let (tx, mut rx) = trpl::channel();
        let message = String::from("Teri gaa-nd maarunga");
        tx.send(message).unwrap();

        let received = rx.recv().await.unwrap();
        println!("Message Received {}", received);
    })
}
