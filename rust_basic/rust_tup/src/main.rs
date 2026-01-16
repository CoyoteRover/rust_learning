fn main() {
    //创建元组
    let tup: (i32, f64, u8) = (500, 3.14, 1);

    //通过索引访问
    let five_hundred = tup.0;
    let pai = tup.1;
    let one = tup.2;
    println!("{}{}{}", five_hundred, pai, one);

    //解构元组
    let (x, y, z) = tup;
    println!("x:{};y:{};z:{}", x, y, z);
    let (sum, product) = cauculate(5, 3);
    println!("和:{};积:{}", sum, product);
}

//函数返回多个值
fn cauculate(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)
}
