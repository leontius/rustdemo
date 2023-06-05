fn main() {
    // 生命周期省略的三个规则
    // 1.每个引用的参数都有自己的生命周期
    // 2.如果只有1个输入生命周期参数，那么该生命周期被赋给所有的输出生命周期参数。
    // 3.如果有多个输入生命周期参数，但其中一个是&self或&mut self（是方法），那么self的生命周期会被赋给所有的输出生命周期。
    let string1 = String::from("hello");
    let string2 = "xyz";
    let result = langest(string1.as_str(), string2);
    println!("The langest string is: {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");

    let first_sentence = novel.split('.').next().expect("Cloud not fina a '.");

    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn langest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i128 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
