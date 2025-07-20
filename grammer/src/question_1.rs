// Question-1) Study : enum, match
// enumを使って信号の状態を定義し、matchで表示を分岐してください。

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn show_light(light: TrafficLight) {
    // ここをmatchで分岐
    match light {
        TrafficLight::Red => println!("Red"),
        TrafficLight::Yellow => println!("Yellow"),
        TrafficLight::Green => println!("Green")
    }
}

pub fn proc() {
    println!("\n(Question-1) enum, match test");

    for light in TrafficLight::iter() {
        show_light(light);
    }
}