/// 01. & vs ref

/// &
/// & denotes that your pattern expects a reference to an object.                                    
/// & is a part of said pattern: &Foo matches different objects than Foo does.
/// & destructures a borrow, lets you reach through a borrow
/// -> "let &x = sth" 은 곧 "sth 이 가리키는 곳의 value 를 x로 선언" 
/// -> sth 은 pointer 형 (&value) 이어야 한다. 
///      * let &x = value -> 사용불가
///      * let x = value or let &x = &value 로 사용해야 
///                         -> 이 경우 결국 x = value 임.
  
/// ref
/// ref indicates that you want a reference to an unpacked value. 
/// It is not matched against: Foo(ref foo) matches the same objects as Foo(foo).
/// ref binds to a location by-reference rather than by-value
/// ref says “take a borrow to this place within the thing I’m matching”.
/// -> "let ref x = sth"  은 "sth 을 가리키는 pointer x 를 선언"
///      * let x = ref .. -> 사용 불가. 
///      * let ref x = value (or pointer) -> Ok
///        -> x 자체가 우변type 에 참조형(&이 붙는) type 이 됨. 

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

}


/// ---------------------------
pub fn eof() {}