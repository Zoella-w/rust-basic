/**
 * 8.2 Trait Object 与 Box
 */
// trait 不可变引用 和 move
struct Obj {}

trait Overview {
    // 默认实现
    fn overview(&self) -> String {
        String::from("overview")
    }
}

impl Overview for Obj {
    fn overview(&self) -> String {
        String::from("Obj")
    }
}

// 1.不可变引用
fn call_obj(item: &impl Overview) {
    println!("Overview {}", item.overview());
}
// 2.move
fn call_obj_box(item: Box<dyn Overview>) {
    println!("Overview {}", item.overview());
}

trait Sale {
    fn amount(&self) -> f64;
}

struct Common(f64);
impl Sale for Common {
    fn amount(&self) -> f64 {
        self.0
    }
}

struct TenDiscount(f64);
impl Sale for TenDiscount {
    fn amount(&self) -> f64 {
        self.0 - 10.0
    }
}

struct TenPercentDiscount(f64);
impl Sale for TenPercentDiscount {
    fn amount(&self) -> f64 {
        self.0 * 0.9
    }
}

// Vec：表示一个动态数组，可以存储多个元素
// Box<dyn Sale>：表示这些元素是在堆上分配的；dyn Sale 表示这些元素是实现了 Sale 特质的对象（允许存储不同具体类型的对象，只要都实现了 Sale 特质）
// &：表示对 Vec<Box<dyn Sale>> 的引用
fn calculate(sales: &Vec<Box<dyn Sale>>) -> f64 {
    sales.iter().map(|sale| sale.amount()).sum()
}

fn main() {
    let a = Obj {};
    call_obj(&a);
    println!("{}", a.overview());
    let b_a = Box::new(Obj {});
    call_obj_box(b_a);
    // println!("{}", b_a.overview()); // error

    // let c: Box<dyn Sale> = Box::new(Common(100.0));
    // let t1: Box<dyn Sale> = Box::new(TenDiscount(100.0));
    // let t2: Box<dyn Sale> = Box::new(TenPercentDiscount(100.0));
    // let sales = vec![c, t1, t2];
    let c = Box::new(Common(100.0));
    let t1 = Box::new(TenDiscount(100.0));
    let t2 = Box::new(TenPercentDiscount(200.0));
    let sales: Vec<Box<dyn Sale>> = vec![c, t1, t2]; // c t1 t2 不再使用
    println!("pay {}", calculate(&sales));
}
