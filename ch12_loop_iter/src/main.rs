/**
 * 4.2 循环与 break, continue 及迭代的区别
 */
fn main() {
    // loop {
    //     println!("Ctrl+C");
    //     std::thread::sleep(std::time::Duration::from_secs(1));
    // }
    let mut i = 0;
    while i < 10 {
        println!("{i}");
        i += 1;
    }
    println!("for1");
    let arr = [0, 1, 2, 3];
    for element in arr {
        println!("{element}");
    }
    println!("for2");
    for i in 8..10 {
        println!("{i}");
    }
    // break
    println!("break");
    for element in arr {
        if element == 2 {
            break;
        }
        println!("{element}");
    }
    // continue
    println!("continue");
    for element in arr {
        if element == 2 {
            continue;
        }
        println!("{element}");
    }
    'outer: loop {
        println!("outer");
        loop {
            println!("inner");
            // break;
            break 'outer;
        }
    }

    // 循环的写法
    let numbers = [1, 2, 3, 4, 5];
    let mut for_numbers = Vec::new();
    for &number in numbers.iter() {
        let item = number * number;
        for_numbers.push(item);
    }
    println!("for: {:?}", for_numbers);
    // 迭代的写法
    let numbers = [1, 2, 3, 4, 5].to_vec();
    let iter_number: Vec<i32> = numbers.iter().map(|&x| x * x).collect();
    println!("iter: {:?}", for_numbers);
}
