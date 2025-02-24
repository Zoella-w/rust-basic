/**
 * 6.1 Borrow checker
 */
fn main() {
    let mut s = String::from("Hello");
    // 不可变引用：可以同时拥有多个
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);
    // 有可变引用，不可变引用不可用
    // println!("{} {}", r1, r2); // error

    let result1: &str;
    let result2: &str;
    {
        result1 = "oo"; // 可以在这里初始化
        let r4 = &s;
        result2 = ff(r4);
    }
    println!("{}", result1); // oo
    // println!("{}", r4); // 无法打印，r4 生命周期已结束
    println!("{}", result2); // Hello
}

// fn ff<'a>(s: &'a str) -> &'a str {
//     s
// }
fn ff(s: &str) -> &str {
    s
}
