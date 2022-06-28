/// 1. Memory Terminology - "value", "variables" and "pointers"
/// 
/// "value" = "type" 과 type domain 의 "element" 의 조합. 
/// "value" 는 "type" 의 "representation" (표현) 을 사용하여 byte sequence 로 변환될 수 있다. 
/// "value" 의 의미는 값이 저장된 영역과 독립적이다.
/// 
/// === e.g. ===
/// u8 type 의 숫자 6 은 수학적 의미의 정수 6 의 인스턴스이며 메모리 표현은 byte 0x06.
/// "Hello world" str 은 UTF-8 인코딩으로 표현될 수 있는 string 의 도메인에 있는 "value".
/// ============
/// 
/// "place" 는 value 를 저장하는 Rust 용어로써 stack, heap 및 기타 영역을 지칭.
/// "variable" 은 value 를 가장 일반적으로 저장하는 공간을 말한다.
/// "pointer" 는 메모리 영역을 가리키는 주소를 가진 value.
/// "pointer" 는 역참조되어 value 에 접근할 수 있으며 하나의 pointer 를 여러 variable 에
/// 저장할 수 있으며 이를 통해 여러 variable 이 간접적으로 하나의 값을 가진 주소를 참조.


/// example_1 에서 42 (i32), 43 (i32), &x (pointer) 그리고 &y (pointer) 는 value.
/// x, y, var1 그리고 var2 는 variable.
#[test]
pub fn example_1() {
    let x = 42;
    let y = 43;
    let var1 = &x;
    let mut var2 = &x;
    var2 = &y;
}

/// variable, value 그리고 pointer 의 구별이 가장 두드러지는 예제는 아래와 같다.
/// 
/// === e.g. ===
/// let string = "Hello, world";
/// ============
/// 
/// "string" 이라는 variable 이 실제로 갖는 value 는 첫 글자를 가리키는 pointer.
/// variable 을 "value slot" 이라고 생각할 수 있다. 새로운 value 를 지정하면 슬롯이 차고
/// 만약 기존의 value 가 있었다면 dropped & replaced 된다.

/// let x: usize 라면 variable "x" 는 usize 만큼의 크기를 갖는 value 를 저장할 수 있는 
/// 메모리 공간의 이름. slot 이 비어있기 때문에 현재는 value 를 갖지 않지만 x = 6 과 같이
/// value 를 지정하게 된다면 6 이라는 value 를 표현하는 비트를 가진 메모리 공간을 갖게 된다.
/// &x 는 value 와 상관없이 변하지 않지만 여러 variable 을 같은 이름으로 선언한다면 다른
/// 메모리 주소를 가지게 된다. 이러한 메모리 모델은 C 또는 C++ 과 같은 다른 저수준 언어들과 같다.
#[test]
pub fn example_2() {
    let mut x: usize = 0;
    println!("{:p}", &x);
    println!("{:?}", x);

    x = usize::MAX;
    println!("{:p}", &x);
    println!("{:?}", x);

    let x: i32 = 0;
    println!("{:p}", &x);
    println!("{:?}", x);
}

/// 2. Memory Regions - "stack", "heap" and "static"
/// 
/// Rust 코드를 작성할 때 가장 중요한 세 가지 메모리 영역은 "stack" "heap" 그리고 "static".
/// 
/// "stack" 은 함수 호출을 위해 프로그램이 사용하는 메모리 segment. 함수가 호출될 때마다
/// "frame" 으로 불리는 연속적인 메모리 chunk 가 stack 의 맨 위에 할당.
/// "heap" 은 현재 call stack 과 관련 없는 memory pool. 또한 명시적으로 해제 (freeing) 될 때까지 
/// 살아있다. 주로 함수 frame 을 넘어서 살아있어야 하는 value 들을 저장할 때 유용.
/// heap-allocated 된 메모리 pointer 의 lifetime 은 제한이 없음.
/// heap 과 상호작용하는 가장 일차적인 메커니즘은 Box 를 이용하는 것.
#[test]
pub fn example_3() {
    use std::boxed::Box;

    let value: String = String::from("value");
    println!("{:p}", &value);

    let heap_allocated_value = Box::new(value);
    println!("{:p}", heap_allocated_value);
}

/// 메모리를 할당 해제하지 않아서 애플리케이션이 모든 메모리를 사용하게 되는 현상이 memory leak.
/// 전체 프로그램이 접근할 수 있는 read-only configuration 이 필요할 때에는 heap 에 할당하고
/// Box::leak 을 통해 'static reference 로 접근.
#[test]
pub fn example_4() {
    use std::boxed::Box;
    
    let x = vec![1, 2, 3, 4].into_boxed_slice();
    let static_reference = Box::leak(x);
    static_reference[1] = 0;

    assert_eq!(*static_reference, [1, 0, 3, 4]);
}

/// "static memory" 는 프로그램이 컴파일되는 여러 메모리 영역들을 통칭. 프로그램이 실행될 때
/// 자동으로 로드되며 static memory 에 있는 value 들은 프로그램과 lifetime 을 함께한다.
/// 프로그램의 바이너리로 된 text 영역 및 static keyword 또는 constant 로 선언된 variable 들을 포함. 
/// 
/// 'static variable 은 프로그램 종료시까지 할당 해제되지 않지만 'static reference 의 경우
/// static memory 를 가리키지 않을 수 있다. 하지만 static memory 에 존재하므로 레퍼런스 variable 은
/// 프로그램 종료시까지 살아있음.
/// 
/// const 는 compile 시 완전히 계산되며 const 를 참조하는 코드들은 전부 const 의 value 로 대체됨.
/// const 는 place 가 아니므로 메모리 영역이나 다른 저장 영역과 관련이 없음.
/// 특정 value 를 위한 사용하기 편한 이름으로 생각해도 된다.

/// 3. Ownership
/// 
/// 모든 value 는 single owner 를 갖는다 ("Copy" trait 을 갖는 value 들은 예외).
/// 
/// === value move 의 예 ===
/// - 기존 value 를 새 variable 에 부여.
/// - value 를 vector 에 push.
/// - box 를 통해 heap 에 할당.
/// ========================
/// 
/// integer 와 floating-point type 을 포함한 Rust 의 원시 타입들은 Copy.
/// non-Copy type 이나 resource 를 할당 해제해야 하는 타입들의 경우 제외.
/// Box 가 Copy 라면 box1 = box2 일 때 두 box 가 heap-memory 를 이중으로 free 하려는 문제가 생김.
/// 
/// value 를 "drop" 하는 책임은 owner 에게 있음. Rust 에서 dropping 은 owner 가 담당하며
/// variable 이 scope 에 더 이상 존재하지 않을 때 실행.
#[test]
pub fn example_5() {
    let x1 = 42;
    let y1 = Box::new(84);

    {
        let z = (x1, y1);
    }

    let x2 = x1;
    // let y2 = y1; y1 은 primitive type 이 아니므로 Copy 가 아님 -> compile error.
}

/// Drop 순서
/// 
/// variable (함수 파라미터를 포함) 은 역순으로 drop
/// variable 이 순서대로 drop 된다면 나중에 선언된 variable 들이 잘못된 reference 를 가질 수 있음.
/// 
/// nested value 들은 source-code 순서대로 drop
/// nested value 들은 순서대로 drop 돼야 직관적이므로 순서대로 drop.

/// 4. Borrowing and Lifetimes
/// 
/// "reference" 는 추가적인 기능을 갖는 pointer.
/// "shared referece": &T 는 공유될 수 있는 pointer 를 의미.
/// shared reference 는 Copy 이므로 복사되며 reference 가 가리키는 value 들은 immutable.
#[test]
pub fn example_6() {
    let x = 5;
    let mut y = 6;
    example_6_inner(&x, &mut y);
}

pub fn example_6_inner(input: &i32, sum: &mut i32) {
    *sum = *input + *input;
    assert_eq!(*sum, 2 * *input);
}

/// "mutable reference": &mut T 는 컴파일러에 의해 exclusive 로 간주됨 (exclusive mutability).
/// 현재 thread 이외에 다른 thread 가 target value 에 접근할 수 없다고 추정.
/// exclusive mutability 가 없었다면 example_7 의 주석처리된 부분이 compile 될 수 있음.
#[test]
pub fn example_7() {
    let x = 1;
    let mut y = 0;
    example_7_inner(&x, &mut y);
    // example_7_inner(&y, &mut y); compile error
}

pub fn example_7_inner(input: &i32, output: &mut i32) {
    if *input == 1 {
        *output = 2;
    }
    if *input != 1 {
        *output = 3;
    }
}

/// example_8 에서 y 는 다른 pointer 를 가리킬 수 있지만 y 가 가리키는 pointer 의 value 를 바꾸지는 못함.
/// z 를 통해서 y 의 pointer value 를 바꿀 순 있지만 z 가 다른 reference 를 갖게는 못함.
#[test]
pub fn example_8() {
    let a = 24;
    let x = 42;

    let mut y = &x;
    y = &a; // 가능
    // *y = 24; 불가능
    println!("{:?}", y);

    let z = &mut y;
    *z = &x; // 가능
    // z = &x; 불가능
    println!("{:?}", y);
}

/// owner 는 value 를 dropping 할 책임이 있다는 것 외에 owner 와 mutable reference 는 큰 차이 없음.
/// owner 로서 할 수 있는 기능들을 mutable reference 를 통해서 할 수 있지만 value 를 mutable reference
/// 로 옮기게 되면 (move) 다른 value 를 원래 자리에 넣어야 한다.
#[test]
pub fn example_9() {
    let mut x = Box::new(42);
    example_9_inner(&mut x);
}

pub fn example_9_inner(s: &mut Box<i32>) {
    // let was = *s; 와 같이 mutable reference 를 통해서 그 값의 ownership 을 변경하게 되면 기존의
    // mutable reference 가 빈 값을 가리키게 돼서 불가능하나 아래와 같이 std::mem::take(s) 를 통해서 s 의 
    // 값을 default 로 replace 해주고 가져올 수 있음.
    let was = std::mem::take(s);
    println!("{:?}, {:?}", s, was); // 0, 42
    // s 의 value 는 이제 0 을 가리키는 mutable reference 이고 was 가 기존 value 였던 42 의
    // mutable reference 를 소유하게 됨.

    *s = was; 
    // s 는 was 의 mutable reference.

    let mut r = Box::new(84);
    std::mem::swap(s, &mut r);
    assert_ne!(*r, 84);
    // mutable reference 를 value 로 갖는 variable 간의 swap
}

/// 5. Interior Mutability
/// 
/// "Interior Mutability" 는 type 의 shared reference 를 통해서 "value" 를 변경할 수 있는 기능으로
/// 두 가지로 분류.
///     1. shared reference 를 통해 mutable reference 를 가지는 것. => Mutex, RefCell.
///     2. shared reference 를 통해 기존 value 를 "replace" 하는 것. => std::sync::atomic, Cell
/// 
/// 1 번의 경우 Mutex 와 RefCel 이 있으며 내부적으로 한 개의 mutable reference 만 줄 수 있도록 
/// safety mechanism 을 구현하며 UnsafeCell type 에 의존적임. 현재로써 UnsafeCell 은 shared reference
/// 를 통해서 value 를 변경할 수 있는 수단을 제공하는 "유일한" 방법.
/// 
/// 2 번은 "method" 를 통해 value 를 읽고 replace 하는 방법을 제공. Cell type 은 "safe" 한
/// interior mutability 를 제공하는 흥미로운 type 으로 thread 간 공유될 (shareable) 수 없고 Cell 안의 
/// value 의 reference 를 제공하지 않는다. 대신 Cell 의 method 를 통해 value 를 바꾸거나 value 의 
/// copy 를 얻을 수 있다. Thread 간 공유되지 않기 때문에 Cell 의 value 는 shared reference 를 통해 
/// 동시에 변경 (concurrent mutation) 되는 것을 막는다.

/// 6. Lifetimes
/// 
/// "lifetime" 이란 어떤 reference 가 유효한 (valid) 코드의 영역을 부르는 이름.
/// Rust 의 "borrow checker" 가 lifetime 의 본질.
#[test]
pub fn example_10() {
    let arbitrary_number = 0.5; // 0.0 ~ 1.0
    let mut x = Box::new(42);
    let r = &x; // 'a (x 의 reference 의 lifetime) 시작.

    if arbitrary_number > 0.5 {
        *x = 84;
        // borrow checker 가 r 이 사용되지 않은 것을 확인하고 x 의 mutable reference 를
        // 통해 x 의 값을 변경하는 것을 허용.
    } else {
        println!("{}", r); // 'a 종료
    }
}

/// example_11 을 통해 lifetime 에 hole 이 존재할 수 있다는 것을 확인.
#[test]
pub fn example_11() {
    let mut x = Box::new(42);
    let mut z = &x; // (1)

    for i in 0..100 {
        println!("{:?}", z); // (2)
        x = Box::new(i); // (3)
        z = &x; // (4)
    }

    println!("{}", z);
}
/// lifetime 'a 는 z 가 &x 를 갖는 순간부터 시작한다. (3) 에서 'a 는 종료하고 (4) 에서 다시 시작.
/// (4) 에서 print 문으로 빠져나오거나 다시 (2) 로 돌아가도 둘 다 valid 한 value flow (x 가 move 되면 
/// z 가 그 줄 이후부터 더 이상 존재하지 않음) 이므로 example_11 은 borrow checker 를 통과하여 compile 된다.

/// "Generic Lifetimes"
/// 사용자 정의 type 에 다른 value 의 reference 를 저장해야 하거나 &self 보다 오래 살아야 하는 reference
/// 를 반환해야 할 때 사용.
/// type 과 lifetime 의 사용에 있어서 두 가지 미묘한 차이가 있음.
///     1. 사용자 정의 type 이 Drop 을 구현한다면 type 을 drop 할 때 generic lifetime 도 종료된다.
///        만약 Drop 을 구현하지 않는다면 type 안에 저장된 reference 들을 무시하거나 더 이상 사용하지
///        않아도 됨.
///     2. 여러 generic lifetime 에 대한 type 을 사용하는 것은 복잡도를 올리는 일이므로 여러 reference
///        를 갖는 사용자 정의 타입에서만 사용하는 것이 권장되며 method 에서 반환하는 reference 의
///        lifetime 은 그 중 하나의 generic lifetime 을 사용한다.
/// 
/// multiple generic lifetime example
#[test]
pub fn example_12() {
    let target = "Hello, world!";
    let by = ",";
    
    let mut tokenizer = StrSplit::new(by, target);
    for token in tokenizer {
        println!("{:?}", &token);
    }
}

pub struct StrSplit<'s, 'p> {
    delimiter: &'p str,
    document: &'s str,
    spliced: Vec<&'s str>,
    index: i32,
}

impl<'s, 'p> StrSplit<'s, 'p> {
    pub fn new(delimiter: &'p str, document: &'s str) -> Self {
        Self {
            delimiter,
            document,
            spliced: document.split(delimiter).collect::<Vec<&'s str>>(),
            index: 0,
        }
    }
}

impl<'s, 'p> Iterator for StrSplit<'s, 'p> {
    type Item = &'s str;

    fn next(&mut self) -> Option<Self::Item> {
        self.spliced.pop()
    }
}

/// Three Types of "variance" - "covariant", "invariant" and "contravariant"
/// 
/// "variance" 는 어떤 type 이 다른 type 들의 subtype 이고 어떤 상황에 subtype 이 supertype 대신
/// 쓰일 수 있는지 정하는 개념. type A 가 최소한 type B 만큼 유용할 때 A 는 B 의 subtype 이라고 함.
/// Rust 에서 &'a str 을 인자로 받는 함수에 &'static str 을 전달할 수 있는 상황이 variance 의 예시.
/// 'static 은 어느 'a 가 살아있는 동안에도 같이 생존하므로 최소한 'a 보다 더 유용할 수 있기 때문에
/// 'a 의 subtype 이라고 할 수 있음. 이런 정의는 공식적이라기보다는 실용적인 정의.
/// 
/// "covariant" type: 해당하는 type 대신 subtype 을 대신 사용해도 되는 type.
/// 
/// === e.g. ===
/// &'a T 를 type 으로 갖는 variable 은 해당 타입 대신 더 유용하고 오래 생존하는 &'static T 를 사용할 수 있기 
/// 때문에 'a T 는 covariant type.
/// ============
/// 
/// "invariant" type: 반드시 주어진 타입을 제공해야 함을 의미.
/// 
/// === e.g. ===
/// &mut T 와 같이 &mut Vec<&'a str> 을 인자로 받는 함수에 &mut Vec<&'static str> 을 전달할 수 없기 때문에
/// &mut Vec<&'a str> 은 invariant type. 
/// ============
/// 
/// "contravariant" type: 함수 타입처럼 함수 인자가 덜 중요할 때 더 유용해지는 type.
/// 
/// === e.g. ===
/// let x: &'static str; // more useful, lives longer
/// let x: &'a      str; // less useful, lives shorter
/// 
/// fn take_func1(&'static str) // stricter, so less useful
/// fn take_func2(&'a      str) // less strict, more useful
/// ============
/// 
/// 위 예시처럼 Fn(T) 는 T 에 대해 contravariant 하다.
/// 
/// variance 가 lifetime 에 미치는 영향
#[test]
pub fn example_13() {
    let mut s = "hello";
    *MutStr { s: &mut s }.s = "world";
    // 위 코드는 MutStr type 의 variable 'x' 를 정의하고 *x.s 에 "world" 를 value 로 쓰는 것과 같음.
    // 수학에서의 치환 (replacement) 과 비슷함. => *MutStr { s: &mut s } == x
    println!("{}", s);
    
    let mut s = "hello";
    *MutStrError { s: &mut s }.s = "world";
    // println!("{}", s); // 이 코드는 컴파일되지 않음.
}

struct MutStr<'a, 'b> {
    s: &'a mut &'b str,
}

struct MutStrError<'a> {
    s: &'a mut &'a str,
}

/// 
pub fn eof() {}