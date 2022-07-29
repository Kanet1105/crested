/// chapter 01

/// ---------------------------------------------------------------------------
/// 1. Memory Terminology 
///    Variables in Depth
///     
///  High-Level Model 
/// variable 이란 value 의 이름
/// 실제 value 가 생성된후, 새로운 이름이 붙거나 소멸하기 전까지 해당 value 를 가리키는 존재 
/// 따라서 value 가 없는 곳 (null point) 를 가리킬 수 없다. 
/// value 한번에 하나의 이름만 갖지만 변경은 가능하다. 
/// value 기준에서 variable 의 변화를 "flows" 라고 한다.
///  
/// 만약 동일한 variable 명에 다른 value 를 대입할 경우, 이를 "shadowing" 이라고 하며, 
/// 이전 value 는 더이상 가리킬 수 없다. 
///  
///  Low-Level Model
/// variable 은 "value slot"
/// 만약 해당 "slot" 에 old value 가 있을 경우, 이를 비우고 value 를 교체. 
/// 만약 let x: usize 로 선언한 경우, stack 내 usize value 를 위한 공간의 이름으로 사용됨.  
/// ---------------------------------------------------------------------------

/// ---------------------------------------------------------------------------
/// 2. Memory Regions
/// 
///  The Stack
///  |         |
///  |---------|
///  | frame(n)| <- 만약 상위 fn 에서 선언한 (ex. frame (1)에 저장된) variable 에 대해 이후 call 된 fn (ex. frame(n))    
///  |---------|    에서 재선언(overwrite) 한다면, unsafe 한 접근이 된다. (?)
///      ...        해당 frame 의 작업이 종료되면 해당 내용은 사라지게 된다. 따라서 모든 reference 의 lifetime 은 frame 의 lifetime 만큼 이어야 한다.
///  |---------|
///  | frame(1)| <- chunk of memory for a function and local variables within the function.
///  |---------|                 
///  | main fn |
///   --------- 
/// 
///  The Heap  
/// pool of memor, call stack 에 의존적이지 않음.
/// Heap memory 내 values 는 할당이 해제될때까지 살아있음. 
/// 따라서 작업중이 function frame의 lifetime 보다 더 길게 살아있을 수 있음.
/// 해당 특성을 이용해 heap memory value pointer 를 threads 간 넘겨주면, 
/// thread 간 안전하게 동일 value 에 대한 작업을 진행할 수 있음.
/// 
/// Rust 에서 heap 을 통한 threads 간 상호 작용의 주요 메카니즘은 Box type (heap memory pointer) 을 사용한다.
/// 다만 heap memory 사용시 사용 해제를 적절하게 해주지 않으면 "Memory leaking" 의 문제가 발생한다.
///   
///  Satic Memory
/// 프로그램 실행시 자동으로 할당되며, 전체 실행 프로그램이 static memory 에 저장되게 된다. 
/// binary code, read only 
/// static keyword (ex. strings)도 프로그램 시작시, static memory 에 공간을 할당, 해당 value 를 저장
/// 따라서 'static reference 는 프로그램 실행 전체에서 lifetime을 가짐. (일정 시점에 가리키지 않도록 할 수 있음)
/// ---------------------------------------------------------------------------

/// --------------------------------------------------------------------------- 
/// 5. Interior Mutability
///    immutable struct 내 field 일부만 변경 가능하게 하는 기능을 의미. 
///    
///    Cell     : copy trait 에 implement 되어있는 기본형 (low-level model) 만 기본적으로 cell 사용 가능
///    RefCell  : RefCell 내부적으로 Cell<BorrowFlag> type field 가 있어, RC 와 같이, borrowing 에 대해 counting 되고 있으며
///               value 는 Cell 과 같이 UnsafeCell<T> type 으로 해당 value memory 위치를 저장하게 된다.
///     
#[test]
pub fn ex_cell() {
    use std::cell::{Cell, RefCell};

    #[derive(Debug)]
    struct SomeStruct {
        _regular_field: u8,
        cell_field_u8: Cell<u8>,                // low-level model u8 type Cell 을 사용하여 interior mutability 적용
        cell_field_str: Cell<&'static str>,     // satic memory 를 사용하는 &str 도 Cell 사용 (다만 reference 이므로 lifetime 명시해야)
        refcell_field_string: RefCell<String>,  // High-level model, 일반 reference 의 경우, RefCell 을 사용
    }

    let my_struct = SomeStruct {
        _regular_field: 1,
        cell_field_u8: Cell::new(2),
        cell_field_str: Cell::new("Hello"),
        refcell_field_string: RefCell::new("World".to_string()),
    };

    println!("{:?}", my_struct);

    let _new_regular = 11;
    let new_u8 = 22;
    let new_str = "HELL!";
    let new_string = "RUST!".to_string();

    // my_struct.regular_field = 11;                    // interior mutability 를 적용하지 않은 type 의 field 는 변경 불가
    my_struct.cell_field_u8.set(new_u8);        
    my_struct.cell_field_str.set(new_str);      
    my_struct.refcell_field_string.replace(new_string); // copy trait 을 사용하지 않으므로, value 를 교체하는 방식으로 value 변경 
    
    println!("{:?}", my_struct);

}

///  atomic : shared memory 에 여러 threads 가 접근하여 읽기/쓰기 작업이 이루어지는 경우, 
///           그 결과가 예측대로 일정하게 나오는것을 보장할 수 없을 때 사용.
/// 
///  ex) (https://doc.rust-lang.org/nomicon/atomics.html?highlight=atomic#atomics)
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
///     해결 방법 (참고: https://doc.rust-lang.org/nomicon/atomics.html?highlight=atomic#atomics, https://modoocode.com/271)
///     1. Stronly-rrdered Memory : 한 프로세스에서 memory write 을 하면, 모든 프로세스에 즉시 알림.
///                                -> performance 에 손해 발생 (실제로 사용 X)
///                                -> weakly-ordered memory 구조에서 해결할수 있는 방법은?
/// 
///     2. Strict Consistency : 모든 core 가 완벽히 동기화된 global clock 이 있고, 해당 clock 에따라 메모리 접근을 직렬화하여 해결
///                            -> 완벽한 동기화가 불가능하며, 해당 구조에서 write process 시간이 0에 수렴해야 함.
/// 
///     3. Non-Strict Consistency
///     - Sequential Consistency
/// 
///       * Ordering 유지 
///       * Atomic read/write operation
///
///       위 2가지 조건을 충족하는 구조를 구성해보면,
///       shared memory 에 접근할 때, single operation queue 에 먼저 쌓아서 처리 
///       write 작업의 경우, 그 전에 read 가 선행되어야 한다. 이런 read/write 작업을 모두 atomic 처리를 한다. 
///       (x86 compiler 는 읽기, 연산, 쓰기를 한번에 처리할 수 있는 assamblely code 를 생성할 수 있음.)
///       (그런데.. 단순 연산이 아닌 복합 연산이면 atomic 처리가 불가능 할거 같은데.... 이건 잘 모르겠음..)
///       
///       -> Sequential Consistency 는 shared memory 사용에 대한 가장 현실적으로 강력한 해결 방법
/// 
///     - Relaxed Consistency
///       memory 접근을 queue 를 통해야 한다는 의미는, 곧 병목 구간이 형성될 수 있음을 의미한다.
///       
///      'ordering' 에 대한 대안으로, (추가 설명 : concurrent_programming/tips.rs 04 번 참조)
///       * write to read : previous write 가 끝나기 전에 read 가 수행될수 있다.
///       * write to write : previous write 가 끝나기 전에 write 이 수행될 수 있다.
///       * read to read or write : previous read 가 끝나기 전에 read 또는 write 이 수행될 수 있다.
/// 
///       write atomicity 에 대한 대안으로,
///       * read others' write data early
/// 
///       위 대안 조건들에 대한 자세한 방법 : https://lolki.tistory.com/16
///       -> Relaxed Consistency 는 read-modify-write 의 atomic 처리가 아니지만, 
///          단순 counting 같은 조건에서는 안전하게 사용가능하다(? 이유는 모르겠음..)
///---------------------------------------------------------------------------

/// ---------------------------------------------------------------------------
/// 6. Lifetimes
/// Generic Lifetime
/// 프로그래밍을 하다보면, variable 의 reference 상태로 가지고 있어햐 하는 경우가 있다. 
/// 그런데 borrow checker 가 정상적으로 작동하기 위해 reference 의 lifetime 명확히 해주어야 한다.
/// 따라서 reference 는 상황에 따라 추가된 별도의 lifetime 의 정의가 필요할수 있다. 
/// 
/// 또한, 함수의 매개변수로 받은 reference 와 함수의 결과물 return reference 간에 lifetime 이 다를 수 있다. 
/// 위와 같은, 다양한 상황에 대해 variable 의 lifetime 을 규정할 때 generic lifetime 을 사용한다.
///
/// Lifetime Variance
/// 
/// subtype 이란?
/// 다른 OOP languages 와 다르게 lifetime 에 대해 사용하며,
/// lifetime 을 명시적으로 추가 부여했을 때, 
/// 해당 type 이 기본적으로 가지고 있는 lifetime 의 위치가 subtype 이 됨. 
///
/// supertype   : higher in rank/ added / explicit / specific
/// subtype     : lower in rank / base  / implicit / general
/// 
/// Variance (전체적으로 참고 : https://doc.rust-lang.org/reference/subtyping.html) 
/// 하나의 variable 에 여러 lifetime 이 존재하므로, 각 lifetime 간 차이가 variance 이다.
/// 이들간 어떤 lifetime 을 사용해야 하는지, 상황에 따라 다를 수 있다.  
///  
/// Convariant / Contravariant / Invariant
///  1. OOP적 개념 정리 (Convariance vs Contravariance) (참고 : https://see-ro-e.tistory.com/245)
///     covariance(공변성)      : X -> Y일때 C<T>가 C<X> -> C<Y>
///     contravariance(반공변성): X -> Y일때 C<T>가 C<Y> -> C<X> 
/// 
///     covariance 인 구조는 x > y 관계의 input 에 대해서 output이 동일한 C<x> > C<y> 구조로 나오는 것. 
///     ex) input   : 애완동물 > 해수어 > 흰동가리
///         function: ...를 키운다.
///         결과    :
///                               개    상어  흰동가리                                                        
///                 애완동물   :   T      T       T      
///                 해수어     :   F      T       T     
///                 흰동가리   :   F      F       T  
///         -> input 에 정의한 x > y 구조대로 결과값(C<x> > C<y>)이 도출됨
/// 
///     contravariance 의 경우, input : x > y 구조가 반대 구조의 output (C<x> < C<y>)을 발생시킴.
///     ex) input   :  한국거주 철수 >  (한국 10년 프랑스 10년 산) 은수 > (한국 10년 프랑스 10년 거주, 디자인 경력 5년) 금수
///         function:  어떤 회사에 갈 수 있을까?
///         결과    :
///                         현대자동차    르노자동차    카르띠에
///                 철수 :      T            F            F
///                 은수 :      T            T            F     
///                 금수 :      T            T            T
///         -> 가장 구체적 특성을 가진 금수가 가장 넒은 분야에서 직장을 구할 수 있다.(예시가 마음에 들지 않지만..) 
/// 
///     * object 가 상속을 받아 내려가면서 자식 object 는 좀더 구체적 의미를 부여받는다.
///     (아래 2가지는 개인적인 생각)
///     * 만약 질문(function) 이 각 object 의 상태에 대한 것이라면, 이는 covariant 한 경우가 된다.
///     * 만약 각 object 이 가진 의미(기능/능력?) 에 의존한 것이라면, contravariant 이다.
/// 
///  2. rust lifetime variance 관점에서
///     OOP 개념에서 상속에 의한 x > y 에서 자식에게 부가되는 가치를 lifetime 으로 바꿔서 생각해보면,
///     variable 에 추가적으로 (specific 한) generic lifetime 을 부가했을 때, 
///     상대적으로 기본 subtype 은 좀더 일반적인(긴) lifetime 을 가지게 된다. 
///     (찾아본 여러 자료에서 모두 해당 예시로 subtype : &'static str 만 들어서... 
///      모든 subtype 이 무조건 더 긴 lifetime 을 갖는것이 보장되는지 잘 모르겠다.)
///     
///     그리고 책에서 언급된 "useful" 의 의미와 결합했을 때,
///     더 긴(genernal) 한 lifetime 인 subtype 을 적용했을 때 "useful" 하다면 -> covariant
///     짧은(specific) 한 lifetime 인 supertype 을 적용했을 때 "useful" 하다면 -> contravariant
///     
///     "useful" 하다는 것을 OOP 개념과 비교해서 이야기하면,
///     프로그램 내에서 좀더 일반적으로 사용할수 있는가 (OOP covariant 예시에서는 '애완동물' / contravariant 예시에서는 '금수')  
///
///     마지막으로, invariant 는 단순히 사전적 의미인 'never changing' 으로 쉽게 이해할수 있다.
///     어떤 제약조건이 있기 때문에 "useful" 함을 따질 필요없이, 해당 lifetime 을 그대로 유지해야 하는 조건이다.
/// 
///   3. 예시로 Convariant / Contravariant / Invariant 개념 확인
///      (해당 예시 참조 table : https://doc.rust-lang.org/reference/subtyping.html)
///     
///         -----------------------------------------------------
///             Type	         Variance in 'a	    Variance in T
///         -----------------------------------------------------
///    (6)  &'a T	              covariant	         covariant
///    (7)  &'a mut T	          covariant	         invariant
///    (5)  *const T		                         covariant
///    (4)  *mut T		                             invariant
///         [T] and [T; n]		                     covariant
///    (1)  fn() -> T		                         covariant
///    (2)  fn(T) -> ()		                         contravariant
///    (3)  fn(T) -> T		                         invariant
///         std::cell::UnsafeCell<T>		         invariant
///         std::marker::PhantomData<T>		         covariant
///    (8)  dyn Trait<T> + 'a	  covariant	         invariant
///         -----------------------------------------------------
/// 
///     (이해가 부족해서 일부만 작성...)
///     (1) fn() -> T 
///         retrun value 의 lifetime 만 고려하면된다. 
///         다른 제약조건이 없으므로, return T lifetime 이 길수록 해당 함수는 "useful" 해진다.
///         -> covariant
/// 
///     (2) fn(T) -> ()
///         return value 없이 argument litetime 만 고려애햐 한다면, 
///         특정(짧은) lifetime 만 보장되도 쓸수 있는 supertype 을 적용하는것이 함수를 적용하기 "useful" 해진다.
///         -> contravariant
///         -> 위 표에서 contravariant 는 이거 밖에 없다. 그만큼 특수한 경우에만 적용 가능 조건으로 생각됨
///            (단 하나의 case 이지만 실제로는 해당 case 자체가 많이 쓰일 수도 있을듯)
///            (함수가 아닌 일반 variable 선언에서 짧은 lifetime 을 갖는 조건이 "useful" 하다는게 성립하지 않을듯..)
/// 
///     (3) fn(T) -> T
///         input parameter 와 return value 가 동일한 generic 이므로 "useful" 을 따질수 없이, 조건 확정됨.
///         -> invariant
/// 
///     (4) *mut T
///         (2) 추가 결론에서 이야기한 바와 같이, 함수에 적용되지 않은 자체 variable 의 lifetime 에서는
///         contravariant 발생경우가 없을것으로 생각되므로, 
///         subtype 적용이 가능하다면 : covariant,
///         제약 조건으로 불가능하다면 : invariant 로 생각해도 될듯 하다. 
///         mutable 이므로 lifetime 을 더 길게 사용할수 없으므로  
///         -> invariant
/// 
///     (5) *const T
///         lifetime 에 대한 추가 고려 조건이 없으므로 subtype 이 "useful"
///         -> covariant
/// 
///     (6) &'a T 
///         generic type T 의 자체 lifetime 과 추가 부여된 lifetime 'a 모두 추가 고려 조건 없음
///         -> 둘다 covariant
/// 
///     (7) &'a mut T
///         (4) 번 조건과 같이 mutable Type 이므로 T 에 대해서는 lifetime 을 확정해줘야 함.
///         -> Variance in 'a : covariant	    Variance in T : invariant
///        
///     (8) dyn Trait<T> + 'a
///         (7) 번과 유사하게 Trait bound 된 generic type T 에 대해서 lifetime 을 건드릴수 없음.
///         -> Variance in 'a : covariant	    Variance in T : invariant
/// 
/// ---------------------------------------------------------------------------

#[allow(dead_code)]          
fn eof() {}
