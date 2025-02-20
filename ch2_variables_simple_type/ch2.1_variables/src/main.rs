/**
 * 常用命令
 * 1、编译器rustc
 * rustc --version // 查看版本
 * rustc -o output_filename filename.rs // 编译生成二进制文件
 * rustc --crate-type lib filename.rs // 编译生成库文件
 * 
 * 2、包管理工具cargo（隐式使用rustc编译）
 * cargo new project_name // 创建
 *     cargo new --lib project_name // 创建rust库项目
 * cargo build // 构建（生成二进制可执行或库文件）
 *     cargo build --release // 生成优化的可执行文件，用于生产环境
 * cargo check // 检测
 * cargo run/cargo test  // 运行/测试
 * 
 * 3、插件cargo-edit
 * cargo install cargo-edit // 安装
 * cargo add dependency_name // 添加库
 *     cargo add dependency_name@1.2.3 // 指定版本
 *     cargo add --dev dependency_name // 开发依赖库
 *     cargo add --build build_dependency_name // 构建依赖库
 * cargo rm dependency_name // 删除库
 */

/**
 * 2.1 变量与不可变性
 */
fn main() {
    // 不可变与命名
    let _nice_count = 100; // 自动推导i32
    let _nice_number: i64 = 54;
    // nice_count = 23; // error

    // 声明可变
    let mut _count = 3;
    _count = 4;

    // shadowing
    let x = 5;
    {
        // 命名空间（作用域）
        let x = 10;
        println!("inner x : {}", x);
    } // 内部的 x 被销毁
    println!("Outer x: {x}");

    // 在同一作用域下重新声明x，覆盖了之前的 x
    let x = "hello"; // 切片的引用
    println!("New x: {x}");
    // x = "hello1"; // error

    // 可以重定义类型和可变性
    let mut x = "this";
    println!("x: {x}");
    x = "that";
    println!("Mut x: {x}");
}