use crate::models::books::Books;
use crate::resp::{RespData, Response};

use poem_openapi::{param::Path, param::Query, payload::Json, OpenApi};
use tokio::sync::Mutex;

pub struct BooksApi {
    books: Mutex<Vec<Books>>,
}

impl Default for BooksApi {
    fn default() -> Self {
        let book_list = [
            "Rust编程之道",
            "Rust权威指南",
            "Rust程序设计语言",
            "Rust编程入门",
            "Rust实践指南",
        ];
        let mut books = vec![];
        book_list.iter().enumerate().for_each(|(i, book)| {
            books.push(Books::new(
                Some(i as i32 + 1),
                Some(book.to_string()),
                Some("Rust".to_string()),
                Some(29.99),
            ));
        });
        BooksApi {
            books: Mutex::new(books),
        }
    }
}

#[OpenApi]
impl BooksApi {
    #[oai(path = "/", method = "get")]
    async fn books_list(&self) -> Json<Vec<Books>> {
        let books = self.books.lock().await;
        Json(books.clone())
    }

    #[oai(path = "/:id", method = "get")]
    async fn get_book(&self, #[oai(name = "id")] id: Path<i32>) -> Response<Books, RespData> {
        let books = self.books.lock().await;
        let book = books.iter().find(|book| book.get_id().unwrap() == id.0);
        match book {
            Some(book) => Response::ok(book.clone()),
            None => Response::not_found(RespData::new(-1, Some("book not found".to_string()))),
        }
    }

    #[oai(path = "/search", method = "get")]
    async fn search_book(
        &self,
        #[oai(name = "s")] s: Query<String>,
    ) -> Response<Vec<Books>, RespData> {
        println!("search content: {}", s.0);
        let books = self.books.lock().await;
        let book_list = books
            .iter()
            .filter(|book| book.get_name().unwrap().contains(&s.0))
            .cloned()
            .collect::<Vec<Books>>();
        println!("book_list: {:?}", book_list);
        if book_list.len() > 0 {
            Response::ok(book_list)
        } else {
            Response::not_found(RespData::new(
                -1,
                Some(format!("暂无与 {} 相关的书籍！", s.0)),
            ))
        }
    }

    #[oai(path = "/", method = "post")]
    async fn add_book(&self, mut book: Json<Books>) -> Response<Books, RespData> {
        // {"name":"Rust编程之道","author":"Rust","price":29.99} 为请求体
        let mut books = self.books.lock().await;
        // 检查书籍是否已存在
        let check_book = books
            .iter()
            .find(|b| b.get_name().unwrap() == book.get_name().unwrap());
        if check_book.is_some() {
            return Response::already_exists(RespData::new(
                -1,
                Some(format!(
                    "书籍 {} 已存在！",
                    book.get_name().unwrap().clone()
                )),
            ));
        }
        // 书籍不存在，添加书籍
        let id = books.len() as i32 + 1;
        book.set_id(id);
        book.set_create_time();
        book.set_update_time();
        books.push(book.0.clone());
        println!("book: {:?}", book);
        Response::ok(book.0)
    }

    #[oai(path = "/:id", method = "delete")]
    async fn delete_book(&self, #[oai(name = "id")] id: Path<i32>) -> Response<Books, RespData> {
        let mut books = self.books.lock().await;
        let index = books.iter().position(|book| book.get_id().unwrap() == id.0);
        match index {
            Some(index) => {
                let book = books.remove(index);
                Response::ok(book)
            }
            None => Response::not_found(RespData::new(-1, Some("book not found".to_string()))),
        }
    }

    #[oai(path = "/:id", method = "put")]
    async fn update_book(
        &self,
        #[oai(name = "id")] id: Path<i32>,
        mut book: Json<Books>,
    ) -> Response<Books, RespData> {
        let mut books = self.books.lock().await;
        let index = books.iter().position(|b| b.get_id().unwrap() == id.0);
        match index {
            Some(index) => {
                book.0.set_update_time();
                books[index] = book.0.clone();
                Response::ok(book.0)
            }
            None => Response::not_found(RespData::new(-1, Some("book not found".to_string()))),
        }
    }
}
