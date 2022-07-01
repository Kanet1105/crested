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
/// 다른 container 와 generic type 들과 사용 가능한 Default 제공.
/// Arc, Box, Cow 등의 단일 element container 들의 기본 implementation.
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

fn eof() {}