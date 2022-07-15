/// 01. & vs ref

/// &
/// & denotes that your pattern expects a reference to an object.                                    
/// & is a part of said pattern: &Foo matches different objects than Foo does.
/// & destructures a borrow, lets you reach through a borrow
/// -> "let &x = sth" 은 곧 "sth 이 가리키는 곳의 value 를 x로 선언" 
/// -> sth 은 pointer 형 (&value) 이어야 한다. (value 가 pointer type 이 아닌경우)
///      * let &x = value -> 사용불가
///      * let x = value or let &x = &value 로 사용해야 
///        -> &x 는 pointer 형 또는 &vaule 에만 사용가능하며, 
///           x 는 pointer 아닌 value 를 의미하게 된다.(mutability, ownership 개념은 별개)
  
/// ref
/// ref indicates that you want a reference to an unpacked value. 
/// It is not matched against: Foo(ref foo) matches the same objects as Foo(foo).
/// ref binds to a location by-reference rather than by-value
/// ref says “take a borrow to this place within the thing I’m matching”.
/// -> "let ref x = sth"  은 "sth 을 가리키는 pointer x 를 선언"
///      * let x = ref .. -> 사용 불가. 
///      * let ref x = value  -> Ok
///      * let ref x = &value -> Ok (&&)
///        -> x 자체가 우변type 에 참조형/pointer type 이 됨. 

#[test]
fn tips01() {

    fn print_type_name_of<T>(_: T) {
        println!("{}", std::any::type_name::<T>());
    }

    let x = &false;  
    print_type_name_of(x);  // &bool

    let &x = &false;  
    print_type_name_of(x);  // bool

    // let &x = false; -> error!!!
    let x = &"qwe".to_owned();    // OK!!
    let ref x = "abc".to_owned(); // OK!!
    //let x = ref false; -> error!!

    let ref x = &false; 
    print_type_name_of(x);  // &&bool

    let ref x = 1;
    print_type_name_of(x);  // &i32

    let x = &1;
    print_type_name_of(x);  // &i32

    let x = "abc".to_string();
    print_type_name_of(&x);     // &alloc::string::String
    print_type_name_of(x);      // alloc::string::String
}



/// 02. rust 에서의 matrix 표현
#[test]
fn tips02() {
    let a = [3;4];  // [3, 3, 3, 3]
    println!("{:?}", &a);

    let b = [[3;4];5]; // [[3, 3, 3, 3], [3, 3, 3, 3], [3, 3, 3, 3], [3, 3, 3, 3], [3, 3, 3, 3]]
    println!("{:?}", &b);
}



/// 03. raw pointer 확인 (간단하지만 정리하는겸...)
#[test] 
fn tips03() {
    // 1. variable 이 reference type 의 value 에 대응되는 경우. 
    let str1 = "hi!";
    let str2 = "rust";
    let str3 = "world";

    let addr1 = str1.as_ptr() as usize;
    let addr2 = str2.as_ptr() as usize;
    let addr3 = str3.as_ptr() as usize;

    // 각 string slice 의 저장된 위치는 ...
    println!("{:p}\t {:p}\t {:p}", str1, str2, str3); // 0x7ff73c5d44f0   0x7ff73c5d44f3  0x7ff73c5d44f7
    println!("{}  {}  {}", &addr1, &addr2, &addr3);   // 140699846395120  140699846395123 140699846395127
    // &str 은 명시적으로 reference type 이므로 해당 변수값(pointer) 를 직접 주소값으로 출력하거나, 
    // 이를 직접 raw pointer 형으로 변환 후, 해당 value 를 출력해도 동일한 주소값이 나오는것을 확인할 수 있다.
    // str1 의 값 0x7ff73c5d44f0 을 십진법으로 표기하면 140699846395120
    // 추가로, str1 과 str2 의 차이는 3 /  str2 와 str3 의 차이는 4로 문자 길이대로 각각 3bytes, 4bytes 씩 
    // pointer  값이 증가함을 확인할 수 있다. 

    // 2. low-level model type 에 대한 memory address 확인
    let num1: u8 = 0;
    let num2: u32 = 1;
    let num3: u64 = 2;

    println!("{:p} \t {:p} \t {:p}", &num1, &num2, &num3); // 0x9f785ff188    0x9f785ff1a0    0x9f785ff1b8
    // 간단하게 참조형으로 해당 value 가 저장된 memory 위치를 확인 할 수 있다. 

    // 3. High-level model type 에 대한 vaule 가 저장된 memeory address 확인
    //    -> .. 모르겠다..;;;;

    // 4. smart pointer 
    // (모든 스마트 포인터가 동일하게 적용 가능한지 모르겠지만..)
    // ch03_sync.rs mutex 예제를 보면, memory freeing 을 위해, ownership 을 관리할 필요가 없어지므로,
    // clone 하게 되면, 동일 위치(value?)를 가리키는 pointer 가 생성된다. 
}

/// 04. atomic types 
/// (아직 모)
/// 
/// 05. ordering (Acquire-Release)
/// (참고 : https://doc.rust-lang.org/nomicon/atomics.html?highlight=atomic%20usize#atomics)
/// foundation2.rs 에서 언급한 atomic 에서 같이, 하나의 value 에 여러 thread 가 작업을 했을 때,
/// 그 결과값을 보장할 수 없는 경우가 발생한다. 따라서 해당 연산에 대해 atomic 처리가 보장되어야 한다.
/// 그러나 foundation2.rs 에서도 의문점이 있었듯이, x86 compiler 의 read-modify-write atomic 처리 만으로
/// shared value 의 연산 결과를 보장할 수 없다. 
/// 해당 설명에서 ordering 에 대한 설명이 제대로 되어 있지 않기 때문이다. 
/// 
///     initial state: x = 0, y = 1
///     THREAD 1        THREAD2
///     y = 3;          if x == 1 {
///     x = 1;              y *= 2;
///                     }
/// 
///     위의 경우, y 의 결과 값은 아래 3가지 시나리오에 의해 다르게 결정될수 있다.
///     a. y = 3 : thread2 가 먼저 작업이 완료된 이후, thread1 의 내용이 마지막에 메모리에 씀.
///     b. y = 6 : thread1 가 value 를 update 한 이후, thread2 가 update
///     c. y = 2 : thread2 (read) - thread1(read and write) - thread2(write) 한 경우
///
/// a. b. 는 문제라기보다는 process 간 어떻게 인과관계를 설정하는가에 따라 최종 결과가 바뀔 수 있음을 보여주는 case 이다. 
/// c. case 로 문제를 한정해도, read-modify-write atomic 처리만으로 해결할 수 없다. 
/// 이를 해결하기 위해 (앞에서 설명이 부족했던) ordering 이 필요하다.
/// 
/// * Acquire-Release     
/// 우선 code 작성자가 의도한 정답이 y = 6 (thread1 완료 후 thread2 작업) 이라고 가정하자
/// thread1 작업을 통해 y = 3, x = 1 의 결과를 얻었다면, 이를 memory 에 덮어쓰기 전에 
/// 누군가 (thread2) 가 동일 memory 에 작업을 했다면 기존 값이 변경되어 있을 것이다. 
/// 따라서, thread1 은 가져와서 (thread1 resister 에) 저장되어 있는 값이 memory 값과 일치하는지 먼저 비교하고
/// 일치하면 그때 memory 값을 thread1 가 연산한 결과값으로 바꾸어 놓는다. 
/// 이를 Compare and Swap (CAS) 라고 한다. 
/// 
/// 이를 x86-64 complie 로 작성하면
/// 
///     cmpq %rsi, (%rdi)   ; %rsi == (%rdi)                     // rsi register 의 값과 rdi register 가 가리키는 메모리 상의 값을 비교하여 ZF flag 에 저장
///     jne LBB0_1          ; if %rsi != (%rdi) then goto LBB0_1 // 비교결과 (ZF flag 검사) 가 같지 않으면 LBB0_1 라벨로 점프
///     movq %rdx, (%rdi)   ; (%rdi) = %rdx     // (LBB0_1 로 점프하지 않았다면) 결과값 rdx 을 rdi register 가 가리키는 메모리에 입력
///     movl $1, %eax       ; %eax = 1          // 그리고 1 (true) 
///     retq                ;                   // 을 반환  
///  LBB0_1:
///     xorl %eax, %eax     ; %eax = 0          // (불일치로 해당 라베롤 점프해왔으므로) 0(false) 을
///     retq                ;                   // 을 반환             
///
/// 해당 과정의 process 를 ordering 하는 방법이 Acquire-Release 이다. 
/// (아마도 단의의 의미대로 (메모리를 확인하여 결과 값을 넣을 수 있는 상황을) acquire 하고 결과값 memory 에 release) 
/// 이를 참고 사이트 예제 그대로 rust code 로 구성하면, (https://doc.rust-lang.org/nomicon/atomics.html?highlight=atomic%20usize#atomics)

fn tips04() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread;

    let lock = Arc::new(AtomicBool::new(false)); // value answers "am I locked?"

    // ... distribute lock to threads somehow ...

    // Try to acquire the lock by setting it to true
    while lock.compare_and_swap(false, true, Ordering::Acquire) { } // deprecated / compare_exchange 나 compare_exchage_weak 으로 사용해야
    // broke out of the loop, so we successfully acquired the lock!

    // ... scary data accesses ...

    // ok we're done, release the lock
    lock.store(false, Ordering::Release);
}


/// ---------------------------
pub fn eof() {}