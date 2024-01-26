use tokio::task;

#[tokio::main]
async fn main() {
    let count = 100000000;
    let mut handles = Vec::with_capacity(count);

    for i in (0..count).rev() {
        let handle = task::spawn(async move {
            // 这里执行异步任务
            println!("Rust count: {}", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }
}
