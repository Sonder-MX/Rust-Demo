mod person;
mod person_manage;
mod utils;

use person_manage::PersonManage;
use utils::input;

fn main() {
    let mut management = PersonManage::new();
    loop {
        println!("人员信息管理");
        println!("\t1.添加人员\n\t2.删除人员\n\t3.修改人员\n\t4.查询所有人员\n\t5.退出\n");
        let choice = match input::<u32>("请选择操作：") {
            Some(val) => val,
            None => {
                println!("\n输入有误，重新输入！\n");
                continue;
            }
        };
        match choice {
            1 => {
                let pstr = input::<String>(
                    "输入人员信息，例如:\n\t张三 男 2023-1-1 其他(可选)\n空行退出\n输入信息：",
                );
                if let Some(pstr) = pstr {
                    // 将String 转换为 &'static str
                    let s = Box::leak(pstr.into_boxed_str());
                    let pvec: Vec<&'static str> = s.split(' ').collect::<Vec<&str>>();
                    if pvec.len() >= 3 && pvec.len() <= 4 {
                        management.add_person(&pvec);
                    } else {
                        println!("信息输入有误");
                    }
                }
            }
            2 => {
                if let Some(val) = input::<usize>("输入要删除的人员ID：") {
                    management.delete_person(val);
                } else {
                    println!("输入正确的ID")
                }
            }
            3 => {
                if let Some(val) = input::<usize>("输入要修改的人员ID：") {
                    management.update_person(val);
                } else {
                    println!("输入正确的ID")
                }
            }
            4 => management.print_info(),
            5 => break,
            _ => {
                println!("\n没有该选项，重新输入！\n");
                continue;
            }
        }
    }
    println!("程序已退出");
}
