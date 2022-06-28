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
/// "shared reference" 를 통해서 "value" 를 
pub fn eof() {}