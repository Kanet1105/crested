/// "false sharing" (추후 작업 필요)
/// (참고 : https://hwan-shell.tistory.com/230)
/// 
/// 
/// 
/// 
use std::time::SystemTime;

struct _PositiveBackend;

impl _PositiveBackend{
    fn compute(&self, number: u64) -> u64{
        number+1
    }
}

#[allow(dead_code)]
fn main() {
    let backend = Box::new(_PositiveBackend);
    let mut res= 0 as u64;
    let start_time = SystemTime::now();
    let total = 20_000_000 as u64;
    
    // our main loop
    for i in 0 .. total {
        res += backend.compute(i);
    }

    println!("Result: {}",res);
    println!("Elapsed_ms: {}", start_time.elapsed().unwrap().as_millis());
}

/// -----------------
/// 
pub fn eof() {}