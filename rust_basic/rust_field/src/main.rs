//rust作用域

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
}
