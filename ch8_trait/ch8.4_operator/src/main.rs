/**
 * 8.4 重载操作符
 */
// 导入 std::ops::Add 特质，该特质定义了加法运算符（+）行为
// 通过实现该特质，可以自定义加法操作
use std::ops::Add;

// 泛型在编译时处理，性能好
// 相当于实现 Debug 特质：impl Debug for Point {}
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// 通过实现 Add 特质，自定义如何对 Point<T> 执行加法运算，及定义了如何将两个 Point 对象相加
// T 是一个泛型类型参数，这意味着 Point 可以与任何实现了 Add 特质的类型一起使用，例如：整数、浮点数或其他自定义类型
impl<T> Add for Point<T>
// 约束，表示类型 T 必须实现 Add 特质，且其返回值类型（即 Output）也必须是 T，保证 x 和 y 字段可以被相加并返回同样的类型
where
    T: Add<Output = T>,
{
    type Output = Self; // 关联类型 Output 为 Self（即 Point）

    // self 为调用该方法的实例；rhs 为右侧的操作数
    // 如 i1+i2 中，i1 是调用 Add 方法的实例，i2 是右侧操作数
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let i1 = Point { x: 1, y: 2 };
    let i2 = Point { x: 2, y: 3 };
    let sum = i1 + i2;
    println!("{:?}", sum);
}
