/**
 * 10.4 闭包类型 FnOnce FnMut Fn 的实例
 */
fn closure_fn<F>(func: F)
where
    F: Fn(),
{
    func();
    func();
}

fn closure_fn_mut<F>(mut func: F)
where
    F: FnMut(),
{
    func();
    func();
}

fn closure_fn_once<F>(mut func: F)
where
    F: FnOnce(),
{
    func();
}

fn main() {
    // 不可变引用
    let s1 = String::from("11111");
    closure_fn(|| println!("{}", s1));

    // 可变引用（尽量不用）
    let s1 = String::from("11111");
    closure_fn_mut(|| println!("{}", s1));

    let mut s2 = String::from("22222");
    closure_fn_mut(|| {
        s2.push_str("000");
        println!("{}", s2);
    });
    println!("{}", s2);

    // 所有权转移
    let s1 = String::from("11111");
    closure_fn_once(|| println!("{}", s1));

    let mut s2 = String::from("22222");
    closure_fn_once(|| {
        s2.push_str("000");
        println!("{}", s2);
    });
    println!("{}", s2);

    let s3 = "ff".to_owned();
    closure_fn_once(move || println!("{s3}"));
    // println!("{s3}"); // error
}
