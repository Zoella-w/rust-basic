/**
 * 9.3 获取迭代器的三种方式
 */
fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    // 返回一个不可变引用的迭代器
    for &item in vec.iter() {
        println!("{}", item);
    }
    println!("{:?}", vec);

    // 可变引用（尽量不写）
    let mut vec = vec![1, 2, 3, 4, 5];
    for item in vec.iter_mut() {
        *item *= 2;
    }
    println!("{:?}", vec);

    // 所有权转移（尽量写）
    let vec = vec![1, 2, 3, 4, 5];
    for item in vec.into_iter() {
        println!("{}", item);
    }
    // println!("{:?}", vec); // error
}
