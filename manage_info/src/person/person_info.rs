use crate::utils::input;

use super::birth_date::BirthDate;
use super::sex::Sex;

const NOW_YEAR: u32 = 2023; // 可以用时间库动态获取

#[derive(Debug)]
pub struct PersonInfo {
    pub pid: usize,
    pub pname: &'static str,
    pub psex: Sex,
    pub birth_date: BirthDate,
    pub age: u8,
    pub other: Option<String>,
}

impl PersonInfo {
    pub fn new(
        pid: usize,
        pname: &'static str,
        psex: Sex,
        birth_date: BirthDate,
        other: Option<String>,
    ) -> Self {
        Self {
            pid,
            pname,
            psex,
            birth_date,
            age: (NOW_YEAR - birth_date.year) as u8,
            other,
        }
    }

    // 显示 other 字段
    pub fn other_label(&self) -> String {
        match &self.other {
            Some(val) => val.to_owned(),
            None => "".to_owned(),
        }
    }

    // 更新 person info
    pub fn update(&mut self) {
        self.pname = match input::<String>("输入姓名：") {
            // 卫语句
            Some(val) if val != "" => Box::leak(val.into_boxed_str()),
            _ => self.pname,
        };
        self.psex = match input::<String>("输入性别：") {
            Some(val) if val != "" => Sex::choice_sex(&val),
            _ => self.psex,
        };
        self.birth_date = match input::<String>("输入出生年月日(eg:2023-1-1)：") {
            Some(val) if val != "" => {
                println!("{val}");
                BirthDate::parse_date(&val)
            }
            _ => self.birth_date,
        };
        self.age = (NOW_YEAR - self.birth_date.year) as u8;
        self.other = input::<String>("其他：");
    }
}
