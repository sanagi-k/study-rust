// Question-5) Lifetime
// ライフタイムを指定して、どちらが長いか比較して返す関数を作ってください

fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    // 実装
    if s1.len() >= s2.len() {
        s1
    } else {
        s2
    }
}

pub fn proc() {
    println!("\n(Question-5) Lifetime");

    let str1 = String::from("Rust");
    let str2 = String::from("Ownership");
    let result = longer(&str1, &str2);
    println!("長い文字列: {}", result);
}
