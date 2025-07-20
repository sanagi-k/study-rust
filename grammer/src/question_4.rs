// Question-4) Ownership and Borrowing
// 借用（&String）を使って、文字列の長さを返す関数を作ってください

fn get_length(s: &String) -> usize {
    // 実装
    s.len()
}

fn chage_and_get_length(s: &mut String) -> usize{
    s.push_str("+XYZ");
    s.len()
}

pub fn proc() {
    println!("\n(Qeustion-4) Ownership and Borrowing");

    let name = String::from("abcdefghijklmnopqrstuvwxyz");
    let len = get_length(&name);
    println!("文字列の長さ: {} (str = {})", len, name);

    let mut name2: String = String::from("abcdefghijklmnopqrstuvwxyz");
    let len = chage_and_get_length(&mut name2);
    println!("文字列の長さ: {} (str = {})", len, name2);
}
