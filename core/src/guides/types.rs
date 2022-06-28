/// 1. Types in Memory
/// 
/// type 의 가장 근본적인 역할은 메모리의 bits 를 해석하는 방식을 제공한다는 것.
/// bit sequence 들은 type 이 정해지지 않으면 아무 의미가 없다. 
/// 
/// === e.g. ===
/// 0b10111101 로 이루어진 bit sequence 는 u8 type 일 때 부호 없는 정수 189 가 되고 i8 type 에선 정수 67로 표현.
/// ============
/// 
/// "alignment" 는 타입의 bytes 들의 저장 위치를 지정하는 개념. pointer 들은 bits 가 아닌 bytes 를 가리키고 
/// 이러한 이유로 만약 type T 가 bit 4 에서 시작한다면 위치를 찾을 수 없다. pointer 는 생성될 때
/// byte 0 또는 byte 1 (bit 8) 을 가리켜야 함. 이러한 이유로 모든 value 는 byte-aligned 되어야 한다.
/// "byte-aligned": 8 bits 의 배수에 value 를 위치시킨다. 
/// 
/// 64 bits CPU 에서 대부분의 value 들은 8 bytes (64 bits) 의 덩어리로 접근됨. 이를 CPU 의 "word size"라고 함.
/// 만약 pointer 가 8 byte aligned 되지 않고 8 byte 의 중간에서 시작한다면 하드웨어는 8 byte 를 두 블럭으로 나눠서 
/// 중앙에서 끝까지 읽고 다시 처음부터 중앙까지 읽는 2 번의 read 를 하고 메모리에 쓸 때에도 2 번의 작업으로 
/// 나눠서 하기 때문에 비효율적이며 multithreading  환경이라면 첫 4 bytes 를 읽는 도중에 나머지 4 bytes 에 다른 
/// thread 가 쓸 수 있기 때문에 value 오염 가능성이 존재.
/// 
/// aligned 되지 않은 데이터에 작업을 하는 것을 "misaligned access" 라고 하며 나쁜 퍼포먼스와 동시성 문제를
/// 일으킬 수 있음. 이런 이유로 많은 CPU 들은 arguments 들이 "naturally aligned" 되는 상태를 요구하거나 선호.
/// 
/// Complex type 들의 경우 포함하고 있는 type 들 중 가장 큰 alignment 를 따름.
/// 
/// === e.g. ===
/// 어떤 complex type 이 u8, u16 그리고 u32 를 가지면 해당 type 은 4-byte aligned.
/// ============ 

/// type 의 "layout" 은 compiler 가 해당 type 의 in-memory 표현을 정하는 방식.
/// Rust 는 "repr" attribute 를 통해 "repr(C)" 와 같이 C 또는 C++ 스타일의 메모리 layout 을 가질 수 있으며 이는
/// FFI (Foreign Function Interface) 를 통해 다른 언어들과 상호작용해야 할 때 유용함. 또한 C layout 은
/// 예측 가능하고 변하지 않기 때문에 repr(C) 는 raw pointer 를 다루는 unsafe 한 상황에서 매우 유용하게 쓰임.
/// 또 다른 유용한 표현으로는 repr(transparent) 가 있으며 type 이 단일 field 로 이루어져 있을 때 compiler 는 
/// 내부와 외부의 type 을 같게 보장.
#[repr(C)]
struct Foo {
    tiny: bool,
    normal: u32,
    small: u8,
    long: u64,
    short: u16,
}
/// 위의 struct Foo 에서 tiny field 의 논리적 크기는 1 bit (true or false) 이지만 CPU 와 메모리는 bytes 단위로
/// 작동하기 때문에 tiny 의 in-memory representation 은 1 byte 로 주어진다. normal 은 4 byte type 이므로
/// 4-byte-aligned 되지만 tiny 에 할당된 1 byte 때문에 normal 의 alignment 가 어긋난다. 이를 수정하기 위해
/// compiler 는 유저 코드에서는 무시되는 3-byte "padding" 을 tiny 와 normal 사이에 삽입.
/// 
/// small field 는 1-byte value 이지만 현재 byte offset 이 1 + 3 + 4 = 8 로 이미 byte-aligned 되었기 때문에
/// small 은 normal 바로 다음에 위치. 현재 1 + 3 + 4 + 1 = 9 이므로 Foo 가 aligned 된다면 long 은 8 byte-aligned
/// 될 수 없기 때문에 7 byte padding 을 삽입해야 하고 이는 마지막 short field 의 2-byte alignment 도 편리하게
/// 보장되며 총 크기는 1 + 3 + 4 + 1 + 7 + 8 + 2 = 26 bytes 가 됨. 마지막으로 Foo 의 alignment 를 정하게 될 때
/// 규칙은 가장 큰 field 의 alignment 를 따르기 때문에 8 bytes alignment 를 따라서 6 bytes padding 을 추가하고 
/// 총 크기를 8 의 배수인 32 bytes 로 맞춘다.

/// C 메모리 표현의 한계는 struct 의 모든 field 들을 원본 struct 에 명시된 순서대로 위치시켜야 한다는 것.
/// repr(Rust) 를 사용한 기본 Rust 표현 방식은 이러한 결정적 방식의 field ordering 을 요구하지 않기 때문에
/// 같은 field 를 가진 두 개의 다른 타입이 같은 메모리 layout 을 갖는 것을 보장하지 않는다.
pub fn eof() {}