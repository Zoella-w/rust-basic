/**
 * 8.3 Trait 与 泛型
 */
trait Overview {
    fn overview(&self) -> String {
        String::from("Course")
    }
}

trait Another {
    fn heaven(&self) {
        println!("Welcome to heaven");
    }
}

struct Course {
    headline: String,
    author: String,
}

impl Overview for Course {}
impl Another for Course {}

struct AnotherCourse {
    headline: String,
    author: String,
}

impl Overview for AnotherCourse {}

// impl写法
fn call_overview(item: &impl Overview) {
    println!("call_overview Overview {}", item.overview());
}
// 泛型写法
fn call_overview_generic<T: Overview>(item: &T) {
    println!("call_overview_generic Overview {}", item.overview());
}

// impl写法
fn call_overviewT(item: &impl Overview, item1: &impl Overview) {
    println!("call_overviewT Overview {}", item.overview());
    println!("call_overviewT Overview {}", item1.overview());
}
// 泛型写法
fn call_overviewTT<T: Overview>(item: &T, item1: &T) {
    println!("call_overviewTT Overview {}", item.overview());
    println!("call_overviewTT Overview {}", item1.overview());
}

// 多绑定
// 1. impl写法（不推荐）
fn call_mul_bind(item: &(impl Overview + Another)) {
    println!("all_mul_bind Overview {}", item.overview());
    item.heaven()
}
// 2.泛型写法（推荐）
// fn call_mul_bind_generic<T: Overview + Another>(item: &T) {
//     println!("call_mul_bind_generic Overview {}", item.overview());
//     item.heaven()
// }
fn call_mul_bind_generic<T>(item: &T)
where
    T: Overview + Another,
{
    println!("call_mul_bind_generic Overview {}", item.overview());
    item.heaven()
}

fn main() {
    let c0 = Course {
        headline: "ff".to_owned(),
        author: "yy".to_owned(),
    };
    let c1 = Course {
        headline: "ff".to_owned(),
        author: "yy".to_owned(),
    };
    let c2 = AnotherCourse {
        headline: "ff".to_owned(),
        author: "yz".to_owned(),
    };
    call_overview(&c1);
    call_overview_generic(&c1);

    call_overviewT(&c1, &c2);
    // call_overviewTT(&c1, &c2); // error
    call_overviewTT(&c1, &c0);

    // 多绑定
    // call_mul_bind(&c2); // error
    call_mul_bind(&c1);
    call_mul_bind_generic(&c1);
}
