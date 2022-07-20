struct Component<'a> {
    data: &'a Vec<i32>,
}

impl<'a> Component<'a> {
    fn new(data: &'a Vec<i32>) -> Self {
        Self { data }
    }

    fn get_data(&self) -> &'a Vec<i32> {
        &self.data
    }
}

#[test]
fn example() {
    let mut data = vec![1, 2, 3, 4, 5];
    let component_1 = Component::new(&data);
    let component_2 = Component::new(&data);
    println!("data: {:?}", &data);
    println!("component_1: {:?}", component_1.get_data());
    println!("component_2: {:?}", component_2.get_data());

    // 컴파일 불가
    // data.push(6);
    
    // 컴파일 가능
    let mut data = vec![1, 2, 3, 4, 5, 6];
    println!("data: {:?}", &data);
    println!("component_1: {:?}", component_1.get_data());
    println!("component_2: {:?}", component_2.get_data());
}

// 위 example 에서 data: Vec<i32> 의 변화를 공유하도록 만들기.
use std::cell::RefCell;
use std::rc::Rc;

#[test]
pub fn example_1() {
    let data = Rc::new(RefCell::new(vec![1, 2, 3, 4, 5]));
    let component_1 = Rc::clone(&data);
    let component_2 = data.clone();
    dbg!(Rc::strong_count(&data));
    {
        let mut data_1 = data.borrow_mut();
        data_1.push(6);
    }
    println!("data: {:?}", &data);
    println!("component_1: {:?}", &component_1);
    println!("component_2: {:?}", *component_2);
}

// Rc<RefCell<T>> 를 통해서 구현.
#[derive(Debug)]
struct SharedStruct {
    data: RefCell<Vec<i32>>,
}

impl SharedStruct {
    pub fn new(data: Vec<i32>) -> Rc<Self> {
        Rc::new(Self { data: RefCell::new(data), })
    }

    pub fn push(&self, number: i32) {
        let mut data = self.data.borrow_mut();
        data.push(number);
    }
}

#[test]
fn example_2() {
    let component_1 = SharedStruct::new(vec![1, 2, 3, 4, 5]);
    let component_2 = Rc::clone(&component_1);
    println!("{:?}", component_1);
    println!("{:?}", component_2);

    component_2.push(6);
    println!("{:?}", component_1);
    println!("{:?}", component_2);

    component_1.push(7);
    println!("{:?}", component_1);
    println!("{:?}", component_2);
}