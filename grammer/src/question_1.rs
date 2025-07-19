// Question-1)
// enumを使って信号の状態を定義し、matchで表示を分岐してください。

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
    let light = TrafficLight::Red;
    show_light(light);
    let light = TrafficLight::Yellow;
    show_light(light);
    let light = TrafficLight::Green;
    show_light(light);
}