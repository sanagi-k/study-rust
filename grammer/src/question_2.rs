// Question-2)
// 正の数ならSome(平方根)、負ならNoneを返す関数を作ってください

fn safe_sqrt(n: f64) -> Option<f64> {
    if n < 0.0 {
        None
    } else {
        Some(n.sqrt())
    }
}

pub fn proc() {
    println!("\n(Question-2)");

    for num in -9..=10{
        print!("num = {} : ", num as f64);
        match safe_sqrt(num as f64) {
            Some(result) => println!("平方根: {}", result),
            None => println!("エラー: 負の数です"),
        }
    }
}