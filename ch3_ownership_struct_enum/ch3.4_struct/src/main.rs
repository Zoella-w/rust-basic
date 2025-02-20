/**
 * 3.4 结构体
 */
enum Flavor {
    Spicy,
    Sweet,
    Fruity,
}

struct Drink {
    flavor: Flavor,
    price: f64,
}

impl Drink {
    // 关联变量
    const MAX_PRICE: f64 = 10.0;
    // 方法
    fn buy(&self) {
        // 不可变引用/借用
        if self.price > Drink::MAX_PRICE {
            println!("I'm poor");
            return;
        }
        // if self.price > Self::MAX_PRICE {
        //     println!("I'm poor");
        //     return;
        // }
        // if self.price > 10.0 {
        //     println!("I'm poor");
        //     return;
        // }
        println!("buy it");
    }
    // 关联函数（入参没有 self）
    fn new(price: f64) -> Self {
        Drink {
            flavor: Flavor::Fruity,
            price,
            // price: price
        }
    }
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Spicy => println!("spicy"),
        Flavor::Sweet => println!("sweet"),
        Flavor::Fruity => println!("fruity"),
    }
    println!("{}", drink.price);
}

fn main() {
    let sweet = Drink {
        flavor: Flavor::Sweet,
        price: 6.0,
    };
    print_drink(sweet); // move

    // 方法
    let sweet = Drink {
        flavor: Flavor::Sweet,
        price: 6.0,
    };
    sweet.buy();
    let sweet = Drink {
        flavor: Flavor::Sweet,
        price: 16.0,
    };
    sweet.buy();

    // 关联函数
    let sweet = Drink::new(12.0);
    sweet.buy();
}
