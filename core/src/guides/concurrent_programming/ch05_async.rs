/// 5장 비동기 프로그래밍
 
/// 이벤트 대응 작동 기술 (Linux) 관련 참고 자료
/// * file descriptor : https://dev-ahn.tistory.com/96
/// * select 와 epoll : https://ozt88.tistory.com/21
/// 
/// linux epoll 사용하여 event 받기 (linux 에서만 동작, [devendencies] nix = "0.24.2")
/// epoll
/*
use nix::sys::epoll::{
    epoll_create1, epoll_ctl, epoll_wait, EpollCreateFlags, 
    EpollEvent, EpollFlags, EpollOp,
};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpListener;
use std::os::unix::io::{AsRawFd, RawFd};

fn main() {
    // epoll flag 단축계열
    let epoll_in = EpollFlags::EPOLLIN;
    let epoll_add = EpollOp::EpollCtlAdd;
    let epoll_del = EpollOp::EpollCtlDel;

    // TCP 10000 port listen
    let listener = TcpListener::bind("127.0.0.1:10000").unwrap();

    // epoll 용 객체 생성
    // epoll 에서 감시할 socket (file descriptor) 을  epoll 용 객체로 등록한뒤, 
    // 감시대상 이벤트가 발생할 때까지 대기하고 이벤트 발생 후 해당 이벤트에 대응하는 처리를 수행
    let epfd = epoll_create1(EpollCreateFlags::empty()).unwrap(); 

    // listen socket 을 epoll 감시 대상에 추가 
    let listen_fd = listener.as_raw_fd();

    // listen socket 에 event 를 발생 시, 해당 data 를 담을 EpollEvent type instance 를 생성 (?) 
    let mut ev = EpollEvent::new(epoll_in, listen_fd as u64);
    
    // epoll_ctrl 함수는 감시 대상 추가, 삭제, 수정하는 함수
    epoll_ctl(epfd, epoll_add, listen_fd, &mut ev).unwrap();
    

    let mut fd2buf = HashMap::new();
    let mut events = vec![EpollEvent::empty(); 1024];  // ev 를 담을 vector

    // epoll wait 로 이벤트 발생 감시
    // 두번째 매개변수로 전달된 슬라이스에 이벤트가 발생한 file descriptor 가 쓰여지고, 
    // 발행한 이벤트 수를 Option type 으로 반환
    // 세번째는 매개변수는 time out (milli secs) 조건. -1 일 때 time out X 
    while let Ok(nfds) = epoll_wait(epfd, &mut events, -1) {
        
        // 이벤트가 발생한 file descriptor 에 대해 순서대로 처리. 
        for n in 0..nfds {
            // listen socket 과 client socket 으로 분리
            
            if events[n].data() == listen_fd as u64 {
                // listen socket 이벤트 
                // file descriptor 를 취득하고 읽기 쓰기용 객체를 생성한 뒤,
                // epoll_ctl 함수로 epoll 에 읽기 이벤트를 감시 대상으로 등록    
                if let Ok((stream, _)) = listener.accept() {
                    // 읽기, 쓰기 객체 생성
                    let fd = stream.as_raw_fd();

                    let stream0 = stream.try_clone().unwrap();
                    let reader = BufReader::new(stream0);
                    let writer = BufWriter::new(stream);

                    // fd 와  reader, write 의 관계를 만듬.
                    fd2buf.insert(fd, (reader, writer));

                    println!("accept: fd = {}", fd);

                    // fd 를 감시 대상에 등록
                    let mut ev = EpollEvent::new(epoll_in, fd as u64);
                    epoll_ctl(epfd, epoll_add, fd, &mut ev).unwrap();
                }
            } else {
                // client socket
                // client 에서 data 도착 
                let fd = events[n].data() as RawFd;
                let (reader, writer) = fd2buf.get_mut(&fd).unwrap();

                // 1행 읽기
                let mut buf = String::new();
                let n = reader.read_line(&mut buf).unwrap();

                // 커넥션을 close 한 경우 epoll 감시 대상에서 제외한다. 
                if n == 0 {
                    let mut ev = EpollEvent::new(epoll_in, fd as u64);
                    epoll_ctl(epfd, epoll_del, fd, &mut ev).unwrap();
                    fd2buf.remove(&fd);
                    println!("closed: fd = {}", fd);
                    continue;
                }

                println!("read: fd = {}, buf = {}", fd, buf);

                // 읽은 데이터를 그대로 쓴다.
                writer.write(buf.as_bytes()).unwrap();
                writer.flush().unwrap();
            }
        }
    }

}
*/

/// 5.2 coroutine 과 scheduling
/// coroutine : 중단과 재개가 가능한 함수의 총칭 (해당 책에서의 정의, 좀더 다양하게 해석되어 사용되고 있음.)

/// future trait 에 implement 하기
/// 아래의 파이썬 (비대칭) coroutine code 및 해당 내용을 rust 로 구현
/// -- py --
/// def hello():
///     print('Hello,', end=' ')
///     yield                   # 여기서 일단 중단 1  이후 다시 재개
///     print('World! ')
///     yield                   # 여기까지 실행 2
/// 
/// h = hello()                 # 1까지 실행
/// h.__next__()                # 다시1 부터 재개 ~ 2까지 실행
/// h.__next__()                # 다시2 부터 재개 ~ 끝까지
/// --------
#[test]
fn p186() {
    // Hello type 상태 (함수의 실행위치 표시로 사용) enum 만 field 로 가짐.
    struct Hello {
        state: StateHello,
    }

    // 함수의 세가지 사용자 정의 실행 state 
    enum StateHello {
        HELLO,
        WORLD,
        END,
    }

    impl Hello {
        fn new() -> Self {
            Hello {state: StateHello::HELLO,} // 초기 상태
        }
    }

    impl Future for Hello {
        type Output = ();

        // 실행함수
        fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        // Pin : 가리키고있는 value (대상) 이 재배치(주소 변경)되는 것을 허용하지 않는 pointer) (Unpin 해야 가능해짐)  -> tips11
            match (*self).state {
                StateHello::HELLO => {
                    print!("Hello, ");
                    // WORLD 상태로 전이
                    (*self).state = StateHello::WORLD;
                    Poll::Pending   // 다시 호출 가능
                }
                StateHello::WORLD => {
                    print!("World!");
                    // END 상태로 전이
                    (*self).state = StateHello::END;
                    Poll::Pending   // 다시 호출 가능
                }
                StateHello::END => {
                    Poll::Ready(()) // 종료
                }
            }
        }
    }

    use futures::future::{BoxFuture, FutureExt}; 
    // BoxFuture : dyn Future type (작업 위치를 가리키고 고정시키도록 pin type 으로 감싸져 있음)
    // FutureExt : 여러 adapter 를 제공하는 Future trait 을 위한 확장 trait (ex. map, flatten for future)
    
    use futures::task::{waker_ref, ArcWake};
    // waker_ref:  Arc<impl ArcWake> reference 에서 Waker reference 생성
    //      pub fn waker_ref<W>(wake: &Arc<W>) -> WakerRef<'_>  
    //          where  W: ArcWake,      
    //                     ㄴ Trait futures::task::ArcWake   : A way of waking up a specific task.
    //                        pub trait ArcWake: Send + Sync {
    //                          fn wake_by_ref(arc_self: &Arc<Self>); // 여기서 Self ? polled 되어 작업 준비가 된 task (근데 왜 대문자 Self 인지...;;)
    //                          fn wake(self: Arc<Self>) { ... }      // 위와 동일 (차이점은 by_ref 는 data pointer 를 소모하지 않음.) 
    //                        }     
    //
    // Waker : executor 에게 실행할 준비가 되었음을 알림으로써 (await 상태의) task 가 waking up 하도록 함. 
    // encapsulate a RawWaker -> Waker 
    // struct RawWaker {
    //     data: *const(),       // executor 에 의해 얻어진 pointer, vtable 첫번째 매개변수로 사용
    //     vtable: &'static RawWakerVTable, // Virtual function pointer table for waker
    // } 

    use std::future::Future;
    use std::pin::Pin;
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll};
    // Context : (일반적 의미) multi-tasking 을 지원하는 운영체제에서 Task 들은 운영체제가 정한 기준에 따라 작업이 switching 되면서 수행
    //                        이때, 해당 Task 들의 수행 상태를 기억해서, 해당 Task 작업으로 돌아올 위치를 알아야 함. 
    //                        이런 실행 상태 정보를 Context 라고 함.   
    // 
    // pub struct Context<'a> {
    //      waker: &'a Waker, // lifetime coercion : invariant -> argument (contravariant 조건) / return (covariant) 에 모두 사용되므로    
    //      _marker: PhantomData<fn(&'a ()) -> &'a ()> // PhantomData -> tips 10
    // }

    // async/await 에서 process 의 실행단위 Task
    struct Task {
        hello: Mutex<BoxFuture<'static, ()>>,
    }

    // poll 함수를 실행하려면 Context type 값이 필요(?)
    // 우선 아무것도 하지 않은 task type 을 정의 
    impl Task {
        fn new() -> Self {
            let hello = Hello::new();
            Task { hello: Mutex::new(hello.boxed()), }
        }
    }

    // (현재 코드 상) 아무것도 하지 않음.
    // ArcWake : process 를 scheduling 하기 위한 trait
    impl ArcWake for Task {
        fn wake_by_ref(_arc_self: &Arc<Self>) {}
    }

    // fn main ------   
    // 초기화
    let task = Arc::new(Task::new());
    let waker = waker_ref(&task);
    let mut ctx = Context::from_waker(&waker); 


    let mut hello = task.hello.lock().unwrap();

    //정지와 재개 반복 3
    hello.as_mut().poll(&mut ctx);
    hello.as_mut().poll(&mut ctx);
    hello.as_mut().poll(&mut ctx);
}

/// 5.2.2 scheduling
/// 
/// Task    : scheduling 의 대상이 되는 계산의 실행단위 (단위 작업으로써의 Process 개념과 동일?)
/// Waker   : 실행 가능잰 task 를 scheduling
/// Executor: scheduling 된 task 를 순차적으로 받아서 실제 작업 수행
/// 
/// Executor, Task, Waker 작업 diagram (일반적 패턴의 예이며 아래와 다르게 구성 가능)
/// 
///    ----------                                  ------------- 
///   | Executor |                                |    Waker    |    
///   |          |   <-      | 실행큐 |      <-   |<-(Task 정보)|    
///    ----------                           wake   -------------
///        |                                           |
///        |                --------------             |
///         ------------>  | Task         |  <---------
///            poll        |  - Future    |
///                        |     ㄴFuture |
///                        |     ㄴFuture |
///                        |  - Future    |
///                        |     ㄴFuture |
///                        |     ....     |
///                         --------------
/// 해당 패턴의 구현
#[test]
fn p189() {
    use futures::future::{BoxFuture, FutureExt};
    use futures::task::{waker_ref, ArcWake};
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll};

    // -- Task type --
    struct Task{
        // 실행하는 coroutine (Future)
        // 이 Future 의 실행이 완료할 때까지 (실행 가능 상태에서 다음 yield 전까지)
        // 해당 pin pointer 에 lock 을 권한을 얻어서 배타적으로 실행수 있도록 type 설정
        // lifetime 은 ( Future 가 언제 실행 가능한지 보증할 수 없으므로 ?) -> 'static 
        future: Mutex<BoxFuture<'static, ()>>,  

        // Executor 에 scheduling 하기 위한 channel 에 보낼 수 있는 type 이어야 하며,
        // 각 Task 는 channel 에서 처리할 때, atomic 해야
        sender: SyncSender<Arc<Task>>, 
        // 그런데 Task struct  내 field 가 Task 를 사용하는데...
        // 자기 자신을 해당 type 으로 감싸서, 필요한 trait 내 method 에 사용할 때,
        // 해당 field type 으로 바로 사용 가능해짐???
    }

    impl ArcWake for Task {
        // 자신의 Arc 참조를 Executor 로 송신하고, schduling 
        fn wake_by_ref(arc_self: &Arc<Self>) {      
            let self0 = arc_self.clone();  // deep copy 없이 mutliprocessing 가능하도록 
            arc_self.sender.send(self0).unwrap();     // 새로 생성한 mutable 한 pointer 를 channel 에 보냄 
        }
    }

    // -- Executor type --
    struct Executor {
        // 실행 queue 에 송수신 type
        sender: SyncSender<Arc<Task>>, 
        // 앞에서 task 자체 sender type 이 포함되어 있는데 왜 여기서 또???
        // -> executor 에 대한 channel 을 생성할 때 필요한 sender, receiver variable 을 받기 위해?? 
        
        receiver: Receiver<Arc<Task>>,      
    }

    impl Executor{
        fn new() -> Self {
            // channel 생성, queue size : 최대 1024 개
            let (sender, receiver) = sync_channel(1024);
            Executor { 
                sender: sender.clone(), 
                receiver, 
            }
        }

        // 새롭게 Task 를 생성하기 위한 Spawner 작성
        fn get_spawner(&self) -> Spawner {
            Spawner {
                sender: self.sender.clone(),
            }
        }

        fn run(&self) {
            // channel 에서 Task 를 수신하고 순서대로 실행
            while let Ok(task) = self.receiver.recv() {
                // Context 생성
                let mut future = task.future.lock().unwrap();
                let waker = waker_ref(&task);
                let mut ctx = Context::from_waker(&waker);
                // poll 을 호출해서 실행
                let _ = future.as_mut().poll(&mut ctx);
            }
        }
    }

    // -- Spwanwer type --
    struct Spawner {
        sender: SyncSender<Arc<Task>>,
    }
    
    impl Spawner {
        fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
            let future = future.boxed();
            let task = Arc::new(Task {
                future: Mutex::new(future),
                sender: self.sender.clone(),
            });

            // 실행 queue 에 보냄.
            self.sender.send(task).unwrap();
        }
    }

    // -- 앞에서 예시된 비동기로 task 를 실행하는 Hello, World (polling) 함수 구현 --
    struct Hello {
        state: StateHello,
    }

    enum StateHello {
        HELLO,
        WORLD,
        END,
    }

    impl Hello {
        fn new() -> Self {
            Hello {state: StateHello::HELLO,} // 초기 상태
        }
    }

    impl Future for Hello {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
            match (*self).state {
                StateHello::HELLO => {
                    print!("Hello, ");
                    (*self).state = StateHello::WORLD;
                    cx.waker().wake_by_ref();   // 자신을 실행 큐에 넣음
                    Poll::Pending   
                }
                StateHello::WORLD => {
                    print!("World!");
                    (*self).state = StateHello::END;
                    cx.waker().wake_by_ref();   // 자신을 실행 큐에 넣음
                    Poll::Pending   
                }
                StateHello::END => {
                    Poll::Ready(()) // 종료
                }
            }
        }
    }
    
    // -- fn main --
    // let executor = Executor::new();
    // executor.get_spawner().spawn(Hello::new());
    // executor.run();

    // 5.3.1 Future async/await
    // Future : coroutine 으로 구현. 다만 기존 coroutine 의 의미가 '중단, 재개가능 함수' 에서
    //          '미래에 결정되는 값을 표현한 것' 으로 명칭적 의미를 전환해 놓은 것. (Promise 고 부르는 경우도 있음)
    
    // 앞의 예제를 Future trait 의 async/await 을 사용하여 main 함수를 다시 구현하면
    // -- main with async/await --
    let executor = Executor::new();

    // async 로 Future trait 의 type 으로 변환
    executor.get_spawner().spawn(async {
        let h = Hello::new();
        h.await;    // poll 을 호출 해서 실행
        // h.await 은 아래 내용의 추상화 형태
        // match h.poll(cx) {
        //    Poll::Pending => return Poll::Pending,
        //    Poll::Result(x) => x,
        // }
    });
    executor.run();
}

/// 비동기 프로그래밍 의 callback 을 사용한 구현 vs async/await 사용한 구현
/// 
///             callback               vs           async/await
///     x.poll(|a|{         
///         y.poll(|b|{                       x.await + y.await + c.await
///             z.poll(|c|{
///                 a + b + c        
///             })
///         })
///     })

/// 5.4 비동기 라이브러리
///
/// tokio 는 기본적으로 실행환경의 CPU core 수 만큼 thread 를 실행
/// worker thread : 실제 처리를 수행하기 위한 thread 
///                 thread pool 이든 동적 thread 생성이든 모든 실행 모델을 의미 
///
/// multithread 를 thread pool 로 구성 diagram
/// 
///                            thrad1 
///                         ↗   Executor
///                         
///                            thread2
///                         ↗   Executor 
///  task ->  | 실행 큐 |
///                         ↘ thread3  
///                              Executor
///                             
///                         ↘ thread4
///                              Executor
///                   
/// tokio 를 이용한 sleep
/*
fn p212(){
    use std::{thread, time};

    #[tokio::main]
    async fn main() {
        tokio::join!(async move {
            let ten_secs = time::Duration::from_secs(10);
            // thread::sleep(ten_secs);         // std::thread 모듈 안의 sleep 함수를 호출해서 sleep -> 불필요한 work thread 점유
            tokio::time::sleep(ten_secs).await; // sleep 하는 동안 해당 taske 는 Tokio 의 Executor 에 의해 work thread 에서 대피함.
        });
    }
}
*/

/// tokio 를 이용한 async/await 3가지 설정 (multithread, Mutex lock, sleep)
/* 
fn p215() {
    use std::{sync::Arc, time};
    use tokio::sync::Mutex;

    const NUM_TASKS: usize = 8;

    async fn lock_sleep(v:Arc<Mutex<u64>>) {
        let mut n = v.lock().await;     // 1. lock 권한을 얻은 상태에서 await 
        let ten_secs = time::Duration::from_secs(10);
        tokio::time::sleep(ten_secs).await;     // 2. tokio sleep await
        *n += 1;
    }

    #[tokio::main]
    async fn main() -> Result<(), tokio::task::JoinError> {
        let val = Arc::new(Mutex::new(0));
        let mut v = Vec::new();

        for _ in 0..NUM_TASKS {
            let n = val.clone();
            let t = tokio::spawn(lock_sleep(n));
            v.push(t);
        }

        for i in v {
            i.await?;   // 3. mutilthreading 실행에 대한 await
        }

        Ok(())
    }
}
*/

/// tokio channels
/// - mpsc      : 다수 생산자, 단일 소비자
/// - oneshot   : 단일 생산자, 단일 소비자, 한번만 송수신 가능 -> 변수처럼 사용가능
/// - broadcast : 다수 생산자, 다수 소비자
/// - watch     : 단일 생산자, 다수 소비자, 감시(monitoring) 용으로 사용 가능, 수신자는 과거의 값은 얻을수 없다.
/// 
/// oneshot 을 이용하여 미정의 값을 매개변수로 사용하기
/// 
/*
fn p217() {
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
} 
 */

 /// 최종 출력은 10 초를 기다린후 100 을 보내고 이를 받아서 Ok(100) 이 되므로
 /// n = 100 의 결과를 얻을 수 있다.  











/// -- 미완성 --
/// 5.3.2 IO 다중화와 async/await (linux only)
/// epoll 을 이용하여 Future 발생 -> poll -(기다림)-> ...
/// IO event 가 발생 -> IO queue -> wake -> queue(scheduler) -> Executor 로 실행되는 구조를 구현
/// Diagram
///                                       IO Selector
///                                         epoll
///                                           ↑
///      Executor   <-   | 실행큐 |  <-    Task 정보  
///         ↓                       wake      ↑
///     Task/Waker                            |
///     - Future                              |
///       ㄴFuture  ->   | IO 큐 |   ---------- 
///       ㄴFuture
///        ... 


/// --------------------------------------------
pub fn eof() {}
