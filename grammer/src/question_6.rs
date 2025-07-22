// Question-5) Guess Number
// CUI上で対話形式で数当てゲーム

use rand::Rng;
use std::io::{self, Write};
use std::cmp::Ordering;

fn check_number(val: i32, ans: i32) -> bool {
    if ans == val {
        true
    } else {
        if ans > val {
            println!("More Bigger Number...")
        } else {
            println!("More Smaller Number...")
        }
        false
    }
}

fn input_number() -> Result<i32, String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");

    let val = buffer.trim().parse::<i32>();

    if val.is_err() {
        Err(String::from(format!("Invalid Input : {}", buffer.trim())))
    } else {
        Ok(val.unwrap())
    }
}

/// 実行処理
/// 
/// - 引数 : None
/// - 戻り値 : None
pub fn proc() {
    println!("\n(Question-6) Guess Number");

    // 1. 解答を生成
    let mut rng  = rand::rng();
    let ans: i32 = rng.random_range(0..=10);

    for _cnt in 0..3 {
        // 2. ユーザ入力
        print!("Please input number >>> ");
        io::stdout().flush().unwrap();

        let input;
        match input_number() {
            Ok(val) => input = val,
            Err(e) => {
                println!("{}", e);
                return;
            },
        }

        // 3. 判定
        if check_number(input, ans) {
            println!("Correct!!!");
            return
        }
    }
    println!("Game Over!!! ans = {}", ans)
}

/// this code is the sample of official web site
fn _sample() {
    let mut guess = String::new();
    let secret_number = rand::rng().random_range(1..101);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");                 //数値を入力してください！

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}