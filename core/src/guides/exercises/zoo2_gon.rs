/// zoo1.rs 에서 동물들이 스스로 들어가는 것처럼 보였지만 사실 Zookeeper 의 도움으로 들어갔다. 
/// 같은 타입의 동물 두 마리가 연속으로 들어갈 때 싸움이 일어나는 것을 확인한 Zookeeper 는 서로 
/// 서로 다른 타입의 동물을 순서대로 넣어야 무사히 모든 동물을 Cage 에 넣을 수 있다고 판단했고
/// 실제로 실행한다.
/// 
/// 하지만 문제가 발생했다. 혼자서 Cage 에 넣을 때에는 아무 문제가 없었지만 일을 빨리 하기 위해 
/// 두 명이 동시에 Cage 에 동물을 넣을 때 같은 동물이 연달아 들어가는 경우가 발생했기 때문이다.
/// 여러명의 Zookeeper 가 같은 타입의 동물을 연속으로 넣어 싸움이 발생하는 경우를 방지하기 위해
/// 공유 가능한 lane 을 만들어서 협업하기로 결정했다. 아래의 구조를 확장해서 구현해보자.

#[allow(unused_variables)]

use std::boxed::Box;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;
use std::rc::Rc;

#[allow(unused_imports)]
use rand::{thread_rng, Rng};

trait Animal {
    fn get_name(&self) -> &str;
}

// Animal trait 에 bound 될 동물 type 정의
#[derive(Clone)]
struct Elephant;

impl Elephant {
    fn new() -> Self {
        Self{}
    }
}

impl Animal for Elephant {
    fn get_name(&self) -> &str {
        "Elephant"
    }
}

struct Hippo;

impl Hippo{
    fn new() -> Self{
        Self{}
    }
}

impl Animal for Hippo {
    fn get_name(&self) -> &str {
        "Hippo"
    }
}

struct Lion;

impl Lion {
    fn new() -> Self {
        Self{}
    }
}

impl Animal for Lion {
    fn get_name(&self) -> &str {
        "Lion"
    }
}

// 공유 lane 을 field 로 가지면서 작업을 진행하는 type 정의
struct Zookeeper {
    shared_lane: Rc<RefCell<VecDeque<Box<dyn Animal>>>>,
}

impl Zookeeper {
    // 공유 가능한 mutable lane 으로 초기화
    pub fn new(shared_lane: Rc<RefCell<VecDeque<Box<dyn Animal>>>>) -> Self {
        Self { shared_lane }
    }

    pub fn push(&mut self, animal: Box<dyn Animal>) -> Result<(), FightError> {
        // 특별한 push 를 해야 하는데 먼저 lane 의 마지막에 있는 동물의 타입을 확인 후
        // 다른 타입의 동물만 push 하도록 한다. 만약 같은 타입의 동물을 push 한다면 panic!().
        
        // let previous = self.shared_lane.borrow(); 
        // 해당 code 는 단순 가독성을 위해 작성하였으나, RefCell borrow method 를 사용하여 variable 에 대입할 경우,
        // borrowing 상태가 유지되어 아래에서 borrow_mut 을 사용할 수 없게된다. 
        // 따라서 해당 code 를 match 문에 직접 대입하여, borrow 상태를 바로 해제할 수 있도록 해줘야 함. 
        
        #[allow(unused_assignments)]
        let mut previous_animal = "";

        match self.shared_lane.borrow().back() {
            None => println!("log  \t: it's a first animal."),
            Some(x) => {
                previous_animal = x.get_name();       // 현재 lane 가장 뒤에 있는 동물
                if &previous_animal == &animal.get_name() {
                    println!("Warning\t: The same animals ({}) are putting into the cage in a row", previous_animal);
                    return Err(FightError);
                }
            },
        }
        println!("log  \t: Pushing back {} in the lane", &animal.get_name());
        self.shared_lane.borrow_mut().push_back(animal);
        return Ok(());

    }
    // drain 함수는 구현하지 않는다.
}

// 사용자 정의 Error triat bound
struct FightError;

impl fmt::Display for FightError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error : Fight This work must be stopped.")
    }
} 

impl fmt::Debug for FightError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error : Fight This work must be stopped.")
    }
}

impl std::error::Error for FightError {}


#[test]
fn zoo2() {
    // 2 명 이상의 Zookeeper 가 하나의 lane 에 같은 타입의 동물을 연달아 넣지 않도록 구현.
    let lane = VecDeque::<Box<dyn Animal>>::new();
    let shared_lane = Rc::new(RefCell::new(lane));
    // 위 두 variable 에 대해, VecDeque push_back 기능이 구현되어 있으므로, mutable 로 선언했었으나,
    // 불필요한 것으로 warnig 되어 제거함. 
    // shared_lane 의 경우, 전체가 Rc<RefCell<>> smart pointer 로 감싸져있으므로 mut 의 의미가 없음을 알수 있다. 
    // 그러나 VecDeque 인 lane 에 대해서는... 모르겠다.. ;;;

    let mut zookeeper1 = Zookeeper::new(shared_lane.clone());
    let mut zookeeper2 = Zookeeper::new(shared_lane.clone());

    let mut rng = thread_rng();

    loop{
        let which_keeper: u8 = rng.gen_range(1..3);
        let which_animal: u8 = rng.gen_range(0..3);

        // 우선 mutithreading 환경을 배제하고 작성하다보니까... 
        // zookeeper 들의 작업이 랜덤하게 번갈아 가면서 작업되는 환경으로 작성
        match which_keeper {
            1 => {
                println!("log  \t: Worker - zookeeper 1");
                match which_animal {
                    0 => zookeeper1.push(Box::new(Elephant::new())).unwrap(),
                    1 => zookeeper1.push(Box::new(Hippo::new())).unwrap(),
                    2 => zookeeper1.push(Box::new(Lion::new())).unwrap(),
                    _ => println!("impossible error"),    // 실행 불가
                }
            },
            2 => {
                println!("log  \t: Worker - zookeeper 2");
                match which_animal {
                    0 => zookeeper2.push(Box::new(Elephant::new())).unwrap(),
                    1 => zookeeper2.push(Box::new(Hippo::new())).unwrap(),
                    2 => zookeeper2.push(Box::new(Lion::new())).unwrap(),
                    _ => println!("impossible error"),
                }       
            },
            _ => println!("impossible error"),
        }
    }



}


//----------------------------------------------------
// warning message 를 제거하기 위한 dummy code

#[allow(unused_features)]
pub fn dummy() {
    let e = Elephant::new();
    let _h = Hippo::new();
    let _l = Lion::new();
    let mut z = Zookeeper::new(Rc::new(RefCell::new(VecDeque::<Box<dyn Animal>>::new())));
    let _ = z.push(Box::new(e)).unwrap();
}




// -- 실험실 --
#[test]
fn ex01() {
    let mut rng = thread_rng();

    for _ in 0..10 {
        let n: u8 = rng.gen_range(0..3);
        println!("{}", n);    
    }
}

#[test]
fn ex02() {
let mut buf = VecDeque::new();

buf.push_back(Box::new(3));
buf.push_back(Box::new(4));
buf.push_back(Box::new(5));

// dbg!(buf.get(-1).unwrap());   // index -1 은 안먹네..;;; 
}

#[test]
fn ex03() {
    let x = Box::new("abc");

    if *x == "abc" {
        println!("OK");
    }
}