fn main() {
    //1.loop 无限循环
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2; //break可以用来返回值
        }
    };
    println!("结果是{}", result);

    //2.while 条件循环
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("启动!");

    //for 遍历集合
    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("值为:{}", element);
    }

    //for 遍历范围
    for number in 1..4 {
        println!("{}!", number);
    }//1,2,3

    for number in 1..=4 {
        println!("{}!", number);
    }//1,2,3,4
}
