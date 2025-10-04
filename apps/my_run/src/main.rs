use tokio::sync::Mutex;
use std::sync::Arc;


#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let counter = Arc::clone(&counter);

        let handle = tokio::spawn(async move {
            let mut num = counter.lock().await;
            *num += 1;
            println!("Task {}: {}", i, *num);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let result = *counter.lock().await;
    println!("Result: {}", result); // Result: 5
}