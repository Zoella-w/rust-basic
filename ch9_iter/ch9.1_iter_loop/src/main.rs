/**
 * 9.1 迭代与循环
 */
// loop
fn sum_with_loop(arr: &[i32]) -> i32 {
    let mut sum = 0;
    // 实际得到的 item 是 &i32 的引用（即指向数组中元素的引用），因为 arr 是一个引用
    for &item in arr {
        sum += item;
    }
    sum
}

// iter
fn sum_with_iter(arr: &[i32]) -> i32 {
    arr.iter().sum()
}

fn main() {
    const ARRAY_SIZE: usize = 1_000;
    // Vec<_> 表示根据上下文推断出具体类型
    // 1..=ARRAY_SIZE as i32 是一个范围表达式，生成一个从 1 到 ARRAY_SIZE（包括 ARRAY_SIZE）的整数序列
    // collect() 是一个迭代器消耗方法，将迭代器中的所有元素收集到一个集合中
    let array:Vec<_> = (1..=ARRAY_SIZE as i32).collect();
    let sum1 = sum_with_loop(&array);
    println!("{}", sum1);
    let sum2 = sum_with_iter(&array);
    println!("{}", sum2);
}
