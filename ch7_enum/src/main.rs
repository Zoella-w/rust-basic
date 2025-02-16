/**
 * 3.3 枚举与匹配模式
 */
enum Color {
    Red,
    Yellow,
    Blue,
    Black
}

fn print_color(my_color: Color) {
    match my_color {
        Color::Red => println!("Red"),
        Color::Yellow => println!("Yellow"),
        Color::Blue => println!("Blue"),
        _ => (),
    }
}

enum BuildingLocation {
    Number(i32),
    Name(String), // 不用 &str
    Unknown,
}

impl BuildingLocation {
    fn print_location(&self) {
        match self {
            // BuildingLocation::Number(44)
            BuildingLocation::Number(c) => println!("building number: {}", c),
            // BuildingLocation::Number("ok".to_string())
            BuildingLocation::Name(s) => println!("building name: {}", s),
            BuildingLocation::Unknown => println!("unknown"),
        }
    }
}

fn main() {
    let a = Color::Red;
    print_color(a); // move

    let house1 = BuildingLocation::Name("fdfd".to_string());
    house1.print_location(); // 入参加了 &self，可以调用实例
    let house2 = BuildingLocation::Number(123);
    house2.print_location();
    let house3 = BuildingLocation::Unknown;
    house3.print_location();
    // BuildingLocation::print_location(); // 如果入参没有 &self
}
