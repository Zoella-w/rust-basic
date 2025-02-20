/**
 * 2.3 基础数据类型
 */
fn main() {
    // 进制的字面量
    let a1 = -125;
    let a2 = 0xFF; // 十六进制
    let a3 = 0o13; // 八进制
    let a4 = 0b10; // 二进制
    println!("{a1} {a2} {a3} {a4}");

    // Max Min
    println!("u32 max: {}", u32::MAX);
    println!("u32 min: {}", u32::MIN);
    println!("i32 max: {}", i32::MAX);
    println!("i32 max: {}", i32::MIN);
    println!("usize max: {}", usize::MAX);
    println!("isize is {} type", std::mem::size_of::<isize>());
    println!("usize is {} type", std::mem::size_of::<usize>());
    println!("u64 is {} type", std::mem::size_of::<u64>());
    println!("i64 is {} type", std::mem::size_of::<i64>());
    println!("u32 is {} type", std::mem::size_of::<u32>());
    println!("i32 is {} type", std::mem::size_of::<i32>());

    // float
    let f1: f32 = 1.2345;
    let f2: f64 = 5.6789;
    println!("Float are {:.2} {:.2}", f1, f2);

    // bool
    let is_ok = true;
    let can_ok: bool = false;

    // char
    let char_A = 'A';
    let emo_char = '😊';
    println!("If you get {char_A}, you feel {emo_char}");
    println!("{}", emo_char as usize); // 128522
    println!("{}", emo_char as i32); // 128522
}
