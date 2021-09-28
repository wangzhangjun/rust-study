use std::fs;

/*
1. 访问结构体的成员函数或者变量使用.运算符。
2. 访问命名空间或者对象的静态函数使用双冒号：：
3. 简化命名空间，可以使用use
*/
fn main() {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";
    
    println!("Fetching url: {}",url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Coverting html to markdown...");
    let md = html2md::parse_html(&body);
    
    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved in {}.", output);
}
