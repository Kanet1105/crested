/// 1. Use Borrowed Types for Arguments
/// 함수 인자로는 borrowed type 또는 borrowing the owned type 을 사용
/// 
/// &String 보다는 &str 사용
#[test]
fn example_1() {
    // &String
    let ferris = "Ferris".to_string();
    let curious = "Curious".to_string();
    println!("{}: {}", ferris, three_vowels_string(&ferris));
    println!("{}: {}", curious, three_vowels_string(&curious));

    // println!("{}: {}", ferris, three_vowels_string("ferris"));
    // println!("{}: {}", curious, three_vowels_string("curious"));
    // 위에 주석처리된 println! 은 &String 인자를 받는 형태의 함수로 전달 시 에러가 발생.

    // &str
    let ferris = "Ferris".to_string();
    let curious = "Curious".to_string();
    println!("{}: {}", ferris, three_vowels_str(&ferris));
    println!("{}: {}", curious, three_vowels_str(&curious));

    println!("{}: {}", ferris, three_vowels_str("ferris"));
    println!("{}: {}", curious, three_vowels_str("curious"));
    // &str 인자를 받는 함수에는 둘 다 전달할 수 있다.
    // &str (string slice) 가 &String 이 되려면 명시적으로 할당이 필요한 반면
    // String 은 &str 으로 "type coercion" 을 통해 묵시적으로 바뀔 수 있기 때문.
    // &String 대신 &str 을 인자로 전달하는 것이 비용이 적게 들고 유연하다.
}

fn three_vowels_string(word: &String) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true
                }
            }
            _ => vowel_count = 0
        }
    }
    return false
}

fn three_vowels_str(word: &str) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true
                }
            }
            _ => vowel_count = 0
        }
    }
    return false
}

/// "Type Coercion"
/// 
/// 묵시적으로 type 이 바뀌는 경우를 일컫는다. Type coercion 이 적용되는 경우에도
/// as 를 사용해서 명시적으로 바꿔도 문제되지 않음.

/// let "변수명" 에 type 을 명시했을 경우
#[test]
fn example_2() {
    let a: &i8 = &mut 42;
    println!("{}", a);
    // 'a' 는 &i8 type 으로 coerced.
}

/// 함수 호출 인자
#[test]
fn example_3() {
    example_3_inner(&mut 42);
    // &mut 42 => &i8
}

fn example_3_inner(_: &i8) {}

/// struct, union 또는 enum variant fields 초기화 시
#[test]
fn example_4() {
    Foo { x: &mut 42};
    // &mut 42 => &i8
}

struct Foo<'a> { x: &'a i8 }

#[test]
/// 함수 반환 시
fn example_5() {
    println!("{}", example_5_inner(&12));
}

use std::fmt::Display;
fn example_5_inner(x: &u32) -> &dyn Display {
    x
}

/// [Coercion Types]
/// "Type"                        "Coerced to"    "Case"
/// 1. T                          U               if T is a subtype of U (reflexive case)
/// 2. T_1                        T_3             where T_1 coerces to T_2 and T_2 coerces to T_3 (transitive case)
/// 3. &mut T                     &T
/// 4. *mut T                     *const T  
/// 5. &T                         *const T  
/// 6. &mut T                     *mut T
/// 7. &T or &mut T               &U              if T implements Deref<Target = U>
/// 8. &mut T                     &mut U          if T implements DerefMut<Target = U>
/// 9. Function Item Types        fn pointers      
/// 10. Non-capturing closures    fn pointers
/// 11. !                         any T

/// 8 번 case 의 경우
#[test]
fn example_6() {
    let x = &mut CharContainer {value: 'y'};
    foo(x);
    // &mut CharContainer => &char
}

fn foo(arg: &char) {}

use std::ops::Deref;
struct CharContainer {
    value: char,
}

impl Deref for CharContainer {
    type Target = char;

    fn deref<'a>(&'a self) -> &'a char {
        &self.value
    }
}

/// Unsized Coercions 및 Least upper bound coercions 는 아래 링크 참조
/// https://doc.rust-lang.org/reference/type-coercions.html

/// 2. Concatenating Strings with "format!"
/// 
/// literal 과 non-literal string 을 혼합해야 할 상황에 mutable String 의 push() push_str() 또는 
/// '+' operator 대신 "format!" 을 사용.
fn example_7() -> String {
    let name = "world";
    // let mut result = "Hello, ".to_owned();
    // result.push_str(name);
    // result.push('!');
    // result
    format!("Hello, {}!", name)
    // 위 주석처리된 코드 대신 아래 format! 매크로 사용
}

/// 단점으로는 여러 string 을 이어 붙일 때 효율성 측면에선 push() 를 여러번 호출하는 것이 낫다.

/// 3. Constructors
/// 
/// Rust 는 생성자가 없기 때문에 "associated function" 을 사용해서 "new" 를 구현해서 생성자를 제공.
#[test]
fn example_8() {
    let s = Second::new(42);
    assert_eq!(42, s.value());
    
    let s_default = Second::default();
    assert_eq!(0, s_default.value());
}

pub struct Second {
    value: u64,
}

impl Second {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl Default for Second {
    fn default() -> Self {
        Self { value: 0 }
    }
}

/// 4. The "Default" Trait
/// new() 를 associated function 으로 가진 모든 type 들을 추상화하기 어렵기 때문에 
/// 다른 컨테이너와 generic type 들과 사용 가능한 Default 제공.
/// Arc, Box, Cow 등의 단일 element 컨테이너 들의 기본 implementation.
/// #[derive(Default)] 를 사용해서 struct 내 모든 타입이 implement 할 수 있다.
#[test]
fn example_9() {
    let mut config = MyConfig::default();
    config.check = true;
    println!("config = {:#?}", config);
}

use std::{path::PathBuf, time::Duration};

#[derive(Debug, Default, PartialEq)]
struct MyConfig {
    output: Option<PathBuf>,
    search_path: Vec<PathBuf>,
    timeout: Duration,
    check: bool,
}

/// 5. Collections are Smart Pointers
/// 
/// "Deref" trait 을 사용해서 collection 을 owning 과 borrowed view 를 제공하는 smart pointer 로 취급.
/// struct 가 내부적으로 data structure 를 가지고 있을 때 해당 field 의 borrowed view 를 제공
#[test]
fn example_10() {
    let hvec = HVec::new(&[1, 2, 3, 4, 5]);
    println!("{:?}", *hvec);
}

#[derive(Debug)]
struct HVec<T> {
    data: Vec<Box<T>>,
}

impl<T> Deref for HVec<T> {
    type Target = Vec<Box<T>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Clone> HVec<T> {
    pub fn new(value: &[T]) -> Self {
        Self {
            data: HVec::transform(value),
        }
    }

    fn transform(value: &[T]) -> Vec<Box<T>> {
        let mut result = Vec::<Box<T>>::new();
        for i in value {
            let inner = Box::new(i.clone());
            result.push(inner);
        };
        result
    }
}

/// 6. Finalization in Destructors
/// 
/// Rust 는 finally (함수가 어떤 식으로 종료되어도 실행되는 블럭) 를 제공하지 않기 때문에 type 이
/// 소멸 시 실행되는 destructor 를 Drop trait 을 통해 구현.
/// 
/// Destuctor 가 실행되는 상황
///     1. The end of block
///     2. Early return
///     3. Program panics
/// 
/// 프로그램 panic 시 러스트는 stack 을 되감으며 (unwinding) destructor 를 호출. 따라서 destructor 가 실행되는 
/// 함수에서 panic 시 스택에 있는 destructor 들은 호출이 된다.
#[test]
fn example_11() {
    let _fin = Finalizer { index: 0 };
    Finalizer { index: 1 }; // variable 에 할당하지 않는 경우 destructor 바로 호출.
    let _ = Finalizer { index: 2 }; // _ << 언더바에 suffix 가 없는 경우도 바로 호출.
    example_11_inner();
}

struct Finalizer {
    index: u8,
}

impl Drop for Finalizer {
    fn drop(&mut self) {
        println!("{} running on exit", self.index);
    }
}

fn example_11_inner() {
    let inner = Finalizer { index: 3 };
    panic!(); // panic 시 stack unwind 하면서 destructor 모두 호출
}

/// panic 과 early return 시에도 실행되는 호출 블럭이지만 이미 panic 이 발생한 thread 에서 실행되지 않으며
/// 굉장히 묵시적이고 알아채기 힘든 코드가 될 수 있기 때문에 디버깅 시 매우 까다로워질 수 있다. 만약 stack unwinding 시 
/// panic 이 발생할 경우에는 destructor 를 추가적으로 실행시키지 않으며 이후에 좋은 선택지가 따로 없기 때문에 시스템
/// 리소스들을 원치 않는 방치하게 될 수 있다.

/// 7. std::mem::{take(), replace()}
/// 
/// mutable reference 를 통해서 value 의 ownership 을 가져올 수 없기 때문에 value swap 을 할 때 replace 또는
/// take 를 사용.
#[test]
fn example_12() {
    // 'A' to 'B'
    let mut a = MyEnum::A { name: "MyEnum::A".to_string(), x: 0 };
    a_to_b(&mut a);


}

#[derive(Debug)]
enum MyEnum {
    A { name: String, x: u8 },
    B { name: String, }
}

fn a_to_b(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        // *e = MyEnum::B { name: *name } // 불가능 "cannot move name behind the mutable refernce"
        *e = MyEnum::B { name: std::mem::take(name) };
        // std::mem::take 는 기존 값을 default value 로 대체하고 기존 value 를 반환함.
        // mutable reference 의 value ownership 을 가져올 수 없는 문제를 해결할 수 있는 방법.
        // std::mem::replace() 도 매우 비슷하나 replace 할 value 의 type 까지 인자로 전달해야 함.
        // *e = MyEnum::B { name: std::mem::replace(name, String::new()) };
    }
    println!("{:?}", e);
}

/// 단점으로는 틀리게 되면 borrow checker 가 싫어질 수 있으며 컴파일러가 double store 를 최적화하지 못 하는데서
/// 오는 퍼포먼스 손해를 입을 수 있다. take() 는 destination type 이 Default 를 implement 해야 하며 번거롭다면
/// replace() 를 사용해야 함.

/// 8. On-Stack Dynamic Dispatch
/// 
/// 여러 value 를 dynamic dispatch 할 수 있고 이를 위해 서로 다른 type object 마다 새로운 variable 을 할당해야
/// 했다. dyn 을 사용해서 deferred initialization (지연 초기화) 를 구현할 수 있다.
#[test]
fn example_13() {
    let arg = "-";
    let (mut stdin_read, mut file_read);

    let readable: &mut dyn std::io::Read = if arg == "-" {
        stdin_read = std::io::stdin();
        &mut stdin_read
    } else {
        file_read = std::fs::File::open(&arg).unwrap();
        &mut file_read
    };
}

/// 이 방식은 File 과 Stdin 에 대해 직접 monomorphization 을 구현할 필요가 없고 나주에 사용할 필요가 없는 
/// 것들을 초기화 할 필요가 없으며 heap 에 아무것도 할당하지 않는다는 장점이 있지만 Box 로 구현할 때보다 움직이는
/// 부분이 더 많다는 단점이 있다.
/// 
/// Box-based Version
#[test]
fn example_14() {
    let arg = "-";

    let readable: Box<dyn std::io::Read> = if arg == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(arg).unwrap())
    };
}

/// 9. Iterating Over an Option
/// 
/// "Option" 은 IntoIterator 를 implement 하고 0 개 또는 1 개의 element 를 포함하는 컨테이너라 볼 수 있다.
#[test]
fn example_15() {
    // version 1
    let turing = Some("Turing");
    let mut logicians = vec!["Curry", "Kleene", "Markov"];
    logicians.extend(turing);

    // version 2
    if let Some(turing_inner) = turing {
        logicians.push(turing_inner);
    }

    // chain() 사용
    for logician in logicians.iter().chain(turing.iter()) {
        println!("{} is a logician", logician);
    }
}

/// 10. Pass Variables to Closure
/// 
/// 기본적으로 closure 는 borrowing 으로 환경을 가져온다. move-closure 로 전체 환경을 가져올수도 있지만
/// 몇몇 variable 만 move 하고 싶다면 데이터를 복사하거나 reference 를 전달하거나 다른 변환을 사용해야 한다.
#[test]
fn example_16() {
    use std::rc::Rc;

    let num1 = Rc::new(1);
    let num2 = Rc::new(2);
    let num3 = Rc::new(3);
    let closure = {
        // "num1" moved.
        let num2 = num2.clone(); // num2 is cloned.
        let num3 = num3.as_ref(); // num3 is borrowed.
        move || {
            *num1 + *num2 + *num3;
        }
    };
}

fn example_16_alt() {
    use std::rc::Rc;

    let num1 = Rc::new(1);
    let num2 = Rc::new(2);
    let num3 = Rc::new(3);

    let num2_cloned = num2.clone();
    let num3_borrowed = num3.as_ref();
    let closure = move || {
        *num1 + *num2_cloned + *num3_borrowed;
    };
}

/// 장점으로는 example_16_alt() 보다 example_16() 을 쓰게 되면 복사된 데이터가 closure 정의와 같이 묶이게 되고 목적이
/// 분명해지며 closure 에서 쓰이지 않더라도 drop 될 수 있다. 데이터가 복사되거나 move 되어도 같은 variable 이름을
/// 사용 가능. 추가적인 indentation 이 흠이다.

/// 11. #[non_exhaustive] and Private Fields for Extensibility
/// 
/// Rust 는 라이브러리 저자가 하위 호환성을 해치지 않고 pub struct 에 field 를 추가하거나 pub enum 에 새로운 variant 를 
/// 추가하는 2가지 방법을 제공한다.
///     1. #[non_exhaustive] 를 struct, enum 그리고 enum variant 에 추가.
///     2. private field 를 추가해서 직접 초기화를 하거나 match 를 사용할 수 없게 함.
mod example_17_mod {
    #[non_exhaustive]
    pub struct S {
        pub foo: i32,
    }

    #[non_exhaustive]
    pub enum AdmitMoreVariants {
        VariantA,
        VariantB,
        #[non_exhaustive]
        VariantC { a: String },
    }
}
#[test]
fn example_17() {
    let s = example_17_mod::S { foo: 0 };
    let example_17_mod::S { foo: _, .. } = s;
    let some_enum = example_17_mod::AdmitMoreVariants::VariantA;

    match some_enum {
        example_17_mod::AdmitMoreVariants::VariantA => println!("It's an A"),
        example_17_mod::AdmitMoreVariants::VariantB => println!("It's a B"),
        example_17_mod::AdmitMoreVariants::VariantC { a, .. } => println!("It's a c"),
        _ => println!("It's a new variant."),
    }
}

/// 12. Temporary Mutability
/// 전처리 이후 수정될 일이 없는 variable 은 2 가지 방법으로 임시적인 mutability 를 구현한다.
///     1. Nested block.
///     2. Variable rebinding.
#[test]
fn example_18() {
    // Nested block
    let data_1 = {
        let mut data_1 = vec![2, 1, 5, 3, 4];
        data_1.sort();
        data_1
    };
    println!("{:?}", &data_1);

    // Variable rebinding
    let mut data_2 = vec![2, 1, 5, 3, 4];
    data_2.sort();
    let data_2 = data_2;
    println!("{:?}", &data_2);
}