/// 5장 비동기 프로그래밍
 
/// 이벤트 대응 작동 기술 (Linux) 관련 참고 자료
/// * file descriptor : https://dev-ahn.tistory.com/96
/// * select 와 epoll : https://ozt88.tistory.com/21

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
        // Pin type : Box 와 비슷하게 pointer 이지만 해당 pointer 참조로 move 되지 않음(onwership 이동이 안되는 pointer?) (Unpin 해야 가능해짐)  
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
    // waker_ref: Waker (Arc<impl ArcWake> type) 의 reference 생성 
    // Waker : executor 에게 실행할 준비가 되었음을 알림으로써 (await 상태의) task 가 waking up 하도록 함. 
    // encapsulate a RawWaker -> Waker 
    // struct RawWaker {
    //     data: *const(),       // executor 에 의해 얻어진 pointer, vtable 첫번째 매개변수로 사용
    //     vtable: &'static RawWakerVTable, // Virtual function pointer table for waker
    // } 
    // ArcWake : 특정 task 를 깨우는 방법?? (fn wake_by_ref 와 fn wake 를 포함한 trait )

    use std::future::Future;
    use std::pin::Pin;
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll};
    // Context : 비동기 task ?
    // pub struct Context<'a> {
    //      waker: &'a Waker, // lifetime coercion : invariant -> argument (contravariant 조건) / return (covariant) 에 모두 사용되므로    
    //      _maker: PhantomData<fn(&'a ()) -> &'a ()> // PhantomData : <> 를 소유한 것처럼 "동작" 하는 항목을 표시 size : 0 type (뭔소리..;;) -> tips 10
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

}

/// --------------------------------------------
pub fn eof() {}
