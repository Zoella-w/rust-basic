/**
 * 2.2 const与static
 */
static MY_STATIC: i32 = 42;
static mut MY_MUT_STATIC: i32 = 42;

fn main() {
    // const
    const SECOND_HOUR: usize = 3_600; // 3600，可读性更强
    const SECOND_DAY: usize = 24 * SECOND_HOUR; // compile-time constant

    {
        const SE: usize = 1_000;
        println!("SE: {}", SE);
    }

    // println!("SE: {}", SE); // error
    println!("SECOND_DAY: {}", SECOND_DAY);
    
    // MY_STATIC = 2; // error
    unsafe {
        MY_MUT_STATIC = 32;
        println!("MY_MUT_STATIC: {MY_MUT_STATIC}");
    }
    // println!("MY_MUT_STATIC: {MY_MUT_STATIC}"); // error
}
