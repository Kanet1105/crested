/// 1. Use Borrowed Types for Arguments
/// 
/// String 와 &str 의 구조적 차이
/// (참고 : https://blog.thoughtram.io/string-vs-str-in-rust/) 
/// 
///  * String 의 저장 구조
/// === e.g.===
/// let mut my_name = "Pascal".to_string(); 
/// 
///                 buffer : heap 영역 실제 value 가 저장된 곳을 가리키는 pointer
///                 /  capacity : value 를 저장하기 위해 확보한 공간
///                /   /  length : 실제 string 길이
///               /   /   /
///             +–––+–––+–––+
/// stack frame │ • │ 8 │ 6 │ <- my_name: String
///             +–│–+–––+–––+
///               │
///             [–│–––––––– capacity –––––––––––]
///               │
///             +–V–+–––+–––+–––+–––+–––+–––+–––+
///        heap │ P │ a │ s │ c │ a │ l │   │   │
///             +–––+–––+–––+–––+–––+–––+–––+–––+
///             [––––––– length ––––––––]
/// ============
/// 
///  * &str (string slices) 의 저장 구조
/// === e.g.===
/// let mut my_name = "Pascal".to_string();
/// my_name.push_str( " Precht");
/// let last_name = &my_name[7..];
/// 
///             my_name: String   last_name: &str
///             [––––––––––––]    [–––––––]
///             +–––+––––+––––+   +–––+–––+
/// stack frame │ • │ 16 │ 13 │...│ • │ 6 │ 
///             +–│–+––––+––––+   +–│–+–––+
///               │                 │
///               │                 +–––––––––+
///               │                           │
///               │                           │
///               │                         [–│––––––– str –––––––––]
///             +–V–+–––+–––+–––+–––+–––+–––+–V–+–––+–––+–––+–––+–––+–––+–––+–––+
///        heap │ P │ a │ s │ c │ a │ l │   │ P │ r │ e │ c │ h │ t │   │   │   │
///             +–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+
/// ============
/// 
/// &str 는 "string slices" 의미 그대로 이미 저장된 문자열의 일부(또는 전체)를 의미하므로 
///   - capacity 에 대한 정보가 필요없다. 
///   - 문자열에 대한 ownership 이 없으므로, 의도하지 않은 ownership 변경, value 변경을 막을 수 있다.
///   - 문자열 일부를 참조하는 작업이 직관적이고 유연하게 이루어진다. 
///     -> String type 으로 문자열 일부분를 다루기 위해서는 
///        my_name.as_bytes()  (&[u8] type)으로 변경해야 한다. (https://rinthel.github.io/rust-lang-book-ko/ch04-03-slices.html)

#[test]
pub fn false_sharing() {
    use std::sync::Arc;
    use std::cell::UnsafeCell;
    use std::thread;

    

    pub fn _cache_line_sharing(arr: [i32; 128], pos: usize) -> (i32, i32) {    
    
        struct SyncWrapper(UnsafeCell<[i32; 128]>);
        unsafe impl Sync for SyncWrapper {}
    
        assert_ne!(pos, 0);
        let arr = Arc::new(SyncWrapper(UnsafeCell::new(arr)));
        let handles: Vec<_> = (0..4)
            .map(|thread_number| {
                let arr = arr.clone();
                let pos = thread_number * pos;
                thread::spawn(move || unsafe {
                    let p: *mut i32 = &mut (*arr.0.get())[pos];
                    for _ in 0..1_000_000 {
                        p.write_volatile(p.read_volatile().wrapping_add(3));
                    }
                })
            })
            .collect();
    
        for handle in handles {
            handle.join().unwrap();
        }
    
        let arr = unsafe { *arr.0.get() };
        (arr[0], arr[1])
    }
    
}

/// 
/// 
struct _CloningLab {
    subject: Vec<Box<&'static dyn Mammal>>,
}

trait Mammal {
    fn walk(&self);
    fn run(&self);
}

#[derive(Clone)]
struct Cat {
    _meow_factor: u8,
    _purr_factor: u8,
}

impl Mammal for Cat {
    fn walk(&self) {
        println!("Cat::walk");
    }

    fn run(&self) {
        println!("Cat::run");
    }
}

