#[derive(Debug, Clone, Copy)]
pub struct BirthDate {
    pub year: u32,
    pub month: u8,
    pub day: u8,
}

impl BirthDate {
    pub fn new(year: u32, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    pub fn parse_date(date_str: &str) -> Self {
        let mut parts = date_str.split('-');
        let year = parts.next().unwrap().parse::<u32>().unwrap();
        let month = parts.next().unwrap().parse::<u8>().unwrap();
        let day = parts.next().unwrap().parse::<u8>().unwrap();
        Self { year, month, day }
    }

    pub fn birth_date_label(&self) -> String {
        format!("{}/{}/{}", self.year, self.month, self.day)
    }
}
