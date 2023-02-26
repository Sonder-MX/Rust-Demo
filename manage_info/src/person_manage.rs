use crate::person::birth_date::BirthDate;
use crate::person::person_info::PersonInfo;
use crate::person::sex::Sex;

#[derive(Debug)]
pub struct PersonManage {
    pvec: Vec<PersonInfo>,
}

impl PersonManage {
    pub fn new() -> Self {
        Self {
            pvec: vec![
                PersonInfo::new(1001, "张三", Sex::Male, BirthDate::new(2000, 10, 12), None),
                PersonInfo::new(
                    1002,
                    "李四",
                    Sex::Male,
                    BirthDate::new(2001, 5, 10),
                    Some("test".to_string()),
                ),
                PersonInfo::new(
                    1003,
                    "王五",
                    Sex::Female,
                    BirthDate::new(2002, 12, 12),
                    None,
                ),
            ],
        }
    }

    pub fn print_info(&self) {
        println!("*****************************************************");
        println!(
            "{}",
            format!(
                "{:<8}{:<8}{:<4}{:<10}{:<4}{}",
                "人员ID", "姓名", "性别", "出生日期", "年龄", "其他"
            )
        );

        for p in &self.pvec {
            println!(
                "{:<10}{:<8}{:<4}{:<16}{:<4}{}",
                p.pid,
                p.pname,
                p.psex.slabel(),
                p.birth_date.birth_date_label(),
                p.age,
                p.other_label()
            );
        }
        println!("*****************************************************\n");
    }

    pub fn add_person(&mut self, person_vec: &Vec<&'static str>) {
        let pid = match self.pvec.last() {
            Some(p) => p.pid + 1,
            None => 1001,
        };
        let pname = person_vec[0];
        let psex = Sex::choice_sex(person_vec[1]);
        let birth_date = BirthDate::parse_date(person_vec[2]);
        let other = match person_vec.get(3) {
            Some(other) => Some(other.to_string()),
            None => None,
        };

        self.pvec
            .push(PersonInfo::new(pid, pname, psex, birth_date, other));
        println!("添加成功");
    }

    pub fn delete_person(&mut self, pid: usize) {
        if self.exists_pid(pid) {
            // 返回 false 的所有元素
            self.pvec.retain(|p| p.pid != pid);
            println!("ID为{pid}的人员信息删除成功！");
        } else {
            println!("不存在该人员信息！！！");
        }
    }

    pub fn update_person(&mut self, pid: usize) {
        if self.exists_pid(pid) {
            println!("不需要修改的字段留空 回车即可");
            // 选择 p.pid == pid 的下标
            if let Some(ind) = self.pvec.iter().position(|p| p.pid == pid) {
                self.pvec[ind].update();
                println!("更新成功！");
            }
        } else {
            println!("不存在该人员信息！！！");
        }
    }
    // private method
    fn exists_pid(&self, pid: usize) -> bool {
        self.pvec.iter().any(|p| p.pid == pid)
    }
}
