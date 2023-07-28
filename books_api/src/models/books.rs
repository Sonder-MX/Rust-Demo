use chrono::prelude::Local;
use poem_openapi::Object;

#[derive(Debug, Clone, Object)]
pub struct Books {
    id: Option<i32>,
    name: Option<String>,
    author: Option<String>,
    price: Option<f64>,
    create_time: Option<String>,
    update_time: Option<String>,
}

impl Books {
    pub fn new(
        id: Option<i32>,
        name: Option<String>,
        author: Option<String>,
        price: Option<f64>,
    ) -> Self {
        Books {
            id,
            name,
            author,
            price,
            create_time: Some(Local::now().to_string()),
            update_time: Some(Local::now().to_string()),
        }
    }

    pub fn get_id(&self) -> Option<i32> {
        self.id
    }

    pub fn get_name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }

    pub fn set_create_time(&mut self) {
        self.create_time = Some(Local::now().to_string());
    }

    pub fn set_update_time(&mut self) {
        self.update_time = Some(Local::now().to_string());
    }
}
