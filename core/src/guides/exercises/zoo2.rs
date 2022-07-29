/// zoo1.rs 에서 동물들이 스스로 들어가는 것처럼 보였지만 사실 Zookeeper 의 도움으로 들어갔다. 
/// 같은 타입의 동물 두 마리가 연속으로 들어갈 때 싸움이 일어나는 것을 확인한 Zookeeper 는 서로 
/// 서로 다른 타입의 동물을 순서대로 넣어야 무사히 모든 동물을 Cage 에 넣을 수 있다고 판단했고
/// 실제로 실행한다.
/// 
/// 하지만 문제가 발생했다. 혼자서 Cage 에 넣을 때에는 아무 문제가 없었지만 일을 빨리 하기 위해 
/// 두 명이 동시에 Cage 에 동물을 넣을 때 같은 동물이 연달아 들어가는 경우가 발생했기 때문이다.
/// 여러명의 Zookeeper 가 같은 타입의 동물을 연속으로 넣어 싸움이 발생하는 경우를 방지하기 위해
/// 공유 가능한 lane 을 만들어서 협업하기로 결정했다. 아래의 구조를 확장해서 구현해보자.
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

trait Animal {}

struct Elephant;

struct Hippo;

struct Lion;

struct Zookeeper {
    lane: todo!(),
}

impl Zookeeper {
    // 공유 가능한 mutable lane 으로 초기화
    pub fn new(lane: todo!()) -> Self {
        Self { lane }
    }

    pub fn push(&self) -> Result<todo!()> {
        // 특별한 push 를 해야 하는데 먼저 lane 의 마지막에 있는 동물의 타입을 확인 후
        // 다른 타입의 동물만 push 하도록 한다. 만약 같은 타입의 동물을 push 한다면 panic!().
        todo!();
    }

    // drain 함수는 구현하지 않는다.
}

#[test]
fn zoo2() {
    // 2 명 이상의 Zookeeper 가 하나의 lane 에 같은 타입의 동물을 연달아 넣지 않도록 구현.
    let lane = VecDeque::<Box<dyn Animal>>::new();
    zookeeper1 = Zookeeper::new(todo!());
    zookeeper2 = Zookeeper::new(todo!());
    todo!();
}