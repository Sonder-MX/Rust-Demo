use std::collections::HashMap;

fn main() {
    let mut word_map = HashMap::new();

    let my_str = String::from("Hello Rust Hello Python");
    let word_vec: Vec<_> = my_str.split(" ").collect(); // _ 自动推断

    word_vec.iter().for_each(|&word| {
        let count = word_map.entry(word).or_insert(0);
        *count += 1;
    });

    for (k, v) in &word_map {
        println!("单词: {k:^10} 出现了{v}次"); // 宽度为10 居中对齐
    }
}
