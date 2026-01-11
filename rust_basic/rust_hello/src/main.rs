//4.
const GLOBAL: u32 = 2000;

fn main() {
    println!("Hello, Rust!");

    //1.直接使用let定义变量，为不可变变量，不能修改
    let x = 5;
    println!("x的值为:{}", x);
    //x=6;运行会报错；

    //2.使用let mut定义变量，为可变变量
    let mut y = 5;
    println!("y的初始值为:{}", y);
    y = 7;
    println!("y修改后的值为:{}", y);

    //3.
    const MAX_POINT: u32 = 100000;
    println!("常量值为:{}", MAX_POINT);

    //4.
    println!("全局常量为:{}", GLOBAL);
}
