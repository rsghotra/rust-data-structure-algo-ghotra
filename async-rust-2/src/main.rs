use std::{thread, time::Duration};
use trpl::{Either, Html};
fn main() {
    let (tx, mut rx) = trpl::channel();

    thread::spawn(move || {
        for i in 1..11 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    trpl::block_on(async {
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
    });


    let args: Vec<String> =  std::env::args().collect();

    trpl::block_on(async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);


        let (url, maybe_title) =
            match trpl::select(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };
            println!("{url} returned first");

            match maybe_title {
                Some(title) => println!("Its page title was: {title}"),
                None => println!("It had no title."),
            }
    });

    //using handle.await
    trpl::block_on(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;

            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        handle.await.unwrap();
        
    });

    println!("=====");

    //using join(fut1, fut2)
    trpl::block_on(async {
        let handle = trpl::spawn_task(async {
                let fut1 = async {
                for i in 1..10 {
                    println!("hi number {i} from the first task!");
                    trpl::sleep(Duration::from_millis(500)).await;

                }
            };

            let fut2 = async {
                for i in 1..5 {
                    println!("hi number {i} from the second task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            trpl::join(fut1, fut2).await;
        });
        handle.await.unwrap();
    });

    println!("#######");

}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text).select_first("title").map(|title| title.inner_html());
    (url, title)
}
