/**
 * 4.4 函数不可变借用、可变借用
 */
fn move_func(p1: i32, p2: String) {
    println!("p1 is {}", p1);
    println!("p2 is {}", p2);
}

// 借用（这里无意义）
fn print_value(value: &i32) {
    println!("{value}");
}

fn string_func_borrow(s: &String) {
    println!("{}", (*s).to_uppercase());
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

fn modify_point(point: &mut Point) {
    (*point).x += 2;
    point.y += 2;
    // 以上两种写法都可以
}

fn main() {
    let mut n = 12;
    let s = String::from("oo");
    move_func(n, s);
    println!("n is {}", n);
    // println!("s is {}", s); // error

    print_value(&mut n);
    let mut s = String::from("oo");
    string_func_borrow(&mut s);
    println!("n is {}", n);
    println!("s is {}", s);

    let mut p = Point{
        x: 0,
        y: 0
    };
    // 调用 debug 特质
    println!("{:?}", p);
    modify_point(&mut p);
    println!("{:?}", p);
}
