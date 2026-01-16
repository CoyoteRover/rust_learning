///结构体
/// rust提供三种结构体：C风格、元组结构体、单元结构体
struct User {
    user_name: String,
    email: String,
    active: bool,
}

//枚举
enum Coin {
    Penny,
    Nickle,
    Dime,
}

fn main() {
    //
    let user1 = User {
        email: String::from("someone@example.com"),
        user_name: String::from("someusername123"),
        active: true,
    };
    println!(
        "用户：{}；邮箱：{}；登陆状态：{}",
        user1.user_name, user1.email, user1.active
    );

    let coin = Coin::Nickle;
    println!("硬币价值:{}美分", value_in_cent(coin));
}

//模式匹配
fn value_in_cent(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
    }
}

//枚举通过列举各种可能的成员类型来定义一整个类型
//rust中，枚举的成员可以携带数据
//利用模式匹配来对应成员类型，拆解或返回数据

//结构体与枚举如何选择：
//结构体中的成员对结构体这一整个类型来说是同时存在的，是and且关系
//枚举中的成员对枚举这一整个类型来说只能同时存在一个，是排斥或关系