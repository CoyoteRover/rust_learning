fn main() {
    let s1 = String::from("hello");

    //创建引用（借用）
    let len = calculate_length(&s1); //&s1创建引用
    println!("'{}'的长度为{}", s1, len); //二者均有效

    //引用默认不可改变
    let s2 = String::from("world");
    //change(&s2);错误！无法通过不可变引用来改变值
    println!("{}", s2);

    //可变引用
    let mut s2 = String::from("hello");

    change(&mut s2);

    println!("{}", s2);

    //可变引用限制
    let mut s = String::from("test");

    let r1 = &mut s;
    //let r2 = &mut s; 报错！不允许有多个可变引用！
    println!("{}", r1);

    //允许存在多个不可变引用
    let r1 = &s;
    let r2 = &s;
    println!("{}{}", r1, r2);

    //悬垂引用
    //这个函数会报错，因为返回了垂悬引用
    //fn dangle() -> &String {
    //    let s = String::from("test");
    //    &s  //错误！s离开作用域后被丢弃
    //}

    //正确做法：返回所有权
    let s = no_dangle();
    println!("{}", s);

    //同理，引用的生命周期不能长于数据
    //let r;
    //{
    //    let x = 5;
    //    r = &x; //错误！x的生命周期不够长
    //}
    //println!("{}", r); //r指向的x的数据已不存在
}

fn calculate_length(s: &String) -> usize {
    s.len()
} //s离开作用域，但由于只是引用，其值不会被丢弃

//该函数会报错
/*fn change(some_string:&String)
{
    some_string.push_str(",world");
}
*/

fn change(some_string: &mut String) {
    some_string.push_str(",world");
}

fn no_dangle() -> String {
    let s = String::from("test");
    s //返回所有权而不是引用
}
