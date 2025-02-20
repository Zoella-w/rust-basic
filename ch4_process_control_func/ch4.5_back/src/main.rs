/**
 * 4.5 函数返回值与所有权机制
 */
fn func_copy_back() -> i32 {
    let n = 42;
    n    
}

fn func_non_copy_back() -> String {
    let s = String::from("hello");
    s    
}

// 使用 'static 无需标注生命周期
fn get_mess(mark: i32) -> &'static str {
    if mark == 0 {
        "😊"
    } else {
        "😭"
    }
}

fn main() {
    let i = func_copy_back();
    println!("{i}");
    let s = func_non_copy_back();
    println!("{s}");
    println!("{}", get_mess(0));
}
