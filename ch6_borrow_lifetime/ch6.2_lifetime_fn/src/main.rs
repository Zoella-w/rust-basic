/**
 * 6.2 生命周期和函数
 */
// 不需要声明周期
// fn no_need(s: &str) -> &str {}

// 标注 static（不建议）
fn no_need(s: &'static str, s1: &str) -> &'static str {
    s
}

// 标注为同一生命周期（性能有损耗）
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn longest_str<'a, 'b, 'out>(s1: &'a str, s2: &'b str) -> &'out str
where
    'a: 'out,
    'b: 'out,
{
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    println!("no need {}", no_need("hh", ""));

    let s1 = "hello world";
    let s2 = "hello";
    println!("longest {}", longest(s1, s2));

    let result: &str;
    {
        let s2 = String::from("world");
        result = longest_str(s1, &s2);
        println!("Longest string: {}", result);
    }
}
