/**
 *  5.3 自定义一个 Error 类型
 */
#[derive(Debug)]
 struct MyError {
    detail: String,
}

// 使得能够打印可读的错误信息
impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Custom Error: {}", self.detail)
    }
}

// 使得 MyError 符合 Rust 的错误处理规范
impl std::error::Error for MyError {
    // 以下可以省略
    // fn description(& self) -> &str {
    //     &self.detail // &String => &str
    // }
    
}

fn func() -> Result<(), MyError> {
    Err(MyError {
        detail: "CustomError".to_owned()
    })
    // Ok(())
}

// fn main() -> Result<(), MyError> {
fn main() -> Result<(), Box<dyn std::error::Error>> {
    match func() {
        Ok(_) => println!("func ok"),
        Err(err) => println!("Error: {}", err)
    }
    // 如果 func() 返回 Err，则自动返回到主函数中的错误，并且不会执行下面的代码；
    // 如果返回 Ok，则继续执行
    func()?;
    println!("oo");

    Ok(())
}
