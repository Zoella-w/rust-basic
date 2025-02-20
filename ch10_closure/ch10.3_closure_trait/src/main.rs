/**
 * 10.3 闭包底层是怎么工作的
 */
// 不可变引用
fn apply_closure<F: Fn(i32, i32) -> i32>(closure: F, x: i32, y: i32) -> i32 {
    closure(x, y)
}
// 可变引用（尽量不用）
fn apply_closure_mut<F: FnMut(i32, i32) -> i32>(mut closure: F, x: i32, y: i32) -> i32 {
    closure(x, y)
}

fn main() {
    let x = 5;
    let add_closure = |a, b| {
        // x++; // error
        println!("x: {x}");
        a + b + x
    };
    let result = apply_closure(add_closure, 5, 6);
    println!("{result}");

    let mut x = 5;
    let mut add_closure = |a, b| {
        x = x + 1; // error
        println!("x: {x}");
        a + b + x
    };
    let result = apply_closure_mut(add_closure, 5, 6);
    println!("{result}");
}
