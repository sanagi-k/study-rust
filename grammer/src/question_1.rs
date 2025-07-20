// Question-1)
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
    for light in TrafficLight::iter() {
        show_light(light);
    }
}