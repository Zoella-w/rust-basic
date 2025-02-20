/**
 * 8.5 trait 多态和继承
 */
use std::collections::VecDeque;

// 1. 多态
trait Driver {
    fn drive(&self);
}

struct Car;
impl Driver for Car {
    fn drive(&self) {
        println!("Car is driving");
    }
}

struct SUV;
impl Driver for SUV {
    fn drive(&self) {
        println!("SUV is driving");
    }
}

// &dyn Driver 表示 vehicle 是实现了 Driver 特质的对象
fn road(vehicle: &dyn Driver) {
    vehicle.drive();
}

// 2. 继承（思想）
// 单项特质
trait Queue {
    fn len(&self) -> usize;
    fn push_back(&mut self, n: i32);
    fn pop_front(&mut self) -> Option<i32>;
}

// 双向特质
trait Deque: Queue {
    fn push_front(&mut self, n: i32);
    fn pop_back(&mut self) -> Option<i32>;
}

#[derive(Debug)]
struct List {
    data: VecDeque<i32>,
}

impl List {
    fn new() -> Self {
        let data = VecDeque::<i32>::new();
        Self { data }
    }
}

impl Deque for List {
    fn push_front(&mut self, n: i32) {
        self.data.push_front(n);
    }
    fn pop_back(&mut self) -> Option<i32> {
        self.data.pop_back()
    }
}

impl Queue for List {
    fn len(&self) -> usize {
        self.data.len()
    }
    fn push_back(&mut self, n: i32) {
        self.data.push_back(n);
    }
    fn pop_front(&mut self) -> Option<i32> {
        self.data.pop_front()
    }
}

fn main() {
    road(&Car);
    road(&SUV);

    let mut list = List::new();
    list.push_back(1);
    list.push_front(0);
    println!("{:?}", list);
    list.push_front(2);
    list.push_back(2);
    println!("{:?}", list);
    println!("{}", list.pop_back().unwrap()); // .unwrap() 从 Option 中取值
    println!("{:?}", list);
}
