//rust所有权基础与作用域

fn main() {
    {
        let s = "hello"; //s从此处开始生效

        println!("{}", s); //使用s
    } //作用域结束，s不再生效

    //字符串字面量&str存储在栈上
    let s1 = "hello";
    let s2 = s1;
    //s1,s2均有效，这一步操作实现了copy
    println!("s1:{},s2:{}", s1, s2);

    //String 类型存储在堆上
    let s3 = String::from("hello");
    let s4 = s3; //s3的所有权转移到s4,s3失效
    //println!("{}", s3);
    //报错！s3已无效！报错：borrow of moved value: 's3' value borrowed here after move
    println!("{}", s4); //正确

    //外层作用域
    let outer = "outer";
    println!("外层:{}", outer);

    {
        //内层作用域
        let inner = "inner";
        println!("内层:{}", inner);
        println!("外层（在内层调用）:{}", outer); //内层可以访问外层变量
    } //inner在这里失效

    //println!("{}",inner);//在这里会报错，已离开inner作用域
    println!("外层（在外层中）:{}", outer);

    //变量遮蔽
    let x = 5;
    let x = x + 1;//遮蔽之前的x
    {
        let x = x * 2; //遮蔽外层的x
        println!("内层x:{}", x);
    }
    println!("外层x:{}", x);
}
