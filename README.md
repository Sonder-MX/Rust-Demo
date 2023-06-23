# Rust-Demo

环境

- Rust==1.67
- Visual Studio Code

运行 eg:

- cd ./guess_number
- cargo run

## [1. 猜数字](/guess_number)

```
[dependencies]
rand = "0.8.5"
```

## [2. 统计单词出现的次数](/word_count)

```rust
use std::collections::HashMap;
```

## [3. 递归和循环斐波那契数列](/fibonacci_seq)

## [4. 人员信息管理](/manage_info)

- 结构体
- 枚举

## [5. 简单网络爬虫-QQ 阅读小说排行](/web_crawler)

```
reqwest = { version = "0.11", features = ["blocking"] }
scraper = "0.14"
```

- reqwest: 网络请求库
- scraper: HTML 选择器

## [6. 二分查找](/binary_search)

- 有序

## [7. 二叉树遍历](/binary_tree)

- BFS
- DFS (先序)

## [8. BFS 和 DFS](/bfs_dfs)

- BFS: 求最大岛屿面积
- DFS: 求所有可能的路径

## [9. Web 自动化测试--百度翻译](/web_auto_test)

1. 下载 Chrome 浏览器
2. 下载与之版本匹配的的 [Chromedriver](http://chromedriver.storage.googleapis.com/index.html)
3. 将下载的 Chromedriver 解压后,进入该目录,终端运行命令: `.\chromedriver.exe --port=9000`
4. 运行该项目: `cargo run`

```
tokio = { version = "1", features = ["full"] }
thirtyfour = "0.31.0"
rand = "0.8.5"
```
