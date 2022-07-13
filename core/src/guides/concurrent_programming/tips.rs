
/// 01. raw pointer 확인. (작업중...)
#[test] 
fn tips01() {
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
    let s1 = "hell".to_string();
    let s2 = "rust".to_string();
    let s3 = "world".to_string();

    let ss1 = &s1;
    let ss2 = &s2;
    let ss3 = &s3;

    println!("{:p}\t{:p}\t{:p}", &s1, &s2, &s3); // 0xdb64afef38    0xdb64afef50    0xdb64afef68
    // low-level 과 동일한 방법으로 address 를 확인하고자 하면, 출력은 되지만, 각 pointer 의 차이를 확인해보면,
    // 이는 value 에 길이와 무관하게 24bytes, value 의 location, length, capacity 정보를 포함한 pointer 가 들어있기 때문이다. 


    println!("{:p}\t{:p}\t{:p}", s1.as_ptr(), s2.as_ptr(), s3.as_ptr());
    // println!("{:p}\t{:p}\t{:p}", ref s1.as_bytes(), ref s2.as_bytes(), ref s3.as_bytes());
    // as_ptr() 은 string slice 를  pointer 로 만들어 준다는데... string 에 대해서도 통할까?

    let aa = std::sync::Arc::new(std::sync::Mutex::new(0));
    println!("{:p}", aa);


    // let y = &x;
    // let z = x.clone();

    // let x_addr = x.as_ptr() as usize;
    // let y_addr = y.as_ptr() as usize;
    // let z_addr = z.as_ptr() as usize;

    // let a = (x_addr + 1);

    // let x_addr_bytes: &[u8] = unsafe{std::slice::from_raw_parts(x_addr as *const u8, 10)};
    // let y_addr_bytes: &[u8] = unsafe{std::slice::from_raw_parts(y_addr as *const u8, 10)};
    // let z_addr_bytes: &[u8] = unsafe{std::slice::from_raw_parts(z_addr as *const u8, 10)};

    // let x_raw: &'static str = std::str::from_utf8(x_addr_bytes).expect("valid UTF");
    // let y_raw: &'static str = std::str::from_utf8(y_addr_bytes).expect("valid UTF");
    // let z_raw: &'static str = std::str::from_utf8(z_addr_bytes).expect("valid UTF");
    
    // println!("x: &x={:p}, x_addr={:p}, x_raw={:p}", &x, &x_addr, &x_raw);
    // println!("y: &y={:p}, y_addr={:p}, y_raw={:p}", &y, &y_addr, &y_raw);
    // println!("z: &z={:p}, z_addr={:p}, z_raw={:p}", &z, &z_addr, &z_raw);
}


/// ---------------------------
pub fn eof() {}