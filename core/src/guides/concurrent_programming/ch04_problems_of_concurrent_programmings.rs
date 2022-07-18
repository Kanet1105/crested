/// ch05 동시성 프로그래밍 특유의 버그와 문제점

/// 4.1 DeadLock
///           철학자 1
///         /  (음식)  \
///     포크A           포크B
///         \  (음식)  /   
///           철학자 2
/// 
/// 철학자 둘다 음식을 먹으려면 2개의 포크가 필요
/// 철학자1 은 포크 A - B 순으로 잡으며, 철학자2 는 포크 B - A 순으로 잡는다.
/// 철학자1 은 포크 A 를 들고 있고, 철학자2 는 포크B 를 들고 있으며 서로 상대방이 포크를 놓기를 기다린다.
/// 둘은 과연 음식을 먹을수 있는가?
#[test]
fn p144() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    // 포크 a
    let c0 = Arc::new(Mutex::new(()));
    let c0_p0 = c0.clone();
    // 포크 b
    let c1 = Arc::new(Mutex::new(()));
    let c1_p0 = c1.clone();

    // 철학자1
    let p0 = thread::spawn(move || {
        for _ in 0..100_000 {
            let _n1 = c0_p0.lock().unwrap();
            let _n2 = c1_p0.lock().unwrap();
            println!("p0 : eating!!");
        }
    });

    let p1 = thread::spawn( move || {
        for _ in 0..100_000 {
            let _n1 = c1.lock().unwrap();
            let _n2 = c0.lock().unwrap();
            println!("p: eating!!");    
        }
    });

    p0.join().unwrap();
    p1.join().unwrap();
}
/// -> (예측할수 없는 특정 시점에) deadlock 이 발생하여, 두 thread 모두 실행되지 않음.

/// Deadlock with RwLock
#[test]
fn p145() {
    use std::sync::{Arc, RwLock};
    use std::thread;

    let val = Arc::new(RwLock::new(true));

    let t = thread::spawn(move || {
        /* 
        let flag: RwLockReadGuard<bool> = val.read().unwrap(); 
        if *flag {
            *val.write().unwrap() = false;
            println!("flag is ture");
        }
        // 이와 같이 RwLockReadGuard<> type 을 그대로 variable 에 넣고, lifetime 이 살아 있는 동안,
        // write 을 실행할 경우, compile 은 되지만, 실행시 deadlock 발생
        */
        let flag = *val.read().unwrap();  // readlock 이 해당 줄에서 끝날수 있도록 deref 로 value 만 가져옴
        if flag {
            // let temp: RwLockReadGuard<bool> = val.read().unwrap(); -> error
            let _ = val.read().unwrap(); // _ 로 선언된 variable 도 해당 줄에서 즉시 파기 되므로 사용 가능.
            *val.write().unwrap() = false;
            println!("flagg is true.");
        }
    });
    t.join().unwrap();
} 

/// 4.2 livelock & starvation
/// 앞의 철학자 문제에서 deadlock 을 피하기 위해 deadlock 상황에 대한 추가 동작을 명시
/// 철학자1 이 포크A 는 얻었으나 포크B 를 가질수 없는 상태일 때, 포크A 를 놓아서 먼저 양보한 후, 다시 시도 (철학자2 도 동일 내용 구성)
/// -> deadlock 은 피할수 있지만.
///    하필 철학자1, 2 가 각자 첫번째 확보한 포크를 포기하고 다시 시도하는 시점이 완벽히 일치하게된다면 ??
///    -> 둘다 첫번째 포크 확보 -> 두번째포크 확보가 어려우므로 첫번째 포크 포기 -> 일정시간후 다시 첫번째 포크 확보 .... 
///       의 행동만 무한 반복하게됨
///       -> livelock 발생 
///
/// 해당 상태를 system operation 측면에서 보면 => livelock
/// 충돌하여 최종 목표에 도달하지 못하는 각 node 측면에서 보면 => starvation

/// 4.3 은행원 알고리즘
/// Deadlock 을 회피하기 위한 알고리즘
#[test]
fn p152() {
    // available : 은행이 보유한 돈, 즉 얼마까지 빌려줄수 있는지. 
    // allocation : 각 고객들이 현재 빌린 돈이 얼마인지.
    // max : 각 고객들이 얼마의 최대값으로 대출을 요구할지. 
    // 시스템 상태가 stable state 이니지 unsatable state 인지 

    struct Resource<const NRES: usize, const NTH: usize> {
        available: [usize; NRES],           // 이용 가능한 리소스 available[j] 는 j번째 리소스
        allocation: [[usize; NRES]; NTH],   // thread i 가 확보중인 리소스 allocation[i][j](? [j][i] 인듯) 는 스레드 i 가 현재 확보하고 있는 리소스 j 의 수  
        max: [[usize; NRES]; NTH],          // thread i 가 필요로 하는 리소스의 최대값 max[i][j](? [j][i] 인듯) 는 스레드 i 가 필요한 리소스 j 의 최대값 
    }

    impl<const NRES: usize, const NTH: usize> Resource<NRES, NTH> {
        fn new(available: [usize;NRES], max: [[usize;NRES]; NTH]) -> Self {
            Resource{
                available,
                allocation: [[0;NRES]; NTH],
                max,    
            }
        }

        // 현재 상태가 데드락을 발생시키지 않는지 확인(safe -> true, deadlock, starvation -> false)
        fn is_safe(&self) -> bool {
            let mut finish = [false; NTH];            
            // 스레드 i 는 리소스 획득과 반환에 성공했는가?
            // finish[i] = true 일 때, 스레드 i 가 리소스를 확보해 처리를 수행하고, 그후, 모든 리소스를 반환할 수 있음을 나타냄
           
            let mut work = self.available.clone();  
            // 이용 가능한 리소스의 시뮬레이션 값. 
            // work[j]  는 시뮬레이션상에서의 은헹원이 보유한 리소스 j 의 수를 나타낸다. 

            loop {
                // 모든 스레드 i 와 리소스 j 에 대해, 
                // finish[i] == false && work[j] >= (self.max[j][i] - self.allocation[j][i]) 
                // 를 만족하는 스레드를 찾는다. 
                let mut found = false;  
                let mut num_true = 0;
                for (i, alc) in self.allocation.iter().enumerate() {
                    if finish[i] {
                        num_true += 1;
                        continue;
                    }

                    let need = self.max[i].iter().zip(alc).map(|(m, a)| m - a);
                    let is_avail = work.iter().zip(need).all(|(w, n)| *w >= n);
                    if is_avail {
                        found = true;
                        finish[i] = true;
                        for (w, a) in work.iter_mut().zip(alc) {
                            *w += *a;
                        }
                        break;
                    }
                }

                if num_true == NTH {
                    return true;
                }
                if !found {
                    break;
                }
            }
            false
        }

        fn ake(&mut self, id: usize, resource: usize) -> bool {
            if id >= NTH || resource >= NRES || self.available[resource] == 0 {
                return false;
            }

            self.allocation[id][resource] += 1;
            self.available[resource] -= 1;

            if self.is_safe() {
                true
            } else {
                self.allocation[id][resource] -= 1;
                self.available[resource] += 1;
                false
            }
        }

        fn release(&mut self, id: usize, resource: usize) {
            if id >= NTH || resource >= NRES || self.allocation[id][resource] == 0 {
                return;
            }
            
            self.allocation[id][resource] -= 1;
            self.available[resource] += 1;
        }
    }
}

/// 4.4 Reentrant lock (with Mutex) 
/// 만약 lock 을 획득한 thread 가 해당 lock 을 해제하기 전에 다시 lock 을 얻고자 한다 (재귀락 recursive lock)면?
/// -> deadlock 발생 
/// -> 재진입 가능 lock (reentrant lock) 구현 필요
///
/// c code 이해가...;;;
/// 
/// 책에서 결론적으로 rust 에서 recursive lock 을 구현하고자 하면, deadlock 발생으로 실행 X
/// 따라서 rust 에서는 다른 thread 에 공유 리소스를 전달할 것만 clone 하고, 동일 thread 내에서
/// clone 하여 이용하지 않도록 해야 (??)

/// 4.5 의사 각성 (spurious wakeup)
/// - 특정 조건이 만족될 때까지 대기 중이어야 하는 process 가 해당 조건이 만족되지 않았음에도 불구하고
///   실행상태로 변경되는것 (넌 아직 깨어나면 안되....;;)
/// - 리눅스 wait 에 futex 라는 시스템콜을 이용하는데 2.6.22 이전버전에서 futex 시그널에 의해 의사 각성이 발생 했었음.

/// 4.6 Signal
/// 시그널 이란? https://velog.io/@meong9090/%EC%9C%A0%EB%8B%89%EC%8A%A4-%EC%8B%9C%EA%B7%B8%EB%84%90%EC%9D%B4%EB%9E%80
/// 프로그램이 동작할 때 Ctrl + c or z같은 키의 조합을 통해서 프로그램을 종료시키는 등
/// 특정 프로그램이 실행되고 있을 때 어떤 신호를 줘서 동작을 멈추거나, 다른 동작을 하게 하는 신호를 시그널이라고 합니다.
/// 
/// mutithreading  환경에서 signal handler 를 사용할 때 deadlock 이 발생
/// -> 이를 방지하기 위해 signal 을 수신하는 전용 thread 를 이용할 수 있음.
/// -> rust 에서는 signal_hook crate 를 이용하여 처리 권장 (UNIX system 에서 사용, windows 사용 불가)

/// 4.7 Memory barrier (Memory fence) 
/// reordering
/// 현대적인 CPU 는 단위 시간당 실행 명령수 (instuctions-per-second, IPC) 를 높이기 위해
/// out-of-order 를 실행하여, 일반적으로는 thread 간 작업의 우선순위를 사용자가 결정할 수 없다. 
/// 이를 reordering 이 발생한다고 말하며,  
/// system 별 reordering 이 발생하는 패턴은 다음과 같다.  (패턴에 대한 정확한 의미는 모르겠음)
///
///  Architecture   W -> W      W -> R      R -> R      R -> W  
///  AArch64          O           O           O           O
///  x86-64                                               O
///  RISC-V(WMO)      O           O           O           O     (WMO: Weak Memory Ordering)
///  RISC-V(TSO)                                          O     (TSO: Total Store Ordering)

/// 하지만 사용자의 의도에 따라서 reordering 을 막고 순차적으로 처리되는 것도 필요하다. 
/// 이를 지원하기 위해 barrier 의 memory barrier (memory fence) 개념을 사용할 수 있다.
/// 단어의 사전적 의미대로 특정시점을 기준으로 reordering 이 가능한 영역을 분리, 막는다.
/// 만약 특정 작업의 전과 후에 memory barrier 를 구성한다면 barrier 사이에는 다른 process 가 접근하지 못하는 영역을 생성할 수 있다.  
/// (이와 관련, 책 169 command 설명과, 170 쪽 상단 dmb st, dmb ld 그림이 맞지 않는듯..)
///
/// rust Ordering 의 각 용어의 의미를 memory barrier 관점에서 살펴보면 다음과 같다. 
/// 
///              barrier              
///          before | after     Relaxed     Acquire     Release     AcqRel     SecCst
/// ldr(읽기)       |                              
///           ------|-> 통과?     가능                    불가                   불가      
///         통과? <-|------       가능        불가                    불가       불가
/// str(쓰기)       |                                        
///           ------|-> 통과?     가능                    불가        불가       불가
///         통과? <-|------       가능        불가                               불가
/// 
///
/// Relaxed : 모든 barrier 제약 없이 reordering 발생
/// Acquire : 이 명령 이후의 메모리 읽기 쓰기 명령이 이 명령 이전에 실행되지 않는 것을 보증 (뒤에서 요청한게 앞에서 실행되는걸 막음?)
///           -> 메모리 읽기명령(read command)에 지정(==사용?)가능 
///             -> 현재 작업중인 process 가 읽어온 값을 기준으로 작업중인데, 그 뒤에 접근한 process 가 내가 결과값으로 수정하기 전에 다시 읽어가면 안되니까?? 
/// Release : 이 명령 이전의 메모리 읽기 쓰기 명령이 이 명령 이후에 실행되지 않는 것을 보증 (앞에서 요청게 뒤에서 실행되는걸 막음?)
///           -> 메모리 쓰기명령(write command)에 지정(==사용?)가능
///             -> 한 process 가 절차상 마지막으로 썼는데, 이미 실행이 끝났어야 할 process 가 그 뒤에 다시 값을 써서 수정하면 안되니까?? 
/// AcqRel  : 읽기의 경우는 Acquire, 쓰기의 경우는 Release
/// SecCst  : 앞뒤의 메모리 읽기 쓰기 명령 순서 유지
///   
/// Compare and Swap 쓰기 성공 => Acquire + Release
///                  쓰기 실패 => Acquire 
/// -> deplicated! -> comapare_exchange or compare_exchange_weak 로 변경
///    아래 예제에서 확인 가능하듯이, multithread - mutable 이 허용된 data 에 대해 

/// rust UnsafeCell & atomic variable type & Ordering 을 이용한 spinlock 구현 
#[test]
fn p172() {
    use std::cell::UnsafeCell;          // 다수의 mutable reference 를 사용할수 있는 unsafe type 
    use std::ops::{Deref, DerefMut};    // Deref, DerefMut Trait 에 impl 하여 * 를 사용하여 참조 제외
                                        // Guard 객체는 참조 제외를 함으로써 보호 대상 데이터를 읽고 쓸 수 있도록 허가해주며, 
                                        // 이를 실현하기 위해 Deref, DerefMut trait 구현
                                        // Mutex, RwLock type 에 접근하기 위해 lock 을 했을때, Guard 객체로 받을 수 있음.      
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    const NUM_THREADS: usize = 4;
    const NUM_LOOP: usize = 100_000;

    // spin lock 용 type
    struct SpinLock<T> {
        lock: AtomicBool,       // lock 용 공유 변수
        data: UnsafeCell<T>,    // 보호 대상 데이터 -> 여러 thread 에서 mutable 접근을 허용할 것이므로 UnsafeCell type
    }

    // lock 해제 및 lock 중에 보호 대상 데이터를 조작하기 위한 type
    struct SpinLockGuard<'a, T> {
        spin_lock: &'a SpinLock<T>,
    }

    impl<T> SpinLock<T> {
        fn new(v: T) -> Self {
            SpinLock { 
                lock: AtomicBool::new(false),
                data: UnsafeCell::new(v), 
            }
        }

        // lock fn
        fn lock(&self) -> SpinLockGuard<T> {
            loop {
                // lock 용 공용 변수가 false가 될 때까지 대기
                while self.lock.load(Ordering::Relaxed) {}

                // lock 용 공유 변수를 atomic 하게 씀
                // 왜 성공시 AcqRel 가 아니고, Acquire 일까?  
                // (앞에서 공부한 바에 의하면) compare and swap (CAS) 이면 순서를 거슬러 올라와 실행하는 read 를 막고(Acquire),
                // 순서가 뒤쳐져, 나중에 실행하는 write 을 막아야 (Release) 야 하는것 아닐까?
                // -> 현재 구현하고 있는 spinlock 은 읽기,쓰기에 대한 권한의 구분이 없으며,
                //    모두 이곳 lock 함수에 의해 정의 된다. 
                //    따라서, 읽기든 쓰기든 동일 lock 권한을 얻어야 하며 그렇지 않으면 무한 대기해야 한다.
                //    이 구조에서 만약 어떤 process 가 작업이 지연되어, 이미 다른 process 가 수정한 data 에 쓰고자 한다면,
                //    아래 함수에 의해 fail 이 발생, loop 문을 빠져나가지 못하고 다시 lock 권한을 얻어 작업을 해야 한다.
                //    따라서 Release 조건을 필요 없어진다.
                if let Ok(_) = self.lock.compare_exchange_weak(
                    false,              // false 면
                    true,                   // true 를 쓴다.
                    Ordering::Acquire,  // 성공 시의 ordering
                    Ordering::Relaxed   // 실패 시의 ordering
                )
                {
                    break;
                }
            }
            SpinLockGuard { spin_lock: self } //
        }
    }

    // SpinLock type 은 thread 사이에 공유가 가능하도록 지정
    unsafe impl<T> Sync for SpinLock<T> {} //
    unsafe impl<T> Send for SpinLock<T> {} //

    // lock 을 획득 후 자동으로 해체되도록 Drop trait 구현
    impl<'a, T> Drop for SpinLockGuard<'a, T> {
        fn drop(&mut self) {
            self.spin_lock.lock.store(false, Ordering::Release);
        }
    }

    // 보호 대상 데이터의 immutable 한 참조 제외
    impl<'a, T> Deref for SpinLockGuard<'a, T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe{ &*self.spin_lock.data.get() }
        }
    }

    impl<'a, T> DerefMut for SpinLockGuard<'a, T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe{ &mut *self.spin_lock.data.get() }
        }
    }

    // main ----------

    let lock = Arc::new(SpinLock::new(0));
    let mut v = Vec::new();

    for _ in 0..NUM_THREADS {
        let lock0 = lock.clone();
        
        let t = std::thread::spawn(move || {
            for _ in 0..NUM_LOOP {
                let mut data = lock0.lock();
                *data += 1;
            }
        });
        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }

    println!("COUNT = {} (expected = {})", *lock.lock(), NUM_LOOP * NUM_THREADS);
}




/// ---------------------------------------
pub fn eof() {}