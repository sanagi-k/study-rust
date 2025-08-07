use gst::prelude::*;
use gstreamer as gst;

fn main() {
    gst::init().unwrap();

    // すべてのタイプ、最低ランク NONE 以上を取得
    let mut factories =
        gst::ElementFactory::factories_with_type(gst::ElementFactoryType::all(), gst::Rank::NONE);

    // klass の文字列でソート
    factories.sort_by(|a, b| a.klass().cmp(b.klass()));

    println!("Registered ElementFactories (sorted by klass):");
    for factory in factories {
        println!(
            "{:<30}:{:?<30} - {}",
            factory.klass(),
            factory.name(),
            factory.description()
        );
    }

    //let source = gst::ElementFactory::make("videotestsrc");
    //let sink = gst::ElementFactory::make("osxvideosink");

    let factory = gst::ElementFactory::find("videotestsrc").unwrap();
    let element = factory
        .create()
        .name("my-source") // 名前をつける（省略可能）
        .build()
        .unwrap();

    println!("Element created: {}", element.name());
}
