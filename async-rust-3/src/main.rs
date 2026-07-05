use std::time::Duration;
use std::thread;

use trpl::StreamExt;


fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();

        let tx1_fut = async move {
                let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap(); //send is not blocking so need to await
                trpl::sleep(Duration::from_millis(1000)).await;
                println!("printing");
            }
        };

        let tx_fut = async move {
                let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap(); //send is not blocking so need to await
                trpl::sleep(Duration::from_millis(1500)).await;
                println!("printing");
            }
        };
        
        let rx_fut = async move {
            while let Some(val) = rx.recv().await {
                println!("received: {}", val);
            }
        };

        trpl::join!(tx1_fut, tx_fut, rx_fut);

    });

    trpl::block_on(async{
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            slow("a", 10);
            slow("a", 20);
            trpl::sleep(Duration::from_millis(50)).await; //non blocking
            println!("'a' finished");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            slow("b", 10);
            slow("b", 15);
            trpl::sleep(Duration::from_millis(50)).await; //non blocking
            println!("'b' finished");
        };

        trpl::select(a, b).await;
    });

    println!("=====");

    trpl::block_on(async{
        let one_ms = Duration::from_millis(1);
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::sleep(one_ms).await;
            slow("a", 10);
            trpl::sleep(one_ms).await;
            slow("a", 20);
            trpl::sleep(one_ms).await;
            println!("'a' finished");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::sleep(one_ms).await;
            slow("b", 10);
            trpl::sleep(one_ms).await;
            slow("b", 15);
            trpl::sleep(one_ms).await;
            slow("b", 350);
            trpl::sleep(one_ms).await;
            println!("'b' finished");
        };

        trpl::select(a, b).await;

        let values = [1,3,5,7,9,11];
        let iter = values.iter().map(|x| x * 2);
        let mut stream = trpl::stream_from_iter(iter);
        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    });

}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms)); //blocking sleep
    println!("'{name}' ran for {ms} ms");
}
