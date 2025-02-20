/**
 * 10.2 闭包获取参数
 */
fn main() {
    // 不可变引用获取外部参数
    let s1 = String::from("111111");
    let s2 = String::from("222222");
    let fn_func = |s| {
        println!("{s1}");
        println!("{s2}");
        println!("{s}");
    };
    fn_func("yz".to_owned());
    fn_func("yzyzyz".to_owned());
    println!("{s1} {s2}");

    // 可变引用获取外部参数
    let mut s1 = String::from("111111");
    let mut s2 = String::from("222222");
    // 闭包实际上是一个结构体，也需要进行 mut 声明
    let mut fn_func = |s| {
        s1.push_str("333");
        s2.push_str("333");
        println!("{s1}");
        println!("{s2}");
        println!("{s}");
    };
    fn_func("yz".to_owned());
    fn_func("yzyzyz".to_owned());
    println!("{s1} {s2}");

    // 所有权转移
    let s1 = String::from("111");
    let fn_once_func = || {
        println!("{s1}");
        std::mem::drop(s1); // 销毁
    };
    fn_once_func();
    // fn_once_func(); // error
    // println!("{s1}"); // error

    // 强制所有权转移
    let s1 = String::from("1111");
    // move 将闭包中外来的变量销毁
    let move_fn = move || {
        println!("{s1}");
    };
    move_fn(); // 可以调用多次
    // println!("{s1}"); // error

    // 线程，也是强制所有权转移
    let s1 = String::from("1111222");
    std::thread::spawn(move || println!("{s1}"));
}
