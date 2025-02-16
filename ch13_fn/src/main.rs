/**
 * 4.3 函数基础与 Copy 值
 */
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn change_i32(mut x: i32) {
    x += 4;
    println!("change x: {x}");
}

fn modify_i32(x: &mut i32) {
    *x += 4;
    println!("modify x: {x}");
}

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32
}

fn print_point(point: Point) {
    println!("point x {}", point.x);
}

fn main() {
    let a = 1;
    let b = 2;
    let c = add(a, b);
    println!("c: {c}");

    let x = 1;
    change_i32(x);
    println!("x: {x}"); // 1
    let mut x = 1;
    modify_i32(&mut x);
    println!("x: {x}"); // 5

    let s = Point {
        x: 1,
        y: 2
    };
    print_point(s); // move
    println!("{}", s.x);
}
