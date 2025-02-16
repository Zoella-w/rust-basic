/**
 * 4.1 if 与 match
 */
fn main() {
    // 1. if
    let age = 50;
    if age < 50 {
        println!("You are young");
    }
    else {
        println!("You are old");
    }

    // if 的表达能力较弱
    let scores = 70;
    if scores > 90 {
        println!("Good!");
    }
    else if scores > 60 {
        println!("You are ok!");
    }
    else {
        println!("Bad!!!");
    }

    let msg = if age > 50 {"old"} else {"young"};
    println!("You are {}", msg);

    // 2. match
    let num = 90;
    match num {
        80 => println!("80"),
        90 => println!("90"),
        _ => println!("Some else")
    }
    match num {
        25..=50 => println!("25 ... 50"),
        50..=100 => println!("51 ... 100"),
        _ => println!("Some else")
    }
    match num {
        25|50|75 => println!("25 or 50 or 75"),
        100 | 200 => println!("100 or 200"),
        _ => println!("Some else")
    }
    match num {
        x if x < 60 => println!("bad"),
        x if x == 60 => println!("luck"),
        _ => println!("good")
    }
    let num = 60;
    let res = match num {
        x if x < 60 => "bad".to_owned(),
        x if x == 60 => "luck".to_owned(),
        _ => "good".to_owned()
    };
    println!("{res}");
}
