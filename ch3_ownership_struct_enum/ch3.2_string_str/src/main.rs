/**
 * 3.2 String 与 &str
 */
// String
struct Person {
    _name: String,
    _color: String,
    _age: i32
}
// &str
struct PersonStr<'a> {
    _name: &'a str, // lifetime specifier
    _color: String,
    _age: i32
}

// &String &str 都可以传
// &String 可以转化为 &str
fn _print(data: &str) {
    println!("{}", data);
}
// 只能传 &String
// &str 无法转化为 &String，因为 &str 不包含元数据（如容量）
fn _print_string_borrow(data: &String) {
    println!("{}", data);
}
fn main() {
    // String  &str
    // 将字面量转化为String
    // 1. String::from
    // 2. to_string()
    // 3. to_owned()
    let name = String::from("Value C++");
    let course = "Rust".to_owned();
    let new_name = name.replace("C++", "CPP");
    println!("{name}, {course}, {new_name}");

    let rust = "\x52\x75\x73\x74"; // rust
    print!("{rust}");

    // struct
    // 1. String
    let color = "green".to_string();
    let name = "John".to_string();
    let _people = Person{
        _name: name,
        _color: color,
        _age: 89,
    };
    // 2. &str
    let color2 = "green".to_string();
    let name_str = "John";
    let _people_str = PersonStr{
        _name: name_str,
        _color: color2,
        _age: 89,
    };

    // function
    // 1. &str
    let value = "value".to_owned();
    _print(&value);
    _print("data");
    // _print_string_borrow("data"); // error: expected reference `&String`
    _print_string_borrow(&value);

}
