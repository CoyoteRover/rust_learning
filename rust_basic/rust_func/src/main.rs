fn main() {
    let plus = add(5, 3);
    println!("5+3={}", plus);

    let product = multi(7, 8);
    println!("7*8={}", product);

    greet("Rust");

    let num = get_number();
    println!("获取的数字为:{}", num);
}

//1.带参数和返回值的函数
fn add(x: i32, y: i32) -> i32 {
    x + y //注意！！此处为返回值，不使用分号！
}

fn multi(x: i32, y: i32) -> i32 {
    return x * y; //使用return也可以返回
}

//2.不带返回值的函数
fn greet(name: &str) {
    println!("Hello,{}", name);
}

//3.只带返回值的函数
fn get_number() -> i32 {
    42
}
