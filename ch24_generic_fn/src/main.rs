/**
 * 7.2 泛型函数
 */
fn swap<T>(a: T, b: T) -> (T, T) {
    (b, a)
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    // 关联函数
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    // 方法
    fn get_coordinates(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }
}

fn main() {
    let result = swap::<f64>(0.1, 1.0);
    let result: (f64, f64) = swap::<f64>(0.1, 1.0);
    println!("{:?}", result);
    let str2 = swap("hh", "tt");
    println!("str2.0 {} str2.1 {}", str2.0, str2.1);

    let i32_point = Point::new(2, 3);
    let i64_point = Point::new(2.0, 3.0);
    let (x1, y1) = i32_point.get_coordinates();
    let (x2, y2) = i64_point.get_coordinates();
    println!("i32 point: x={} y={}", x1, y1);
    println!("i64 point: x={} y={}", x2, y2);
    let string_point = Point::new("xxx".to_owned(), "yyy".to_owned());
    println!("string point x={} y={}", string_point.x, string_point.y);
}
