/**
 * 5.2 unwrap 与 ?
 */
use std::num::ParseIntError;

fn find_first_even(numbers: Vec<i32>) -> Option<i32> {
    let first_even = numbers.iter().find(|&num| num % 2 == 0)?;
    println!("Option");
    Some(*first_even)
}

// 传递错误
fn parse_numbers(input: &str) -> Result<i32, ParseIntError> {
    // let val = input.parse::<i32>();
    let val: i32 = input.parse()?;
    Ok(val)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result_ok: Result<i32, &str> = Ok(32);
    let value = result_ok.unwrap();
    println!("{}", value);
    // let result_ok: Result<i32, &str> = Err("ff");
    // let value = result_ok.unwrap(); // panic
    // println!("{}", value);
    let result_ok: Result<i32, &str> = Ok(32);
    let value = result_ok?;
    println!("{}", value);

    let numbers = vec![1, 2, 3, 4, 5]; // Option first even 2
    // let numbers = vec![1, 3, 5]; // no such number
    match find_first_even(numbers) {
        Some(number) => println!("first even {}", number),
        None => println!("no such number")
    }

    match parse_numbers("d") {
        Ok(i) => println!("parsed {}", i),
        Err(err) => println!("failed to parse: {}", err)
    }

    Ok(())
}
