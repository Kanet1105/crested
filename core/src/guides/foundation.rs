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
/// Rust 코드를 작성할 때 가장 중요한 세 가지 메모리 영역은 "stack" "heap" 그리고 "static".
/// 
/// "stack" 은 함수 호출을 위해 프로그램이 사용하는 메모리 segment. 함수가 호출될 때마다
/// "frame" 으로 불리는 연속적인 메모리 chunk 가 stack 의 맨 위에 할당.
/// 
/// "heap" 은 현재 call stack 과 관련 없는 memory pool. 또한 명시적으로 해제 (freeing) 될 때까지 
/// 살아있다. 주로 함수 frame 을 넘어서 살아있어야 하는 value 들을 저장할 때 유용.
/// heap-allocated 된 메모리 pointer 의 lifetime 은 제한이 없음.
/// heap 과 상호작용하는 가장 일차적인 메커니즘은 Box 를 이용하는 것.
#[test]
pub fn example_3() {
    use std::boxed::Box;

    let value: &str = "value";
    println!("{:p}", value);

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
#[test]
pub fn example_5() {
    
}
pub fn eof() {}