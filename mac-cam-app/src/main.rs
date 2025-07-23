use gst::MessageView;
use gst::parse;
use gst::prelude::*;
use gstreamer as gst;
use std::thread;

#[cfg(target_os = "macos")]
mod macos_gui {
    use objc2::MainThreadMarker;
    use objc2_app_kit::{NSApp, NSApplication};

    pub fn run() {
        let mtm = MainThreadMarker::new().unwrap();
        let _ = NSApplication::sharedApplication(mtm);

        // macOSのメインイベントループ開始（これがないとウィンドウが表示されない）
        NSApp(mtm).run();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // GStreamer 初期化
    gst::init()?;

    // 内蔵カメラ（avfvideosrc）を使って表示
    let pipeline = parse::launch(
        "avfvideosrc device-index=0 ! videoconvert ! video/x-raw,format=UYVY ! videoconvert ! 
         queue ! glimagesink sync=false",
    )?;

    // パイプラインを再生状態に
    pipeline.set_state(gst::State::Playing)?;

    // メッセージ監視
    let bus = pipeline.bus().unwrap();
    thread::spawn(move || {
        for msg in bus.iter_timed(gst::ClockTime::NONE) {
            match msg.view() {
                MessageView::Eos(..) => break,
                MessageView::Error(err) => {
                    eprintln!(
                        "Error from {:?}: {} ({:?})",
                        err.src().map(|s| s.path_string()),
                        err.error(),
                        err.debug()
                    );
                    break;
                }
                _ => (),
            }
        }
    });

    #[cfg(target_os = "macos")]
    macos_gui::run();

    // パイプラインを停止
    pipeline.set_state(gst::State::Null)?;
    Ok(())
}
