/**
 * 3.1 Rust 内存管理模型
 */
fn get_length(s: String) -> usize {
    println!("String: {s}");
    // main::s1 也销毁了
    return s.len();
}

fn main() {
    // copy, move
    // 1. copy
    let c1 = 1;
    let c2 = c1;
    println!("c1: {c1}");

    // 2. move
    let s1 = String::from("value");
    // let s2 = s1; // s1 的所有权转移给 s2
    // println!("s1: {s1}"); // error
    let s2 = s1.clone(); // 相当于深拷贝
    println!("s1: {s1}");

    // get_length(s1);
    // println!("s1: {s1}"); // error
    let len = get_length(s1);
    println!("s1.len: {len}");

    let back = first_word("hello world");
    println!("back: {back}");
}

// 如何返回指针？
fn dangle() -> String {
    return "hello".to_owned();
}

// 不推荐：静态生命周期，污染全局作用域
fn dangle_static() -> &'static str {
    return "asd";
}

// String 与 str vec u8 ref
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        // 是否为空格的 u8 值
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}