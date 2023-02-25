use rand::Rng;
use std::io::{self, Write};

fn main() {
    println!("猜数字游戏(1~100)，只能猜10次！！！");
    let rand_num = rand::thread_rng().gen_range(1..=100); // 生成1~100之间的随机数
    let mut guess_counter = 1; // mut 可变 变量

    loop {
        if guess_counter >= 11 {
            println!("次数用尽了，再试一次吧！");
            break;
        }
        print!("输入一个数：");
        let _ = io::stdout().flush(); // 手动刷新显示提示语

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let inp_num = match input.trim().parse::<i32>() {
            Ok(val) => val,
            Err(_) => {
                println!("输入有误，输入数字重新试一下！");
                continue;
            }
        };

        if inp_num == rand_num {
            println!("恭喜你猜对了, 你一共用了{guess_counter}次");
            break;
        } else if inp_num > rand_num {
            println!("猜大了");
        } else {
            println!("猜小了");
        }
        guess_counter += 1;
    }
}
