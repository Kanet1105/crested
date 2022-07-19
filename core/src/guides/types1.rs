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
/// 만약 pointer 가 8 byte aligned 되지 않고 8 byte 의 중간에서 시작한다면 데이터는 메모리의 두 블럭에 걸쳐서
/// 쓰여지기 때문에 데이터를 읽기 위해서는 첫 번 째 블럭의 중간부터 끝까지 읽고 두 번째 블럭의 처음부터 중간까지 읽는
/// 2번의 read 가 발생하며 write 시에도 똑같이 2 번에 걸쳐서 써야 하기 때문에 비효율적이며 multithreading 환경에서 
/// 첫 4 bytes 를 읽는 도중에 나머지 4 bytes 에 다른 thread 가 쓸 수 있기 때문에 value 오염 가능성이 존재.
/// 
/// aligned 되지 않은 데이터에 작업을 하는 것을 "misaligned access" 라고 하며 나쁜 퍼포먼스와 동시성 문제를
/// 일으킬 수 있음. 이런 이유로 많은 CPU 들은 arguments 들이 "naturally aligned" 되는 상태를 요구하거나 선호.
/// 
/// Complex type 들의 경우 포함하고 있는 type 들 중 가장 큰 alignment 를 따름.
/// 
/// === e.g. ===
/// 어떤 complex type 이 u8, u16 그리고 u32 를 가지면 해당 type 은 4-byte aligned.
/// ============ 

/// type 의 "layout" 은 컴파일러 가 해당 type 의 in-memory 표현을 정하는 방식.
/// Rust 는 "repr" attribute 를 통해 "repr(C)" 와 같이 C 또는 C++ 스타일의 메모리 layout 을 가질 수 있으며 이는
/// FFI (Foreign Function Interface) 를 통해 다른 언어들과 상호작용해야 할 때 유용함. 또한 C layout 은
/// 예측 가능하고 변하지 않기 때문에 repr(C) 는 raw pointer 를 다루는 unsafe 한 상황에서 매우 유용하게 쓰임.
/// 또 다른 유용한 표현으로는 repr(transparent) 가 있으며 type 이 단일 field 로 이루어져 있을 때 컴파일러 는 
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
/// 컴파일러 는 유저 코드에서는 무시되는 3-byte "padding" 을 tiny 와 normal 사이에 삽입.
/// 
/// small field 는 1-byte value 이지만 현재 byte offset 이 1 + 3 + 4 = 8 로 이미 byte-aligned 되었기 때문에
/// small 은 normal 바로 다음에 위치. 현재 1 + 3 + 4 + 1 = 9 이므로 Foo 가 aligned 된다면 long 은 8 byte-aligned
/// 될 수 없기 때문에 7 byte padding 을 삽입해야 하고 이는 마지막 short field 의 2-byte alignment 도 편리하게
/// 보장되며 총 크기는 1 + 3 + 4 + 1 + 7 + 8 + 2 = 26 bytes 가 됨. 마지막으로 Foo 의 alignment 를 정하게 될 때
/// 규칙은 가장 큰 field 의 alignment 를 따르기 때문에 8 bytes alignment 를 따라서 6 bytes padding 을 추가하고 
/// 총 크기를 8 의 배수인 32 bytes 로 맞춘다.

/// C 메모리 표현의 한계는 struct 의 모든 field 들을 원본 struct 에 명시된 순서대로 위치시켜야 한다는 것.
/// repr(Rust) 를 사용한 기본 Rust 표현 방식은 이러한 결정적 방식의 field ordering 을 요구하지 않기 때문에
/// 같은 field 를 가진 두 개의 다른 타입이 같은 메모리 layout 을 갖는 것을 보장하지 않는다. 대신 field 를 
/// reorder 할 수 있기 때문에 field 간 padding 이 필요 없어지고 Foo 는 이제 필드 크기인 16 bytes 가 됨.
/// Rust 는 기본적으로 type 들의 layout 에 대한 보장이 많지 않기 때문에 컴파일러가 type 들을 rearrange 하기가
/// 조금 더 자유롭다.
/// 
/// 또 다른 layout 은 "repr(packed)" 로 컴파일러에 field 간 padding 을 사용하지 않는다고 명시적으로 알려주는 것.
/// misaligned access 가 발생하며 이로 인한 퍼포먼스 저하를 사용자가 감당하는 방식이며 메모리가 제한적일 때 사용.
/// 
/// 특정 field 나 type 에 더 큰 alignment 를 사용하고 싶을 때 "repr(align(n))" 을 사용. 서로 다른 value 를
/// array 처럼 메모리에 연속적 (contiguous) 하게 저장하고 각각 다른 CPU 의 cache line 을 사용하게 해서 동시성
/// 프로그래밍에서 퍼포먼스 저하를 불러올 수 있는 "false sharing" 을 피하기 위해 사용.
/// "false sharing": 두 CPU 가 cache line 을 공유하는 서로 다른 value 를 접근하는 상황. 같은 entry 를 업데이트
/// 하려고 경쟁하는 상황이 발생.

/// 컴파일러와 "complex type" 을 메모리에 표현하는 방식은 다음과 같다.
/// 
///     Tuple: 같은 type 과 순서를 field 로 갖는 struct
///     Array: element 간 padding 이 없이 연속적인 순서의 contained type 
///     Union: variant 마다 독립적인 layout 을 갖고 최대 모든 variant 가 최대 alignment 를 갖는다.
///     Enumeration: Union 과 같으나 "enum variant discriminant" 를 추가적으로 숨겨진 공유 field 로 갖는다.
///                  discriminant 는 코드가 주어진 enum variant 의 value 를 결정하기 위해 사용하는 value.
///                  discriminant field 의 크기는 variant 의 갯수에 따라 정해짐.

/// "Dynamically Sized Types" 과 "Wide Pointers"
/// 
/// Rust 컴파일러는 대부분의 type 에 대해 컴파일 시 type 크기를 알기 위해 Sized 를 자동으로 implement 함.
/// 두 가지 예외로 "trait object" 와 "slice" 가 있음. 컴파일 시 크기를 알 수 없고 런타임에 type 의 크기가 정해지는
/// type 들을 "dynamically sized types" (DSTs) 라고 함.
/// 
/// 컴파일러는 Struct fields, 함수 인자, 반환 values, variable type 그리고 array type 등 거의 모든 상황에서 type 의
/// 크기가 정해질 것을 요구함. T: ?Sized 와 같이 명시적으로 opt-out 하지 않으면 이 규칙은 모든 type 에 적용되며
/// 함수 인자에 slice 나 trait object 같은 DST 가 있다면 이는 매우 불편.
/// 
/// unsized 와 sized type 을 이어주는 방법으로 "wide pointer" (또는 "fat pointer") 가 있음. wide pointer 는
/// 일반적인 pointer 에서 컴파일러가 pointer 를 생성하기 위해 필요한 정보를 제공하는 추가적인 word-sized field 를 
/// 갖는 pointer. DST 의 reference 에 대해서 컴파일러는 자동으로 wide pointer 를 생성하며 생성된 wide pointer 는
/// 크기를 갖는다. 구체적으로 usize 의 2 배 크기 (타겟 플랫폼의 단어 크기) 이며 pointer 를 저장하는 usize 와 
/// type 을 완성하기 위해 필요한 추가 정보를 담는 usize 로 구성. Box 와 Arc 도 T: ?Sized 를 지원하므로 
/// wide pointer 를 저장할 수 있다.

/// 2. Traits and Trait Bounds
/// 
/// Rust type 시스템의 핵심 중 하나인 "trait" 은 정의될 때 서로의 존재를 모르는 두 type 이 상호 운용될 수 있게 해줌.
/// 어떤 함수가 T 에 대해 generic 하다고 할 때 컴파일러는 모든 type T 에 대해 같은 function 의 사본을 만든다.
/// Vec<i32> 또는 Hashmap<String, bool> 을 만든다는 것은 generic type 과 implementation block 을 복사-붙여넣고
/// 해당 block 의 generic instance 들을 구체적으로 제공된 type 으로 교체하는 것을 의미한다. Vec type 전체를 복사해서 
/// T 를 i32 로 바꾸고 Hashmap type 전체를 복사해서 K 를 String, V 를 bool 로 바꾸는 방식이지만 실제 컴파일러는
/// impl block 을 전체적으로 복사-붙여넣기 하지는 않고 사용하는 코드 부분만 컴파일한다. 만약 Vec<i32> type 에서
/// find() 함수를 사용하지 않는다면 컴파일러는 해당 함수를 compile 하지 않는다.
/// 
/// === e.g. ===
/// impl String {
///     pub fn contains(&self, p: impl Pattern) -> bool {
///         p.is_contained_in(self)
///     }
/// }
/// 
/// 위 코드는 아래 코드와 동일하다. T: impl Trait 처럼 쓰여진 위 방식은 아래 방식의 syntactic sugar.
/// 
/// impl String {
///     pub fn contains<P: Pattern>(&self, p: P) -> bool {
///         p.is_contained_in(self)
///     }
/// }
/// ============
/// 
/// 위와 같은 코드가 있을 때 type 이 is_contained_in() 함수를 실행하기 위한 주소를 알아야 하기 때문에 pattern trait 을 
/// implement 한 모든 type 에 대해서 해당 함수 body 의 사본을 만들어야 한다. 어떤 impl pattern type 에 대해서
/// 컴파일러는 그 주소가 바로 impl pattern type 이 trait method 를 implement 하는 주소라는 것을 알아야 하는데
/// 모든 type 에 대해 사용 가능한 단일 주소는 없기 때문에 복사를 통해 각각의 주소를 갖는 것을 static dispatch 라 하며
/// 이렇게 generic type 에서 non-generic type 으로 가는 방식을 "monomorphization" 이라고 함.
/// 
/// 컴파일러가 코드를 최적화할 때 generic 이 존재하지 않았던 것처럼 할 수 있으며 impl Pattern 의 method 인 
/// is_contained_in() 함수가 trait 없이 직접 호출되는 것처럼 만든다. 컴파일러는 해당하는 type 들을 모두 알 수 있고
/// is_contained_in() 의 implementation 을 inline 할 수도 있다.
/// 
/// monomorphization 의 단점은 type instance 가 각각 따로 컴파일되므로 컴파일 시간이 증가하고 함수의 복사본을 만들기
/// 때문에 프로그램 크기가 증가. 또한 같은 명령에 대해 여러 복사본이 존재하므로 CPU instruction cache 의 효율이
/// 떨어진다.
/// 
/// "dynamic dispatch" 는 static dispatch 와 상반되는 개념으로 구체적인 type 을 알 수 없는 상황에서 사용자 코드가 
/// generic type 의 trait method 를 호출할 수 있도록 함.
/// 
/// === e.g. ===
/// impl String {
///     pub fn contains(&self, p: &dyn Pattern) -> bool {
///         p.is_contained_in(&*self)
///     }
/// }
/// ============
/// 
/// 위와 같이 impl Pattern 을 "&dyn Pattern" 으로 대체한다면 호출자는 pattern 의 주소와 is_contained_in method 의
/// 주소를 제공해야 한다. 호출자가 실제로 제공하는 정보는 "vtable" 이라고 하는 메모리 덩어리의 주소이며 vtable 은
/// is_contained_in() 함수를 포함한 모든 trait method 의 implementation 에 대한 주소를 가지고 있다. vtable 을 통해
/// 여러 type 이 하나의 함수를 사용할 수 있다.
/// 
/// "dyn" 앞에 "&" 가 붙는 이유는 더 이상 컴파일 시 pattern type 에 대한 size 를 알 필요가 없기 때문.
/// 바꿔 말하면 dyn trait 은 !Sized 이며 Sized 로 만들기 위해서 사이즈를 알 수 있는 pointer type 으로 감싼다.
/// 이 pointer 는 vtable 을 가리키는 extra word 를 가진 wide pointer. 사용자는 &mut, Box 그리고 Arc 와 같이 
/// wide pointer 를 가질 수 있는 type 을 dynamic dipatch 를 위해 사용 가능.
/// 
/// "trait object" 란 trait 을 implement 하는 type 과 해당 implementation 의 vtable 의 조합.
/// 
/// === e.g. ===
///     1. clone() method 가 Self 를 반환하는 Clone trait 은 trait object 가 될 수 없다. 컴파일러가
///        dyn Clone trait object 가 실행하는 clone() 함수의 반환 type 을 알 수 없기 때문이다.
///     2. std 의 Extend trait 은 제공된 iterator 의 type 에 대해 generic 하기 때문에 dyn Extend 의
///        trait object vtable 은 여러 주소를 가질 수 있어서 하나의 type 에 대해 하나의 entry 를 가져야 하는
///        vtable 의 특성상 trait object 가 될 수 없다.
///     3. 첫 번째 인자로 self 를 가지지 않는 정적 method 는 trait object 가 될 수 없다.
/// ============
/// 
/// trait object 에 대해 보다 보면 Self: Sized 라는 trait bound 를 불 수 있는데 이는 Self 가 trait object
/// 로 쓰일 수 없다는 것을 의미 (쓰일 수 있다면 !Sized 가 됐을 것).
/// 
/// dynamic dispatch 는 type 과 method 에 해당하는 function body 를 복사-붙여넣을 필요가 없기 때문에
/// 컴파일 시간을 줄일 수 있다는 장점이 있으며 CPU instruction cache 의 효율을 높일 수 있지만 컴파일러가
/// 특정 type 에 대해 최적화하는 것을 막으며 vtable 을 찾는 시간 (lookup time) 이 overhead 가 되는 단점이 존재.
/// static 과 dynamic dispatch 의 선택에 있어 명확한 정답은 존재하지 않음. 유저가 dispatch 를 선택할 수 있는
/// library 를 만들고 싶다면 저자는 static dispatch 로 작성해야 함 (dynamic dispatch 로 작성시 유저에게 선택권
/// 없음). Final code 를 작성한다면 dynamic dispatch 를 사용하는 편이 더 깔끔한 코드를 작성할 수 있으며
/// 약간의 vtable lookup cost 만 지불한다면 컴파일 시간을 줄일 수 있는 방법.

/// "Generic Traits"
/// 
/// Rust trait 은 두 개중 하나의 방법을 통해 generic 할 수 있음.
///     1. trait Foo<T> 와 같은 generic type parameter 를 통해서
///     2. trait Foo { type Bar; } 와 같이 associated type 을 사용해서
/// 
/// 둘의 차이점은 미묘하지만 간단한 선택 기준이 존재.
/// 
/// The Rule of Thumb: 주어진 type 에 대해 단 하나의 implementation 만 필요하면 associated type 을,
///                    그렇지 않다면 generic type 을 사용.
/// 
/// 이는 associated type 은 사용이 훨씬 쉬운 데 비해 다중 implementation 을 허용하지 않기 때문이며 가장
/// 권장되는 방법은 associated type 을 사용할 수 있는 곳에는 모두 associated type 을 사용하는 것이다.

/// "Coherence" and the "Orphan Rule"
/// Rust 는 어느 곳에 trait 을 implement 할 수 있는지 어떤 type 에 implement 할 수 있는지에 대해 꽤 엄격한
/// 규칙을 가지고 있음. 이러한 규칙들은 "coherence" property 를 위해 존재.
/// 
/// "Coherence" property: 주어진 type 또는 method 에 대해 옳은 implementation 선택지는 단 하나만 존재.
/// 사용자 정의 Display trait 을 std 의 bool type 에 implement 한다고 가정한다면 컴파일러는
/// bool value 를 출력하는 함수를 실행해야 하는 상황에서 사용자 정의 implementation 을 사용해야 할 지 표준
/// 라이브러리의 implementation 을 사용해야 할 지 어떤 implementation 이 더 올바른지 알 수 없기 때문에
/// 실행할 수 없다. coherence property 는 이렇게 컴파일러가 선택해야 하는 상황을 예방하기 위해 존재.
/// 
/// coherence 를 지키는 가장 쉬운 방법은 trait 을 정의하는 crate 만 해당 trait 의 implementation 을
/// 사용하도록 정하는 것. 이는 trait 이 정의된 crate 이외의 곳에서 implment 하지 않으면 충돌이 발생할 염려가
/// 없기 때문인데 실질적으로 이런 방식은 지나치게 제한적이고 오히려 trait 을 쓸모없게 만들 수 있다. 
/// 
/// === e.g. ===
/// 만약 trait 을 정의하는 crate 에서만 trait 을 implement 하도록 제한하도록 규칙을 정한다면 
/// std::fmt::Debug 또는 serde::Serialize 를 사용자 정의 type 에 implement 하는 것이 불가능해진다.
/// ============
/// 
/// 따라서 적절한 규칙을 정하여 "upstream" crate 가 "downstream" code 의 흐름을 깨지 않는 선에서 코드를 
/// 작성할 수 있어야 한다.
///     "upstream"   : 사용자 코드가 의존하는 것
///     "downstream" : 사용자 코드에 의존하는 것
/// 
/// Rust 는 이를 위해 "orphan rule" 을 제공한다.
/// 
/// "orphan rule": trait 을 implement 할 때 해당 type 또는 trait 중 단 하나만이 사용자 정의 crate 에 대해
///                지역적이어야 (local) 가능하다.
/// 
/// === e.g. ===
/// 사용자 정의 type 이 std::fmt::Debug 를 implement 하는 것 => 가능
/// bool type 이 사용자 정의 trait 을 implement 하는 것 => 가능
/// bool type 이 std::fmt::Debug 를 implement 하는 것 => 불가능 
/// ============

/// "Blanket Implementations"
/// 
/// orphan rule 은 "impl<T> MyTrait for T where T:" 구문처럼 trait 을 여러 type T 에 대해 implment 할 수
/// 있도록 해주며 이를 "blanket implementation" 이라고 한다. 오직 trait 을 정의하는 crate 에서만
/// blanket implementation 을 할 수 있다.

/// "Fundamental Types"
/// 
/// 특정 type 들 (&, &mut 그리고 Box)은 너무나 필수적이어서 표면상 orphan rule 을 무시하더라도 
/// trait 을 implement 할 수 있도록 해야 할 필요가 있고 이 때 "#[fundamental]" marker 를 사용.
/// 
/// === e.g. ===
/// 만약 orphan rule #[fundamental] marker 가 없다면 impl IntoIterator for &MyType 이 허용되지 않음.
/// 마찬가지로 기본 type 에 blanket implementation 을 하는 것도 허용되지 않음.
/// ============

/// "Covered Implementations"
/// todo!();

/// "Trait Bounds"
/// 
/// Hashmap 의 key type 이 Hash + Eq 를 implement 해야 하고 thread::spawn 의 인자로 전달되는 function 이
/// 반드시 FnOnce + Send + 'static 을 implement 해야 하는 것처럼 generic 코드를 작성하다 보면 trait bound
/// 를 반드시 포함하게 된다. trait bound 는 "T: Trait where T: Type" 의 형태를 띌 필요가 없음. 또한 임의의 
/// type 으로 제한할 수 있고 반드시 generic parameter 를 포함할 필요도 없음.
/// 
/// "Derive Trait"
/// 
/// "#[derive(Trait)]" 은 "impl Trait for Foo<T> where T: Trait" 형태의 syntactic sugar. 대부분의 경우 이런 식의
/// 사용을 의도하지만 그렇지 않은 경우도 있다.
/// 
/// === e.g.===
/// #[derive(Clone)]
/// struct Foo<T> {
///     inner: Arc<T>
/// }
/// ===========
/// 
/// 위 struct 에서 Foo 는 Clone 을 implement 하려고 했지만 Arc<T> 는 이미 Foo 와 상관없이 Clone 을 implement
/// 한 type 이다. 대부분의 경우 이런 식의 derive trait 사용이 문제되진 않지만 꼭 필요하지 않은 곳에 trait bound 를
/// 추가하게 되는 상황이 발생. 

/// todo!() Trait Bounds 의 자세한 내용과 사용 예제는 사용해보면서 채울 것

pub fn eof() {}