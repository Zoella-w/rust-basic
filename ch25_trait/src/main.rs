/**
 * 8.1 特质
 */
trait Greeter {
    fn greet(&self);
    // 默认方法
    fn hello() {
        println!("hello");
    }
}

struct Person {
    name: String
}

impl Greeter for Person {
    fn greet(&self) {
        println!("{}", self.name);
    }

    // fn hello() {
    //     println!("hello");
    // }
}

fn main() {
    let person = Person {
        name: "Yz".to_owned()
    };
    person.greet();
    Person::hello();
}
