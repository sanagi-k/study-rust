// Question-3) Study : Result型
// 0で割ろうとしたらエラーを返す関数を作ってください

use rand::Rng;

fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    // 実装
    if b as u64 == 0 {
        Err(String::from(format!("Invalid Input b = {}", b)))
    } else {
        Ok(a/b)
    }
}

pub fn proc() {
    println!("\n(Qeustion-3) Result Type");
    let mut rng = rand::rng();

    for _cnt in 0..10 {
        let a: i32 = rng.random_range(0..255);
        let b: i32 = rng.random_range(0..5);
        match safe_divide(a as f64, b as f64) {
            Ok(result) => println!("結果: {} (a = {}, b = {})", result, a, b),
            Err(e) => println!("エラー: {} (a = {}, b = {})", e, a, b),
        }
    }
}
