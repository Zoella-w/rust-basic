/**
 * 8.6 常见的 trait
 */
// Debug Clone Copy PartialEq

// 也需要实现 Debug 特质
#[derive(Debug, Clone, Copy)]
enum Race {
    White,
    Yellow,
    Black
}

impl PartialEq for Race {
    fn eq(&self, other: &Self) -> bool {
        // 匹配模式
        match (self, other) {
            (Self::White, Self::White) => true,
            (Self::Yellow, Self::Yellow) => true,
            (Self::Black, Self::Black) => true,
            _ => false
        }
    }
}

// #[derive(Debug, Clone, Copy)]
#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String, // 没有 Copy
    race: Race
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name && self.race == other.race
    }
}

fn main() {
    let user = User {
        id: 3,
        name: "Zoella".to_owned(), // 没有 Copy
        race: Race::Yellow
    };
    println!("{:#?}", user); // # 格式更好

    let user2 = user.clone(); // Clone 在实例上实现
    println!("{:#?}", user2);

    // let user3 = user;
    // println!("{:#?}", user3);
    // println!("{:#?}", user);

    println!("{}", user == user2);
}
