//4.
const GLOBAL: u32 = 2000;

fn main() {
    println!("Hello, Rust!");

    //变量与可变性
    //1.直接使用let定义变量，为不可变变量，不能修改
    let x = 5;
    println!("x的值为:{}", x);
    //x=6;运行会报错；

    //2.使用let mut定义变量，为可变变量
    let mut y = 5;
    println!("y的初始值为:{}", y);
    y = 7;
    println!("y修改后的值为:{}", y);

    //3.常量
    const MAX_POINT: u32 = 100000;
    println!("常量值为:{}", MAX_POINT);

    //4.全局常量
    println!("全局常量为:{}", GLOBAL);

    //数据类型
    //1.整数类型
    let int32: i32 = 42;
    let uint64: u64 = 100;

    println!("32位整型:{};64位无符号整形:{}", int32, uint64);

    //2.浮点类型
    let pi: f64 = 3.14159;

    println!("圆周率64位浮点:{}", pi);

    //3.布尔类型
    let is_rust_interesting: bool = true;

    if is_rust_interesting {
        println!("Rust有点意思");
    }

    //4.字符类型
    let letter: char = 'e';

    println!("字符:{}", letter);

    //5.元组

    let tup = (500, 6.4, true);

    let (a, b, c) = tup; //解构

    println!("元组中整形a:{},元组中浮点:{},元组中布尔:{}", a, b, c);

    //6.数组
    let arr = [1, 2, 3, 4, 5];

    println!("数组第一个元素:{}", arr[0]);
}
