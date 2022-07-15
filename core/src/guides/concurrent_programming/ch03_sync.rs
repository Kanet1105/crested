/// Rust 동기 처리 library

/// 3.8.1 Mutex
#[test]
fn p122() {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use rand::{random, thread_rng, Rng};

    fn some_func(th: &str, lock: Arc<Mutex<u64>>) {
        let mut checker: u64 = 0;
        
        while checker <= 5000 {
            // lock 을 하지 않으면 Mutex type 내 value 참조 불가
            let mut val = lock.lock().unwrap();
            let added = thread_rng().gen::<u8>();       
            *val += added as u64;       // 각 thread 가 각자 다르게 동작함을 보이기 위해 random 값으로 더함.
            println!("{}: {}", th, *val);
            checker = *val;
        }
    }

    let lock0 = Arc::new(Mutex::new(random::<u8>() as u64));
    // Mutex 용 변수를 저장하는 thread 가 multithreading 접근에서 safe 하도록 Arc Mutex 스마트 포인터 생

    // 참조 counter 가 증가될 뿐이며, value 에 대한 memcpy 가 발생하지 않음.
    let lock1 = lock0.clone();
    
    let c_lock0 = lock0.clone();
    let c_lock1 = lock1.clone();        
    { // cf. lock1 은 lock0 의 clone 인데.. value 의 copy 인가? pionter 의 추가인가?              
        println!("c_lock0 : location {:p}, value: {:?}", c_lock0, c_lock0); // c_lock0 : location 0x24564891e60, value: Mutex { data: 132, poisoned: false, .. }
        println!("c_lock1 : location {:p}, value: {:?}", c_lock1, c_lock1); // c_lock1 : location 0x24564891e60, value: Mutex { data: 132, poisoned: false, .. }
    } // -> clone 을 해도 memcpy 가 발생하지 않고, 단순히 해당 value 를 가리키는 pointer 만 추가됨.
    
    // 책에서도 내용은 클론되지 않는다고 나와 있다.
    // 아울러, 위의 println! 문에서도 동일 변수의 원형을 반복해서 썼는데도 에러 없이 실행된다. 
    // rust 에서 일반적 high-level model type 의 경우, ownership 때문에 이는 불가능 하다. 
    // 만약 GC 만 고려한다면, 스마트 포인터는 자체적으로 clone 된 variables 에 대해 counting 해서 관리하므로,
    // value 을 mutable 할 권한을 ownership 과 묶어 놓지 않아도 문제 없이 동작 할수 있을것으로 보인다. 
    // 그러나 이에 따른 다른 문제가 없는지... 잘 모르겠다.  

    // thread 생성 
    // closure 내로 onwership 이동
    let th0 = thread::spawn(move || {
        some_func("th0", lock0);
    });

    let th1 = thread::spawn(move|| {
        some_func("th1", lock1);
    });

    th0.join().unwrap();
    th1.join().unwrap();
}
 
/// 기본적인 critical session 에서의 lock algorithm 은 이미 lock 걸려 있는 data 에 접근할때,
/// 사용권한을 얻을때까지 무한 loop 형태로 시도하게 된다. 
/// 이렇게 resource 가 비는 것을 polling 으로 확인하는 방법을 spinlock 라고 한다. 
/// 그런나 해당 구조는 의미없는 cpu 작업이 발생한다. 
/// polling 에 대한 load 를 줄일 수는 없을까?

/// 3.8.2 조건 변수(condition variable) Condvar (1)
/// 조건 변수 : (p105) 어떤 조건을 만족하지 않는 동안에는 프로세스를 대기상태로 두고
///             조건이 만족되면 대기중인 프로세스를 실행. 
///             -> "대기" 의 의미 
///                 .. it consumes no CPU time while waiting for an event to occur. 
#[test]
fn p124 () {
    use std::sync::{Arc, Mutex, Condvar};
    use std::thread;

    fn child (id: u64, p: Arc<(Mutex<bool>, Condvar)>) {
        let &( ref lock, ref cvar) = &*p;   // p 의 ownership 을 유지 해야 하므로 ref 사용
                                                                    // ref keyword : tips03 참조
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
    // 따라서 p 가 완료되고 나서, c0, c1 (순서에 관계없이) 완료된다.
    let c0 = thread::spawn(move || {child(0, pair0)});
    let c1 = thread::spawn(move || {child(1, pair1)});
    let p = thread::spawn(move || {parent(pair2)});

    c0.join().unwrap();
    c1.join().unwrap();
    p.join().unwrap();

}

/// 3.8.2 조건 변수(condition variable) Condvar - timeout 
/// 위 예제에서 parent 작업이 끝날때까지 wait 하는 것을 일정 시간으로 한정할 경우
#[test]
fn p124plus () {
    use std::sync::{Arc, Mutex, Condvar};
    use std::{thread, time};

    fn t_child(wait_id: u64, p:Arc<(Mutex<bool>, Condvar)>) {
        let &(ref lock, ref cvar) = &*p;
        let mut started = lock.lock().unwrap();

        cvar.wait_timeout(started, time::Duration::from_secs(wait_id)).unwrap();

        println!("t_child {}", wait_id);
    }

    fn t_parent(p: Arc<(Mutex<bool>, Condvar)>) {
        let &(ref lock, ref cvar) = &*p;

        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_all();

        println!("parent");
    } 

    let pair0= Arc::new((Mutex::new(false), Condvar::new()));
    let pair1 = pair0.clone();
    let pair5 = pair0.clone();

    let c5 = thread::spawn(move || {t_child(5, pair5)});
    let c1 = thread::spawn(move || {t_child(1, pair1)});
    
    thread::sleep(time::Duration::from_secs(2));
    
    let p = thread::spawn(move || {t_parent(pair0)});

    c5.join().unwrap();
    c1.join().unwrap();
    p.join().unwrap();
    // 작업의 실행 순서 : t_child 5000 -> t_child 2000 -> sleep(2 s) -> parent
    // 작업의 출력 순서 : t_child 1000 -> parent -> t_child 5000  (전체 실행 시간 2.03s)
    // 1. 두 t_child 는 우선 parent 를 기다린다.
    // 2. wait_timeout 이 1초로 설정된 t_child 1000 는 parent 가 오기전에 시간이 다 경과했으므로 작업을 진행한다.
    // 3. 2초가 지나면 parent 가 실행된다.
    // 4. parent 가 condvar 을 true 로 변경하였으므로, t_child 5000 은 parent 가 끝나고 바로 실행된다.  
}

/// 지금까지 읽기, 쓰기 작업은 모두 lock 권한을 얻어야 작업이 가능하다. 
/// 그런데 일반적으로 lock 필요한 이유는 쓰기 때문이다. (읽기에 lock 이 필요한 경우는... 모르겠음;;)
/// 그렇다면 쓰기 작업에만 lock 권한을 받도록 하는 것이 좀더 효율적이지 않을까?
/// 그래서...
/// 3.8.3 RwLock
#[test]
fn p126() {
    use std::sync::RwLock;

    let lock = RwLock::new(10);
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        // value 를 immutable reference 로 사용하기 위해 접근하는 경우, RwLockReadGuard 로 참조 가능하며
        // 이때는 shared reference 사용하듯이 동일 scope 내에서 여러번 사용 가능하며, 
        // scope 을 벗어나면 자동으로 readlock 에 해제된다.
        println!("r1 = {}", r1);
        println!("r2 = {}", r2);
    }
    {
        let mut w1 = lock.write().unwrap();
        // let mut w2 = lock.write().unwrap(); // readlock 과 달리, 해당 code 추가시, copile 은 되지만, 실행시 starvation 현상이 나타난다.
        *w1 = 7;
        println!("w1 = {}", w1);
    }
}

/// 3.8.4 Barrier synchronization
/// thread 간 공유 counter가 있고, theads 의 작업이 일정 지점에 도착한 thread를 counting 한다.
/// counter 가 설정한 값에 도달하면 barrier 를 풀고 현재까지 모인 thread 들이 다음 단계 작업이 가능하도록 해준다.
/// 
/// ex. 만약 영상처리에서 하나의 frame 을 여러 thread 에서 처리하고 이를 모아서 결과값을 출력해야 한다면,
///     각 thread 끼리 동일한 시간에 처리되는 것을 보장할 수 없기 때문에,
///     하나의 frame 을 처리한 thread 들 중 가장 늦게 처리된 thread 를 기다렸다가 다음 단계 작업을 진행해야 한다.
#[test]
fn p127() {
    use std::sync::{Arc, Barrier};
    use std::{thread, time};

    let one_ms = time::Duration::from_millis(1);

    let mut v = Vec::new();
    // 다수의 threads 을 나중에 join 을 수행하기 번거로우므로 필요한 작업이 왼료한 thread 는
    // 해당 Vec 에 push 해 놓고, 나중에 for 문으로 join() 처리 하기 위해 생성

    // 생성한 thread 들을 barrier sync 를 3개씩 끊어서 처리하고자 함. 
    let barrier = Arc::new(Barrier::new(3));

    // 12개 threads 실행
    // 만약 10개로 설정하면 3개씩 barrier 가 걸리므로 마지막 1개는 barrier 가 걸린 상태에서 pending 됨.
    for i in 0..12 {
        let b = barrier.clone();
        let th = thread::spawn(move || {
            println!("th{} before wait", &i);        
            b.wait();                 // 여기에서 barrier 걸려서 대기함.
            println!("th{} finished barrier", &i);
        });
        v.push(th);

        thread::sleep(one_ms); // 3개씩 barrier 처리를 명확하게 보여주기 위해
    }
    //                    v--------------- 반복 ---------------ㄱ
    //  main thd  : thread spawn -> v.push(th) -> print v  -> sleep  
    //                    L->  print "before wait" -> wait (3개가 쌓일때까지..)
    //                                                   ㄴ-> print "finished.." (3개 각각 thread 잔여작업 실행) -> join                                

    for th in v {
        th.join().unwrap();
    } 
}

/// 3.8.5 semaphore (Rust 에서 표준으로 제공하고 있지 않으므로 자료 구조 작성해야)
/// (아마도 Rust 는 mutability / ownership 를 추가로 고려 해야 하므로, Rc / RefCell 이 대신 사용되는듯?? )
/// 
///  semaphore  자료 구조의 개념.
/// 동시에 작업이 가능한 최대 숫자를 먼저 설정하고, 해당 자원 사용을 요청할 때마다, counter 를 올린다.
/// 만약 counter 가 최대 숫자과 같아진 상태에서, 추가 요청이 온다면 해당 요청은 wait 하고,
/// 다른 작업이 완료 counter 가 max 값보다 작아졌을 때, wait 하고 있는 thread 에 작업을 진행, counter 를 올린다.  
#[test]
fn p128() {
    use std::sync::{Condvar, Mutex};

    // 세마포어용 타입 (기존 Condvar 구조에 max 추가)
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
            while *cnt >= self.max {    // 카운터가 최댓값 이상이면 wait
                cnt = self.cond.wait(cnt).unwrap();
            }
            *cnt += 1;  // 접근하여 lock 을 한 process 의 숫자 count
        }

        pub fn post(&self) {
            // 카운터 감소
            let mut cnt = self.mutex.lock().unwrap();
            *cnt -= 1;
            if *cnt <= self.max {       // 최댓값 이하로 counter 가 떨어졌을 때, 
                self.cond.notify_one(); // 대기중인 threads 중 하나만 wake up 알림
            }
        }
    }

    // 위에서 작성한 semaphore 자료구조 test code
    use std::sync::atomic::{AtomicUsize, Ordering}; // memory ordering : https://int-i.github.io/rust/2022-01-15/memory-ordering/
    use std::sync::Arc;

    const NUM_LOOP: usize = 100_000;
    const NUM_THREADS: usize = 8;
    const SEM_NUM: isize = 4;

    static mut CNT: AtomicUsize = AtomicUsize::new(0); // AtomicUsize : threads 간 공유해도 safe 한 usize ?? tips04 참조

    {
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


///-------------------------------------------------------------
fn main() {
    println!("Hello, world!");
}


