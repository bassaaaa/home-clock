mod clock;
mod font;
mod forecast;
mod framebuffer;
mod weather;

use chrono::{Datelike, Local};
use minifb::{Key, Window, WindowOptions};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use clock::{draw_date, draw_time};
use forecast::draw_forecast;
use framebuffer::FrameBuffer;
use weather::{get_weather, Weather};

const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 480;
const BG_COLOR: u32 = 0x001020;

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut fb = FrameBuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut window = Window::new(
        "Home Clock",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .expect("ウィンドウの作成に失敗しました");

    window.set_target_fps(30);

    // 天気データを保持
    let weather_data: Arc<Mutex<Option<Weather>>> = Arc::new(Mutex::new(None));
    let mut last_weather_fetch = Instant::now();
    let mut first_run = true;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // 起動時と10分ごとに天気を取得
        if first_run || last_weather_fetch.elapsed() > Duration::from_secs(600) {
            first_run = false;
            let weather_clone = Arc::clone(&weather_data);
            rt.spawn(async move {
                if let Ok(weather) = get_weather().await {
                    let mut data = weather_clone.lock().unwrap();
                    *data = Some(weather);
                }
            });
            last_weather_fetch = Instant::now();
        }

        fb.clear(BG_COLOR);

        let now = Local::now();
        let hour = now.format("%H").to_string().parse::<u8>().unwrap();
        let minute = now.format("%M").to_string().parse::<u8>().unwrap();
        let year = now.year() as u16;
        let month = now.month() as u8;
        let day = now.day() as u8;
        let weekday = now.weekday();

        let blink = now.timestamp_subsec_millis() < 500;

        draw_date(&mut fb, year, month, day, weekday);
        draw_time(&mut fb, hour, minute, blink);

        // 予報を描画
        if let Ok(data) = weather_data.lock() {
            if let Some(ref weather) = *data {
                draw_forecast(&mut fb, &weather.forecast);
            }
        }

        window
            .update_with_buffer(&fb.buffer, fb.width, fb.height)
            .expect("バッファの更新に失敗しました");

        std::thread::sleep(Duration::from_millis(16));
    }
}
