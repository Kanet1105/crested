/// Rust 동기 처리 library

// #[test]
// fn getting_pointing_address() {
//     let x = "hell";
//     let y = x.to_owned();

//     let x_addr = x.as_ptr() as usize;
//     let y_addr = y.as_ptr() as usize;

//     let x_addr_bytes: &[u8] = unsafe{std::slice::from_raw_parts(x_addr as *const u8, 10)};
//     let y_addr_bytes: &[u8] = unsafe{std::slice::from_raw_parts(y_addr as *const u8, 10)};

//     let x_raw: &'static str = std::str::from_utf8(x_addr_bytes).expect("valid UTF");
//     let y_raw: &'static str = std::str::from_utf8(y_addr_bytes).expect("valid UTF");
    
//     println!("x: &x={:p}, x_addr={:p}, x_raw={:p}", &x, &x_addr, &x_raw);
//     println!("x: &y={:p}, y_addr={:p}, y_raw={:p}", &y, &y_addr, &y_raw);
// }


// 3.8.1 Mutex
#[test]
fn p122() {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use rand::random;

    fn some_func(lock: Arc<Mutex<u64>>) {
        loop {
            // lock 을 하지 않으면 Mutex type 내 value 참조 불가
            let mut val = lock.lock().unwrap();
            *val += 1;
            println!("{}", *val);
        }
    }

    let lock0 = Arc::new(Mutex::new(random::<u8>()));
    // Mutex 용 변수를 저장하는 thread 가 multithreading 접근에서 safe 하도록 Arc Mutex 스마트 포인터 생성 (초기값 0)

    let lock0_addr = &lock0;
    let lock0_bytes: &[u8] = unsafe{std::slice::from_raw_parts(&lock0 as *const u8, 10)};
    let lock0_pointing_addr: &'static str = std::str::from_utf8(lock0_bytes).expect("valid UTF");

    // 참조 counter 가 증가될 뿐이며, value 에 대한 memcpy 가 발생하지 않음.
    let lock1 = lock0.clone();
    
    println!("lock 0: (pointer location: {:p}, raw pointer address: {:p}, value :{:?})", &lock0, &*lock0 as *const u32, *lock0);
    println!("lock 1: (pointer location: {:p}, raw pointer address: {:p}, value :{:?})", &lock1, &*lock1 as *const u32, *lock1);
    // 동일한 value location 을 가리킴.
    


}


 
// 3.8.2 조건 변수 Condvar
// 조건 변수 : (p105) 어떤 조건을 만족하지 않는 동안에는 프로세스를 대기상태로 두고
//             조건이 만족되면 대기중인 프로세스를 실행. 
//             -> "대기" 의 의미 
//                 .. it consumes no CPU time while waiting for an event to occur. 
#[test]
fn p124 () {
    use std::sync::{Arc, Mutex, Condvar};
    use std::thread;

    fn child (id: u64, p: Arc<(Mutex<bool>, Condvar)>) {
        let &(ref lock, ref cvar) = &*p;

        // Mutex lock 실행
        let mut started = lock.lock().unwrap();
        while !*started {   // Mutex 안의 공유 변수가 false 인 동안 루프 
            started = cvar.wait(started).unwrap(); // 알림이 있을 때까지 대기
        }
        // 아래와 같이 wait 가능
        // cvar.wait_while(started, |started| !*started).unwrap();

        println!("child {}", id);
    }

    fn parent(p: Arc<(Mutex<bool>, Condvar)>) { // 알림 thread 용 함수??
        let &(ref lock, ref cvar) = &*p;

        // 락을 수행한뒤, 공유 변수값을 true 로 설정하고 알림.
        let mut started = lock.lock().unwrap();
        *started = true;    // 공유변수 업데이트
        cvar.notify_all();  // 알림 (wakes up all threads on this condvar)
        println!("parent");
    }

    let pair0 = Arc::new((Mutex::new(false), Condvar::new()));
    let pair1 = pair0.clone();
    let pair2 = pair0.clone();

    // condvar 이 false 로 최초 설정되어 있으므로, c0 과 c1 은 p 가 실행되어
    // condvar 을 true 로 바꿔주기 전까지 대기한다. 
    let c0 = thread::spawn(move || {child(0, pair0)});
    let c1 = thread::spawn(move || {child(1, pair1)});
    let p = thread::spawn(move || {parent(pair2)});

    c0.join().unwrap();
    c1.join().unwrap();
    p.join().unwrap();

}

#[test]
fn p126() {
    use std::sync::RwLock;

    let lock = RwLock::new(10);
    {
        let v1 = lock.read().unwrap();
        let v2 = lock.read().unwrap();
        println!("v1 = {}", v1);
        println!("v2 = {}", v2);
    }
    {
        let mut v = lock.write().unwrap();
        *v = 7;
        println!("v = {}", v);
    }
}

// 3.8.4 Barrier sync
#[test]
fn p127() {
    use std::sync::{Arc, Barrier};
    use std::{thread, time};

    let one_ms = time::Duration::from_millis(1);

    // thread handler 를 저장하는 Vec
    // 나중 join 을 수행하기 위해 thread handler 를 보존하는 Vec 을 정의
    // 해당 Vec type 은 동적 배열 객체를 다루는 데이터 컨테이너 이다. 
    let mut v = Vec::new();

    // 10 threads 만큼의 barrier sync 를 Arc 로 감쌈.
    let barrier = Arc::new(Barrier::new(3));

    // 10 threads 실행
    for _ in 0..12 {
        let b = barrier.clone();
        let th = thread::spawn(move || {
            // thread::sleep(one_ms); // option1
            println!("before wait");        
            b.wait();
            println!("finished barrier");
        });
        v.push(th);
        println!("{:?}", v);
        thread::sleep(one_ms); // option0
    }
    //                    v--------------- 반복 ---------------ㄱ
    //  main thd  : thread spawn -> v.push(th) -> print v  -> sleep  
    //                    L->  print "before wait" -> wait (3개가 쌓일때까지..)
    //                                                   ㄴ-> print "finished.." (3개한꺼번에) -> join                                

    for th in v {
        th.join().unwrap();
    } 
}

// 3.8.5 semaphore (Rust 에서 표준으로 제공하고 있지 않으므로 해당 자료 구조 구성)
#[test]
fn p128() {
    use std::sync::{Condvar, Mutex};

    // 세마포어용 타입
    pub struct Semaphore {
        mutex: Mutex<isize>,
        cond: Condvar,
        max: isize,     // lock 을 획득할 수 있는 최대 process 수
    }

    impl Semaphore {
        pub fn new(max: isize) -> Self {
            Semaphore {
                mutex: Mutex::new(0),
                cond: Condvar::new(),
                max,
            }
        }

        pub fn wait(&self) {
            
            let mut cnt = self.mutex.lock().unwrap();
            while *cnt >= self.max {    // 카운터가 최댓값 이상이면 대기
                cnt = self.cond.wait(cnt).unwrap();
            }
            *cnt += 1;  // 접근하여 lock 을 한 process 의 숫자 count
        }

        pub fn post(&self) {
            // 카운터 감소
            let mut cnt = self.mutex.lock().unwrap();
            *cnt -= 1;
            if *cnt <= self.max {   //  최댓값 이하로 counter 가 떨어졌을 때, 대기중인 process 중 하나 wake up d알림
                self.cond.notify_one();
            }
        }
    }

    // 위에서 작성한 semaphore test code
    use std::sync::atomic::{AtomicUsize, Ordering}; // memory ordering : https://int-i.github.io/rust/2022-01-15/memory-ordering/
    use std::sync::Arc;

    const NUM_LOOP: usize = 100_000;
    const NUM_THREADS: usize = 8;
    const SEM_NUM: isize = 4;

    static mut CNT: AtomicUsize = AtomicUsize::new(0);

    ma();

    fn ma() {
        let mut v = Vec::new();
        //SEM_NUM 만큼 동시 실행 가능한 semaphore
        let sem = Arc::new(Semaphore::new(SEM_NUM));

        for i in 0..NUM_THREADS {
            let s = sem.clone();
            let t = std::thread::spawn(move || {
                for _ in 0..NUM_LOOP {
                    s.wait();

                    // 아토믹하게 증가 및 감소
                    unsafe { CNT.fetch_add(1, Ordering::SeqCst)}; // mutable static 변수 값을 수정하기 위해 unsafe 사용 https://m.blog.naver.com/sssang97/221660436556
                    // SeqCst는 메모리 명령의 순차적 일관성(Sequential consistency)을 보장하는 방식입니다.
                    // 즉, 메모리 재배치 없이 코드에 작성된 그대로 프로그램을 컴파일하는 것과 동일한 결과가 나오도록 하라는 것입니다.
                    // 그만큼 최적화가 제한되기 때문에 최대한 지양하는 방식이기도 합니다.
                    let n = unsafe {CNT.load(Ordering::SeqCst)}; 
                    println!("semaphore: i = {}, CNT ={}", i, n);
                    assert!((n as isize) <= SEM_NUM);
                    unsafe{CNT.fetch_sub(1, Ordering::SeqCst)};

                    s.post();
                }
            });
            v.push(t);

        }
        for t in v {
            t.join().unwrap();
        }
    }
}


//-------------------------------------------------------------
fn main() {
    println!("Hello, world!");
}



fn main() {
    let x = "hell".to_string();
    let y = &x;
    let z = x.clone();

    let x_addr = x.as_ptr() as usize;
    let y_addr = y.as_ptr() as usize;
    let z_addr = z.as_ptr() as usize;

    let x_addr_bytes: &[u8] = unsafe{std::slice::from_raw_parts(x_addr as *const u8, 10)};
    let y_addr_bytes: &[u8] = unsafe{std::slice::from_raw_parts(y_addr as *const u8, 10)};
    let z_addr_bytes: &[u8] = unsafe{std::slice::from_raw_parts(z_addr as *const u8, 10)};

    let x_raw: &'static str = std::str::from_utf8(x_addr_bytes).expect("valid UTF");
    let y_raw: &'static str = std::str::from_utf8(y_addr_bytes).expect("valid UTF");
    let z_raw: &'static str = std::str::from_utf8(z_addr_bytes).expect("valid UTF");
    
    println!("x: &x={:p}, x_addr={:p}, x_raw={:p}", &x, &x_addr, &x_raw);
    println!("y: &y={:p}, y_addr={:p}, y_raw={:p}", &y, &y_addr, &y_raw);
    println!("z: &z={:p}, z_addr={:p}, z_raw={:p}", &z, &z_addr, &z_raw);
}
