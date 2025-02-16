/**
 * 3.6 堆栈与Copy和Move
 */
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug, Clone, Copy)]
struct Book {
    // 基础类型可以实现 Copy
    _page: i32,
    _rating: f64,
    // 需要自己实现
    // name: String // error: the trait `Copy` cannot be implemented for this type
}

fn main() {
    // box
    // 将结构体放在堆上
    let boxed_point = Box::new(Point{
        x: 10,
        y: 20
    });
    println!("x: {}, y: {}", boxed_point.x, boxed_point.y);
    // 将基础类型放在堆上
    let mut boxed_point = Box::new(32);
    println!("{}", *boxed_point);
    *boxed_point += 10;
    println!("{}", *boxed_point);

    // clone
    let x = vec![1, 2, 3, 4];
    let y = x.clone();
    println!("{:?}", y);
    println!("{:?}", x);

    let x = "ss".to_string();
    let y = x.clone();
    println!("{:?}", y);
    println!("{:?}", x);

    let b1 = Book {
        _page: 1,
        _rating: 0.1
    };
    let b2 = b1;
    println!("{:?}", b2);
    println!("{:?}", b1);
}
