fn main() {
    //if let-只匹配一种情况
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("使用你喜欢的颜色{}", color);
    } else if is_tuesday {
        println!("星期二是绿色日");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("使用紫色");
        } else {
            println!("使用蓝色");
        }
    } else {
        println!("使用蓝色");
    }

    //while let-循环直到模式不匹配
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    //for 循环中的模式匹配
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{}在索引{}", value, index);
    }
}
