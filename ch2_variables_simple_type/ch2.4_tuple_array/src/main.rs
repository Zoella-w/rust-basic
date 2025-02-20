/**
 * 2.4 元组和数组
 */
fn main() {
    // tuple
    let tup = (0, "hi", 3.4);
    println!("tup elements {} {} {}", tup.0, tup.1, tup.2);
    // tup.len(); // error

    let mut tup2 = (0, "hi", 3.4);
    println!("tup2 elements {} {} {}", tup2.0, tup2.1, tup2.2);
    // tup2.0 = "f"; // error
    tup2.1 = "f";
    println!("tup2 elements {} {} {}", tup2.0, tup2.1, tup2.2);

    // ()
    let tup3 = ();
    println!("tup3 {:?}", tup3);

    // array
    let mut arr = [11, 12, 13];
    arr[0] = 999;
    println!("arr len {} first element is {}", arr.len(), arr[0]);

    for elem in arr {
        println!("{}", elem);
    }

    // copy
    // move ownership
    let string_item = String::from("aa");
    let string_item_tt = string_item;  // String类型把ownership进行move操作
    // println!("string_item: {}", string_item) // error: value borrowed here after move
}
