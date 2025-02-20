/**
 * 9.2 Intoiterator、Iterator 和 Iter 之间的关系
 */
fn main() {
    // vec
    let v = vec![1, 2, 3, 4, 5]; // intoInterator，具有特质 into_iter
    // 转换为迭代器 iter, 为 iterator 的特质对象
    let iter = v.into_iter(); // 发生 move 所有权转移
    let sum: i32 = iter.sum();
    println!("sum: {}", sum);
    // println!("{:?}", v); // error

    // array
    let array = [1, 2, 3, 4, 5];
    let iter: std::slice::Iter<'_, i32> = array.iter(); // 所有权不转移
    // let iter = array.iter();
    let sum = iter.sum::<i32>();
    // let sum: i32 = iter.sum();
    println!("sum: {}", sum);
    println!("{:?}", array);

    // chars
    let text = "hello world";
    let iter = text.chars(); // 实现了 iterator 特质
    let uppercase = iter.map(|c| c.to_ascii_uppercase()).collect::<String>();  // 所有权不转移
    // let uppercase: String = iter.map(|c|c.to_ascii_uppercase()).collect();
    println!("uppercase: {}", uppercase);
    println!("text: {}", text);
}
