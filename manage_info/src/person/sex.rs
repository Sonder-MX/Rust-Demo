#[derive(Debug, Clone, Copy)]
pub enum Sex {
    Male,
    Female,
}

impl Sex {
    pub fn slabel(&self) -> &'static str {
        if let Self::Male = self {
            return "男";
        }
        return "女";
    }

    pub fn choice_sex(s: &str) -> Self {
        if s == "男" {
            return Self::Male;
        }
        return Self::Female;
    }
}
