// fn main() {
//     println!("Hello, world!");
// }

use tokio::sync::oneshot;

// tx : 미래 언젠가의 시점에서 값이 결정되는 매개변수를 받음
async fn set_val_later(tx: oneshot::Sender<i32>) {
    let ten_secs = std::time::Duration::from_secs(10);
    tokio::time::sleep(ten_secs).await;
    if let Err(_) = tx.send(100) {  // 보낸 tx 가 없으므로 sleep 이 끝나면 100을 보낸다.
        println!("failed to send");
    }
}

#[tokio::main]
pub async fn main() {
    let (tx, rx) = oneshot::channel();
    tokio::spawn(set_val_later(tx));   // argument tx 의 값은 아직 정해지지 않았으나 future type 으로 spawn 가능

    match rx.await {    // rx 값을 받을 때까지 대기
        Ok(n) => {
            println!("n = {}", n);
        } 
        Err(e) => {
            println!("failed to receive: {}", e);
            return;
        }
    }
}