mod thread1;
mod thread2;
mod thread3;
mod thread_ctrl;

use std::thread;
use thread_ctrl::ThreadCtrl;
use thread1::Thread1;
use thread2::Thread2;
use thread3::Thread3;

/// スレッド初期化をすべて順に実行
fn init_all(runners: &Vec<(String, Box<dyn ThreadCtrl>)>) -> Result<(), String> {
    println!("初期化フェーズ開始");
    for (name, runner) in runners.iter() {
        println!("{}: 初期化中...", name);
        if let Err(e) = runner.init() {
            eprintln!("{}: 初期化失敗: {}", name, e);
            return Err(format!("{}: 初期化に失敗しました。", name));
        }
        println!("{}: 初期化完了", name);
    }
    println!("全スレッドの初期化完了");
    Ok(())
}

/// run() を全スレッドで並列実行
fn run_all(runners: Vec<(String, Box<dyn ThreadCtrl>)>) {
    println!("スレッド実行フェーズ開始");

    let handles: Vec<_> = runners
        .into_iter()
        .map(|(name, runner)| {
            thread::spawn(move || {
                if let Err(e) = runner.run() {
                    eprintln!("{}: 実行エラー: {}", name, e);
                } else {
                    println!("{}: 実行完了", name);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("すべてのスレッド処理が完了しました。");
}

fn main() {
    // 名前付きトレイトオブジェクトを構築
    let runners: Vec<(String, Box<dyn ThreadCtrl>)> = vec![
        ("スレッド1".to_string(), Box::new(Thread1)),
        ("スレッド2".to_string(), Box::new(Thread2)),
        ("スレッド3".to_string(), Box::new(Thread3)),
    ];

    // すべての初期化が成功すれば run を開始
    if let Err(e) = init_all(&runners) {
        eprintln!("初期化失敗: {}", e);
        return;
    }

    run_all(runners);
}
