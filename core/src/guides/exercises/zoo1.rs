/// 동물원에는 코끼리, 하마 그리고 사자 세 종류의 동물이 산다. 낮에는 자유롭게 뛰놀지만 
/// 밤이 되면 다시 사육장에 들어가야 한다. 사육장에 들어갈 때 한 통로로 모든 동물이 같이 들어가며
/// 같은 종류의 동물 2 마리가 연달아 들어가게 되면 서로 싸우게 된다.
/// 
/// 사육장(Cage) 에 들어가는 동물은 모두 고유한 이름을 가지며 한 줄로 
/// 코끼리, 사자 그리고 하마 타입을 push 하고 pop 할 때 같은 타입이 2 번 연달아 나오면 특정 문장을 
/// 출력하는 FIFO queue 를 구현하는 것이 목적이며 아래 코드를 자유롭게 "확장" 해서 구현.
/// 
/// 예)
/// Hippo1 - Lion1 - Elephant1 - Elephant2 - Hippo1
/// 출력 => "Elephant1 and Elephant2 fight."
use std::collections::VecDeque;
use std::boxed::Box;

struct Cage {
    lane: VecDeque<>,
}

pub trait Animal {}

struct Elephant {
    name: String,
}

struct Lion {
    name: String,
}

struct Hippo {
    name: String,
}

#[test]
fn zoo1() {
    todo!();
}