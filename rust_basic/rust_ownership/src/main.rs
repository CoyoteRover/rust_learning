fn main() {
    //String 类型存储在堆上
    let s1 = String::from("hello");

    //所有权移动(Move)
    //s1的值移动到s2,s1失效
    let s2 = s1;

    //println!("{},word",s1);报错！s1已失效
    println!("{},rust", s2);

    //克隆(Clone)
    let s3 = s2.clone();
    println!("s2:{};s3:{}", s2, s3);

    //规则1：每个值都有一个所有者
    let s = String::from("hello");
    //s 为"hello"的所有者

    //规则2：每个值在任意时刻有且仅有一个所有者
    let s_2 = s;
    //s的所有权转移到s_2
    println!("{}", s_2);

    //规则3：所有者离开作用域，值被丢弃
    {
        let s_3 = String::from("rust");
        println!("{}", s_3); //s3在此处有效
    } //s_3离开作用域，值被自动释放

    //函数调用也会转移所有权
    let s4 = String::from("test");
    takes_ownership(s4);
    //s4的所有权转移到函数
    //println!("{}",s4);错误！s4已无效

    //返回值也会转移所有权
    let sth = give_ownership();
    println!("{}", sth);

    //整数实现了copy，会复制
    let x = 5;
    let y = x;
    println!("{},{}", x, y);
    //两者都有效

    //克隆

    let lan_1 = String::from("hello");
    let lan_2 = lan_1.clone(); //使用clone进行深拷贝，s1、s2均有效
    println!("1:{};2:{}", lan_1, lan_2);

    //对于实现了Clone trait的类型
    let v1 = vec![1, 2, 3];
    let v2 = v1.clone();
    println!("v1:{:?},v2:{:?}", v1, v2);

    //结构体也可以克隆
    #[derive(Clone)]//自动实现Clone
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 0, y: 0 };
    let p2 = p1.clone();

    println!("p1:({},{}),p2:({},{})", p1.x, p1.y, p2.x, p2.y);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} //some_string离开作用域，值被丢弃

fn give_ownership() -> String {
    let some_string = String::from("ownership");
    some_string //将所有权交给调用者
}
