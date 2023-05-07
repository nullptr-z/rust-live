use tokio::select;
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    let mut interval1 = time::interval(Duration::from_secs(1));
    let mut interval2 = time::interval(Duration::from_secs(2));

    loop {
        select! {
            // _ret 是一个变量，用于存储 interval1.tick() 异步操作的结果。
            _ret = interval1.tick() => {
                println!("【 _ret 】==> {:?}", _ret);
                println!("Interval 1 tick");
            }
            _ = interval2.tick() => {
                println!("Interval 2 tick");
            }
        }
    }
}
