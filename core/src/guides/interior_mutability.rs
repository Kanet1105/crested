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