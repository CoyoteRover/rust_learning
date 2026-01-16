fn main() {
    let number = 5;
    //基本匹配
    match number {
        1 => println!("一"),
        2 => println!("二"),
        3 => println!("三"),
        4 => println!("四"),
        _ => println!("其他"),
    }

    //匹配多个值
    match number {
        1 | 2 => println!("一或二"),
        3..=5 => println!("三到五"),
        _ => println!("其他"),
    }

    let point = (0, 5);
    //匹配并绑定值
    match point {
        (0, y) => println!("在y轴上,y={}", y),
        (x, 0) => println!("在x轴上,x={}", x),
        (x, y) => println!("在({},{})", x, y),
    }

    //match作为表达式
    let result = match number {
        n if n < 5 => "小于5",
        n if n == 5 => "等于5",
        _ => "大于5",
    };
    println!("{}!", result);
}
