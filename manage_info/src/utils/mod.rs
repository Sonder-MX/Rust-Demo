use std::fmt::Debug;
use std::io::{self, Write};
use std::str::FromStr;

// FromStr 是一个特征，它提供了一个类型的方法，用来将字符串转换为指定类型
// 如果转换失败，它会返回一个错误
// 如果转换成功，它会返回一个指向转换后的结果的指针
pub fn input<T: std::str::FromStr>(tips: &str) -> Option<T>
where
    <T as FromStr>::Err: Debug,
{
    print!("{}", tips);
    io::stdout().flush().unwrap(); //手动刷新
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).unwrap();
    match inp.trim().parse::<T>() {
        Ok(val) => Some(val),
        Err(_) => None,
    }
}
