/**
 * 4.6 高阶函数
 */
fn func_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

fn mul(x: i32) -> i32 {
    x * x
}

fn add(x: i32) -> i32 {
    x + 10
}

fn main() {
    let result = func_twice(mul, 3);
    println!("{result}");
    let res = func_twice(add, 10);
    println!("{res}");

    // 数学计算
    // iter()的三种方法：map, filter, fold
    // 1. map
    let numbers = vec![1, 2, 3, 4, 5, 6, 7];
    let res: Vec<_> = numbers.iter().map(|&x| x + x).collect();
    println!("{:?}", res); // [[2, 4, 6, 8, 10, 12, 14]

    // 2. filter
    let numbers = vec![1, 2, 3, 4, 5, 6, 7];
    // ref ref_mut move
    let evens = numbers
        .into_iter()
        .filter(|&x| x % 2 == 0)
        .collect::<Vec<_>>();
    println!("{:?}", evens); // [2, 4, 6]

    // 3. fold
    let numbers = vec![1, 2, 3, 4, 5, 6, 7];
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("{:?}", sum); // 28
}
