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

// (최신 순서 (역순) 으로 정렬)
// ----------------------------------------------------------------
// 02. Generic -> Box & trait : 성공!!
// - VecDeque 에 다양한 type 을 넣을 수 있도록 Box 로 감싸서 넣어 볼수 있을 듯.
// - Box 내부에 어떻게 다양한 type 이 들어갈수 있게 할것인가? (trait ??)
#[allow(non_snake_case)]

#[test]
fn ex02() {
    
    use rand::random;
    use std::collections::VecDeque;
    use std::boxed::Box;

    struct Cage {
        lane: VecDeque<Box<dyn Animal>>,
    }
    
    impl Cage {
        fn new() -> Self {
            Cage { lane: VecDeque::<Box<dyn Animal>>::new() }
        }

        fn get_animal(&self, index: usize) -> String {
            return self.lane.get(index).unwrap().get_name();     
            // 아예 Anminal trait 에 implement 된 각 get_name() 함수를 여기에 넣어서 main 에서 접근 경로를 단축 
        }
    }
    
    trait Animal {
        fn get_name(&self) -> String;       // public 선언이 trait implement 하는 함수에는 적용이 안되는데 why????
    }
    
    #[derive(Clone)]
    struct Elephant {
        name: String,
    }

    impl Elephant {
        fn new() -> Self{
            Self { name: String::new() }
        }

        fn naming(&mut self, name: String) {
            self.name = name;
        }
    }

    impl Animal for Elephant {
        fn get_name(&self) -> String {
            return self.name.clone();
        }
    }

    // trait Animal {
    //     fn new(name: String) -> Self;
    // }
    // impl Animal for Elephant {
    //     fn new(name: String) -> Self { Elephant { name } }
    // } 
    // 위와 같이 trait 에 생성자로 implement 하려고 했더니 다음의 에러가 메세지를 볼 수 있다. 
    // for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
    // consider turning `new` into a method by giving it a `&self` argument: `&self, `
    // alternatively, consider constraining `new` so it does not apply to trait objects: ` where Self: Sized`
    //
    // 해당 오류를 통해 (확실하지는 않지만) 다음과 같이 유추 해볼 수 있다. 
    // -> dynamic dispach 를 위한 vtable 은 생성된 instance 를 기준으로 만들어 진다. 
    // -> 따라서 instance 를 참조하는 '&self, &mut self' 를 argument 로 갖는 (생성자 외) method 가 trait 에 implement 되어야 dynamic dispach 가 가능하다.

    #[derive(Clone)]
    struct Lion {
        name: String,
    }
    
    impl Lion {
        fn new() -> Self {
            Lion { name: String::new() }
        }

        fn naming(&mut self, name: String) {
            self.name = name;
        }
    }

    impl Animal for Lion {
        fn get_name(&self) -> String {
            return self.name.clone();
        }
    }
    
    #[derive(Clone)]
    struct Hippo {
        name: String,
    }
    
    impl Hippo {
        fn new() -> Self {
            Hippo { name: String::new() }
        }

        fn naming(&mut self, name: String) {
            self.name = name;
        }
    }
    
    impl Animal for Hippo {
        fn get_name(&self) -> String {
            return self.name.clone();
        }
    }

    // -- main --
    let mut hoons_LA_house = Cage::new();

    let mut elephant = Elephant::new();
    let mut lion = Lion::new();
    let mut hippo = Hippo::new();

    for i in 0..10 {
        let rand_num = random::<u8>() % 3;
        match rand_num {
            0 => {
                elephant.naming(format!("Elephant"));
                hoons_LA_house.lane.push_back(Box::new(elephant.clone()));
            },
            1 => {
                lion.naming(format!("Lion"));
                hoons_LA_house.lane.push_back(Box::new(lion.clone()));
            },
            2 => {
                hippo.naming(format!("Hippo"));
                hoons_LA_house.lane.push_back(Box::new(hippo.clone()));
            },
            _ => print!("impossible error"),
        }

        if i > 0 {
            // dynamic dispatch 로 value 를 넣었을 때.. 해당 value 에 어떻게 다시 접근할까?
            // 사부님 말씀대로 Deref 로 dyn Animal type 까지 들어가면, trait 에 implement 된 method 로 접근 가능해진다. 
            // 따라서, trait impl method 에 field 에 접근하여 value 를 가져올 수 있는 함수를 걸어놓고 거기로 접근하여 해당 value 를 가져온다. 
            let previous = hoons_LA_house.get_animal(i - 1);
            let present = hoons_LA_house.get_animal(i);

            if previous[..] == present[..] {
                println!("{}{} and {}{} fight", &previous, i - 1 , &present, i);
            }
        } 
    }
}
// ----------------------------------------------------------------

// ----------------------------------------------------------------
// 01. Generic 만으로 구성 (trait 사용 X) -> 실패!! (학습기록으로 남겨놓음)
// - Cage struct 에서 type 에 대한 문제가 발생하지는 않음. 
// - 그러나 해당 method 를 하나의 type 사용하면 다른 type 들은 쓸수 없게 되어 버림.
/* 
#[test]
fn ex01() {
    use rand::random;
    use std::collections::VecDeque;
    use std::boxed::Box;

    struct Cage<T> {
        lane: VecDeque<T>,
    }
    
    impl <T> Cage<T> {
        fn new() -> Self {
            Cage { lane: VecDeque::<T>::new() } 
        }
    }
    
    struct Elephant {
        name: String,
    }
    
    impl Elephant {
        fn new(name: String) -> Self {
            Elephant { name }
        }
    }
    
    struct Lion {
        name: String,
    }
    
    impl Lion {
        fn new(name: String) -> Self {
            Lion { name }
        }
    }
    
    struct Hippo {
        name: String,
    }
    
    impl Hippo {
        fn new(name: String) -> Self {
            Hippo { name }
        }
    }

    // -- main --
    let mut hoons_LA_house = Cage::new();

    for i in 0..10 {
        let rand_num = random::<u8>() % 3;

        match rand_num {
            0 => hoons_LA_house.lane.push_back(Box::new(Elephant::new(format!("{}: Elephant", i)))),
            // 1 => hoons_LA_house.lane.push_back(Box::new(Lion::new(format!("{}: Lion", i)))),         // error 발생!!
            // 2 => hoons_LA_house.lane.push_back(Box::new(Hippo::new(format!("{}: Hippo", i)))),       // error 발생!!
            _ => print!("impossible error"),
        }        
    }
}
*/
// ----------------------------------------------------------------




// -- 실험실 --
#[test]
fn test01() {
    let x = Box::new(41);
    let y = *x + 1;
    println!("{}", &y);
}