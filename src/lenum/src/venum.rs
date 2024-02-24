struct Ipv4Addr{}
struct Ipv6Addr {}
enum IpAddr{
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self){
    }
}
enum Option<T> {
    Some(T),
    None,
}

pub enum Coin{
    Penny,
    Nickel,
    Dime ,
    Quarter,
}

// 一个枚举和一个以枚举成员作为模式的match表达式
pub fn value_in_centes(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel =>3,
        Coin::Dime =>2,
        _ => 22,
    }
}


