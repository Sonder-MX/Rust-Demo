use thirtyfour::error::WebDriverResult;
use thirtyfour::{By, DesiredCapabilities, WebDriver};

use std::{thread, time::Duration};

async fn close_modal(driver: &WebDriver) {
    let close_btn = driver
        .find(By::XPath(
            "//*[@id=\"desktop-guide-wrapper\"]/div/div/div/a[2]",
        ))
        .await;
    if let Ok(close_btn) = close_btn {
        let click_event = close_btn.click().await;
        if let Ok(_) = click_event {
            println!("close modal");
        }
    }
}

async fn input_content(driver: &WebDriver, content: &str) -> WebDriverResult<()> {
    let input = driver.find(By::Id("baidu_translate_input")).await?;
    input.send_keys(content).await?;
    Ok(())
}

async fn get_result(driver: &WebDriver) -> WebDriverResult<()> {
    let result = driver
        .find(By::XPath(
            "//*[@id=\"main-outer\"]/div/div/div[1]/div[2]/div[1]/div[2]/div/div/div[1]/p[2]",
        ))
        .await?;
    let text = result.text().await?;
    println!("result: {}", text);
    Ok(())
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let url = "https://fanyi.baidu.com/"; // 目标网址
    let caps = DesiredCapabilities::chrome();
    // caps.set_headless()?; // 设置无头模式
    let driver = WebDriver::new("http://127.0.0.1:9000", caps).await?; // 本地服务地址
    driver.goto(url).await?;

    thread::sleep(Duration::from_millis(500));
    close_modal(&driver).await;
    thread::sleep(Duration::from_millis(500));

    // 输入内容
    input_content(
        &driver,
        "The greatest test of courage on earth is to bear defeat without losing heart.",
    )
    .await?;
    thread::sleep(Duration::from_secs(1));
    get_result(&driver).await?;
    thread::sleep(Duration::from_secs(2));

    driver.quit().await?;

    Ok(())
}
