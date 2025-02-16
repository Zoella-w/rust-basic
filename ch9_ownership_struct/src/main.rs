/**
 * 3.5 Ownership 与结构体
 */
struct Counter {
    number: i32,
}

impl Counter {
    // 1. 不可变借用
    fn new(number: i32) -> Self {
        Counter { number }
        // Self { number }
    }
    // fn new(number: i32) -> Counter {
    //     Counter { number }
    // }

    // 2. 可变借用
    fn get_number(&self) -> i32 {
        self.number
    } // Counter::get_number(self: &Self)

    // 3. move
    // 无返回
    fn add(&mut self, increment: i32) {
        self.number += increment;
    } // Counter::add(self: &mut Self, increment)

    fn give_up(self) {
        println!("free {}", self.number);
    }

    // 关联函数
    fn combine(c1: Self, c2: Self) -> Self {
        Self {
            number: c1.number + c2.number,
        }
    }
    // fn combine(c1: Counter, c2: Counter) -> Counter {
    //     Counter {
    //         number: c1.number + c2.number,
    //     }
    // }
}

fn main() {
    // 不可变借用
    let mut c1 = Counter::new(0);
    println!("{}", c1.get_number());
    println!("{}", c1.get_number());

    // 可变借用
    c1.add(2);
    println!("{}", c1.get_number());
    println!("{}", c1.get_number());

    // move
    c1.give_up();
    // println!("{}", c1.get_number()); // error

    let c1 = Counter::new(2);
    let c2 = Counter::new(1);
    let c3 = Counter::combine(c1, c2);
    // println!("{}", c1.get_number()); // error
    // println!("{}", c2.get_number()); // error
    println!("{}", c3.get_number()); // error
}
