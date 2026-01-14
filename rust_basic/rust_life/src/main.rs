
//结构体中的生命周期
struct ImportantExcerpt<'a> {
    part: &'a str,
}


fn main() {
    let string1 = String::from("This is a long string");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("最长的字符串:{}", result);
    }

    let novel = String::from("Call me Ishmael.Some years ago...");
    let first_sentece = novel.split('.').next().expect("找不到'.'");
    let i = ImportantExcerpt {
        part: first_sentece,
    };

    println!("{}", i.part);
}

//这个函数需要生命周期标注
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
//生命周期标注'a表示
//- 参数x和y的引用至少和'a一样长
//- 返回的引用也至少和'a一样长

//'a是告知编译器变量的生命周期的
//编译器通过'a来关联变量的生命周期关系进行计算