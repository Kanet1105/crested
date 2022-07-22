/// concurrent programming 내용중 추가 학습이 필요해 보이는 것들 
/// 또는 해당 책을 읽는 기간 중 rust 관련 학습한 내용들을 정리.

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

/// 04. ordering (Acquire-Release)
/// => ch04_problems_of_concurrent_programmings.rs 4.7 Memory barrier (Memory fence) 로 변경
///    (기존 설명이 불확실하고 틀린점이 많아 대부분 삭제함)
/// 
///
/// * SeqCst (Sequentially-Consistent) 
/// 현대적인 CPU 는 단위 시간당 실행 명령수 (instuctions-per-second, IPC) 를 높이기 위해
/// out-of-order 를 실행하여, 일반적으로는 thread 간 작업의 우선순위대로 진행시킬수 없다. 
/// 이 때, Ordering::SeqCst 를 적용하면 data access 조건을 정해진 순서대로 순차적 접근하게 만들어
/// 각 thread 작업에 대해 SeqCst 조건을 적용한다면, 원하는 결과를 얻을 수 있을 것이다. 
/// 다만, 해당 과정은 multi-threading 의 장점이 사라지게 되며, IPC 를 낮추는 효과를 가져오게된다.
/// 
/// 만약 SeqCst 를 read-wirte 전체 연산 과정이 아닌, 최초 read 과정에만 각각 적용한다면 좀더 IPC 를 높일수 있을 것이다.
/// 그러나, 이 경우, c case 에 의한 문제 발생을 해결할 수 없다. 
/// 
/// * Acquire-Release      
/// thread1 작업을 통해 y = 3, x = 1 의 결과를 얻었다면, 이를 memory 에 덮어쓰기 전에 
/// 누군가 (thread2) 가 동일 memory 에 작업을 했다면 기존 값이 변경되어 있을 것이다. 
/// 따라서, thread1 은 가져와서 (thread1 resister 에) 저장되어 있는 값이 memory 값과 일치하는지 먼저 비교하고
/// 일치하면 그때 memory 값을 thread1 가 연산한 결과값으로 바꾸어 놓는다. 
/// 이를 Compare and Swap (CAS) 라고 한다.  
/// (CAS 와 Acquire-Relase 에 대한 추가 고찰) 
/// => ch04_problems_of_concurrent_programmings.rs 4.7 Memory barrier (Memory fence) -> fn lock(&self) 참조
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

/// 05. argument type 로 trait 을 이용하기. 
#[test]  
fn tips05() {
    use std::fmt::Display;

    // a. trait bound
    //    concrete (fixing types for compiling)  
    fn print1<T:Display> (input: T) {
        println!("hell, {}", input);
    }

    // a. impl trait
    //    concrete (fixing types for compiling)  
    fn print2(input: impl Display) {
        println!("헬 {}", input);
    }

    // c. dyn
    //    dynamic (fixing types for runtime using vtable)
    fn print3(input: Box<dyn Display>) {  // argument 가 high level model type 이어야 함. (ex. &dyn Display)
        println!("WTF, {}", input);
    }

    print1("rust");
    print2("여름");
    print3(Box::new("날씨"));
}


/// 06. call operators : Fn, FnMut, FnOnce
///     어떤 함수를 정의할 때, argument type 을 해당 trait 중 하나로 bound 하면,
///     함수명(|| {... captured variable... }); 의 형태. 즉, 사용자 정의 closure 로 
///     어떤 기능을 구현 할 수 있다. (ex. map, fold...)
#[test]
fn tips06() {
    // a. Fn (!= function pointer (fn))
    //  capture variable 이 없거나, immutable reference 일때, 
    //  해당 variable 를 사용/정의한 function 을 closure 로 implementing.   
    fn fn_closure<F>(f: F) 
    where
        F: Fn(),        
    {
        f();
    }

    // b. FnMut
    //  mutable reference variable 을 capture 해서 closure 를 implementing.
    //  Fn 의 supertrait
    fn fn_mut_closure<F>(mut f: F)
    where
        F: FnMut(),
    {
        f();
    }

    // c. FnOnce
    //  variable 을 소모 (move)하여 capture, closure implementing.
    //  Fn 의 supertrait
    fn fn_once_closure<F>(f: F) 
    where
        F: FnOnce(),
    {
        f();
    }

    // let print_it = || { drop(my_string)};    // closure 사용 예

    let my_string = String::from("Hell, world");
    let mut my_string2 = my_string.clone();

    fn_closure (|| {println!("{}", my_string)});    // Ok
    fn_closure (|| {println!("{}", my_string)});    // Ok
    // fn_closure 는 Fn trait bound 이므로 capture 한 varaibla 의 ownership 을 가져 오지 않는다.

    // fn_mut_closure (|| {my_string = String::from("Hell, rust")});  // error! mutalbe variable 사용해야
    fn_mut_closure (|| {my_string2 = String::from("Hell, rust")});    // Ok
    println!("{}", &my_string2);            // variable value 변경

    // fn_closure (|| {drop(my_string)});   // erorr!! drop method 와 같이 ownership 에 영향을 주는 function 은 사용할 수 없다.
    fn_once_closure (|| {drop(my_string)}); // Ok 
    // println!("{}", my_string);           // FnOnce 에서 capture 한 variable 은 ownership 이 move 되어 더이상 사용 불가!!
}

/// 06. call operator 의 실제 사용 예제 (trait Iterator, fold method)
/// fold : 반복자 적용 가능 type (ex. array) 으로 선언되어 있으며, 각 element value 가 연산 가능 type 일때,
///        각 element 들을 누적하면서 연산 진행하여 결과를 얻음 (ex. 배열의 전체 합을 구하기)
#[test]
fn ex_fold() {
    let a = [1, 2, 3];

    let sum = a.iter().fold(0, |acc, x| acc + x);

    println!("{}", sum);  // 6
}       

/// fold method 의 source code 를 (FnMut() 활용을 중심으로) 확인해 보면 
/* 
fn fold_source () {
    #[doc(alias = "inject", alias = "foldl")]
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn fold<B, F>(mut self, init: B, mut f: F) -> B // init: B capture 할 초기값, f: FnMut() 
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,     // FnMut 의 argument로 초기설정값(captured value)이자 누적값을 가질) B, 현재 상태의 iter 가 가리키는 value
    {
        let mut accum = init;
        while let Some(x) = self.next() { // 반복자의 다음값이 error 가 아닌때?? (잘모르겠음..;;) 
            accum = f(accum, x);          // closure 내에 사용자가 입력할 함수를 적용하여 accum 에 return 값을 저장 
        }
        accum                             // closure 결과값을 다음 반복자에서 초기값으로 사용하도록 return
    }
}
*/

/// 07. unsafe
/// rust 에서 'safe' 에 대한 기본적 태도
/// - 정상적으로 작동할 수 있는 환경일지라도, 문제가 될 가능성이 있다면, 이를 허가하지 않음. (엄마가 어린아이 다루듯이...;;)
///   ex. orphan rule
/// - 하지만 이런 보수적 환경에 제약이 심하므로 작성자가 안전함을 보장하는 하에 unsafe code 를 사용할 수 있게 해줌.
/// 
/// keyword "unsafe" 를 사용하여 사용가능한 작업
/// a. raw pointer를 역참조하기
/// b. 안전하지 않은 함수 혹은 메소드 호출하기
/// c. mutable static variable 의 접근 혹은 수정하기
/// d. 안전하지 않은 trait 구현하기
/// 
/// - unsafe keyword 를 사용하는것이 rust 의 제약을 풀고 쓰는 것이 아닌, 기존 제약이 작동하는 상태에서 위 기능들을 추가적으로 허용해주는 것.
///   ex. unsafe 내에서 참조자를 사용해도, borrow checker 가 그대로 작동하며 위 작업에 대해서만 추가 허용해줌.
///   따라서 unsafe 사용에서도 안전성을 최대한 확보하지만, 
///   작성자도 사용 의도를 확실히 하고, unsafe block 을 작게 유지해야
/// 
///  a. raw pointer를 역참조 하기
///  raw pointer : *const T, *mut T  (여기서 * 는 deref 연산자 X)
///   
///  raw pointer 의 성질
///  - 빌림 규칙 무시 가능하여 *const T, *mut T 을 동시에 갖거나, 여러 *mut T 를 가질 수 있다.
///  - 유효한 메모리를 가리키고 있음을 보장하지 않음 (쓰레기값을 가리킬 수 있다??)
///  - raw ponier 가 null 될수 있음. (dangling reference 발생 가능??)
///  - 자동 메모리 정리 구현 X (lifetime 종료에 의한 memory freeing 자동으로 작동하지 않음??)
///
/// raw pointer 생성 예제

#[test]
fn tips08_raw_pointer1 () {
    let mut num = 5;    // 참조될 variable 이 mutable 이여야만 *mut raw pointer 사용가능

    // let r = &num;         // 만약 일반 참조 borrow 가 먼저 발생하면 *mut raw pointr 사용 불가
    // let r = &mut num;     // 만약 mutable 참조 borrow 가 먼저 발생하면 *mut raw pointr / *cosnt raw pointer 모두 사용 불가        
    let r1 = &num as *const i32;  
    let r2 = &mut num as *mut i32;         

    println!("{:p}", &num); // 0x78264ff264
    println!("{:?}", r1);   // 0x78264ff264
    println!("{:?}", r2);   // 0x78264ff264
    // -> 모두 동일 주소값을 지니며, 
    
    // println!("{}", *r1); // deref 를 사용하여 해당 주소의 value 를 가져오고자 하면,
    // println!("{}", *r2); // unsafe 하여 compile 불가 하므로,
    unsafe{                 // unsafe block 내에서 실행해야 한다.
        println!("{}", *r1);
        println!("{}", *r2);
    }

    // 임의의 메모리 주소를 가리키는 raw pointer 생성
    let address = 0x012345usize;
    let r = address as *const i32;
    // -> 이런 경우, 임의의 주소값 자체는 문제가 되지 않지만,
    //    만약 앞에서와 같이 deref 하여 value 에 접근하는경우, 그 값의 의미를 보장할수 없으므로 unsafe
    

    // skip     
    // let mut s = String::from("Hell");
    // let sr1 = &s as *const String;
    // let sr2 = &mut s as *mut String;

    // println!("{:p}", &s);   // 0xbad93ff338   
    // println!("{:?}", sr1);  // 0xbad93ff338
    // println!("{:?}", sr2);  // 0xbad93ff338
    // (String 에 대해서도 같은 주소값이 나오긴 하는데.... &s 의 의미는 s 의 value 의 위치를 가리키는 pointer 아닌거 같은데....)

}

/// b. 안전하지 않은 함수 혹은 method 호출하기
/// 함수가 rust 에서 safe 함을 보장할 수 없는 사항이 포함되어 있을때 사용 
fn tips08_unsafe_method1() {
    unsafe fn dangerous(){}  // 함수 앞에 unsafe keyword 사용

    unsafe {        // unsafe 함수는 unsafe block 내에서 호출해야
        dangerous();
    }
}
/// 그런데 이와 같이 안전하지 않은 함수를 unsafe block 로 감싸서 실행했다면 최상위 함수도 unsafe 하다는 의미일까?
/// 반드시 그렇다고 말할 수 없다 -> "안전한 추상화" 
///
/// 안전하지 않은 코드 상에 "안전한 추상화" 생성하기
/// (공식문서 내용과 그대로라서..  ;; ) 
/// 이후 내용 참조 : https://rinthel.github.io/rust-lang-book-ko/ch19-01-unsafe-rust.html?highlight=unsafe#%EC%95%88%EC%A0%84%ED%95%98%EC%A7%80-%EC%95%8A%EC%9D%80-%ED%95%A8%EC%88%98-%ED%98%B9%EC%9D%80-%EB%A9%94%EC%86%8C%EB%93%9C-%ED%98%B8%EC%B6%9C%ED%95%98%EA%B8%B0
/// 

/// 09. volitale
/// 아래 내용은 사이트에서 발췌 : http://1st.gamecodi.com/board/zboard.php?id=GAMECODILAB_QnA_etc&page=1&sn1=&divpage=1&sn=off&ss=on&sc=on&select_arrange=hit&desc=asc&no=3835
/// CPU는 최적화를 위해 이미 캐싱한 변수에 대해서는 메모리까지 다녀오지 않습니다. (최적화!)
/// 그러나 멀티스레드에서는 위의 경우는 문제를 발생시키죠
/// A스레드의 캐시에 변수가 있는데
/// B스레드가 변수의 값을 바꾼 경우
/// A스레드는 올바른 변수의 값을 알기위해서는 반드시 메모리까지 다녀와야만 합니다.
/// 이렇게 '반드시 메모리에서 읽어올것' 이라고 명시해주는 키워드가 바로 volatile 입니다.

/// 10. PhantomData
/// https://docs.rs/rustc-std-workspace-std/latest/std/marker/struct.PhantomData.html
/// https://velog.io/@koo8624/Rust-PhantomDataT
/// https://doc.rust-lang.org/nomicon/phantom-data.html
/// (해석한 내용이 정확지 않을 수도....)
/// struct 를 사용하여 정의된 type 내부 field 중에 만약 type 이 raw pointer 로 되어 있다면, 
/// 해당 field 의 lifetime 은 어떻게 명시적으로 사용될 수 있을까? 
/// (공식 문서 예제에서) 만약 raw pointer 로 메모리 시작 위치와 끝 위치를 명시한 Slice struct 이 있다고 할때,
/*
struct Slice<'a, T:'a> {
    start: *const T, 
    end: *const T,
}
 */
/// generic T 에 대한 lifetime 을 명시해야 Slice type 정의가 가능하지만,
/// (raw pointer 에 대한 lifetime 명시는 error 표시는 되지 않지만 아마 안되는 거 같다...)
/// 이는 rust 문법상 성립되지 않으며, 해당 instance 에 대해 drop 도 문제가 발생한다.
/// (raw pointer 로 type 이 정의되었는 의미는 곧 해당 memory 를 사용한다는 의미이므로 drop 이 필요)
#[test]
fn tips10(){
    use std::marker::PhantomData;

    struct Slice<'a, T:'a> {
        start: *const T,
        end: *const T,
        phantom: PhantomData<&'a T>,
        // 이렇게 marker trait PhantomData 의 type 으로 T 에 대한 lifetime 을 명시해주면,
        // 해당 struct 내 모든 T 에 대해 lifetime 'a 를 갖게 해준다. 
        // 따라서 Slice type 은 'a lifetime 이내의 범위 (covariant) 에서 문제 없이 사용할수 있게 해준다.
    }

    // Vec<T> 의 각 field 의 encapsulation 을 제거해서 보면
    pub struct Vec<T> {
        ptr: *const T,
        cap: usize,
        len: usize,
        _marker: PhantomData<T>,
    }
    // 의 형태로 역시 raw pointer 에 사용돈 T 에 대한 lifetime 을 PhantomData 를 사용하여 보장해준다.
}

/// 11. pin
/// 가리키는 value (대상) 가리키고있는 value (대상) 이 재배치(주소 변경)되는 것을 허용하지 않는 pointer) (Unpin 해야 가능해짐)
/// 
/// 언제 주로 사용할까?
/// self 를 사용하여 struct 내 field 에 접근할때.
///    -> instance 를 생성한 후 만약 해당 instance 의 시작 주소값이 변경된되면, 
///       해당 instance 내 field 에 접근했을 때, 기존의 value 가 아닌 쓰레기 값의 위치에서 읽어오게 됨.
/// 
///  참고 : https://medium.com/tips-for-rust-developers/pin-276bed513fd1


/// 
/// 
/// ---------------------------
pub fn eof() {}