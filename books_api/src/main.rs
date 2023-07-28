mod controller;
mod models;
mod resp;
mod settings;

use std::env;

use controller::bookser::BooksApi;
use settings::Settings;

use poem::{listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let mut settings = Settings::new();
    settings.read_args(&args);

    // 创建OpenApiService
    let books_service = OpenApiService::new(BooksApi::default(), "图书管理API", "v1.0.0")
        .server(format!("http://{}/books", settings.ip_port()));
    let books_ui = books_service.swagger_ui();
    // 创建路由
    let route = Route::new()
        .nest("/books", books_service)
        .nest("/", books_ui);
    // 启动服务
    println!("Listening on http://{} ......", settings.ip_port());
    Server::new(TcpListener::bind(settings.ip_port()))
        .run(route)
        .await
}
