use reqwest::blocking as reqs;
use scraper::{Html, Selector};

fn get_read_content(url: &str) -> Vec<String> {
    let mut content = vec![];
    let text_page = reqs::get(url).unwrap().text().expect("获取页面失败");
    let html_doc = Html::parse_document(&text_page);
    let all_books_select = Selector::parse("div.book-large").expect("解析失败！");
    let book_name_select = Selector::parse("h4.title").unwrap();
    for book_div in html_doc.select(&all_books_select) {
        let book_name = book_div
            .select(&book_name_select)
            .next()
            .expect("解析失败！")
            .inner_html();
        content.push(book_name);
    }
    content
}

fn main() {
    let url = "https://book.qq.com/book-rank/male-god/cycle-1";
    get_read_content(url)
        .iter()
        .enumerate()
        .for_each(|(ind, book_name)| println!("第{}名 -> 《{}》", ind + 1, book_name));
}
