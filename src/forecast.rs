use embedded_graphics::pixelcolor::Rgb888;

use crate::font::{draw_colon, draw_digit, DIGIT_WIDTH};
use crate::framebuffer::FrameBuffer;
use crate::weather::{draw_weather_icon, get_weather_icon, Hour};

// 予報1件分を描画（item_widthを渡して中央寄せ計算）
fn draw_forecast_item(fb: &mut FrameBuffer, hour: &Hour, center_x: i32, y: i32) {
    let pixel_size = 2;
    let digit_width = DIGIT_WIDTH as i32 * pixel_size;
    let colon_width = 2 * pixel_size;
    let spacing = pixel_size;

    // 時間を抽出 (例: "2024-01-12 22:00" -> hour=22, minute=00)
    let time_str = &hour.time;
    let hour_val: u8 = time_str
        .split(' ')
        .nth(1)
        .and_then(|t| t.split(':').next())
        .and_then(|h| h.parse().ok())
        .unwrap_or(0);

    let color = Rgb888::new(150, 150, 150);

    // 時間表示 (HH:00) の幅を計算して中央寄せ
    // HH:00 = digit + spacing + digit + spacing + colon + spacing + digit + spacing + digit
    let time_width = digit_width * 4 + colon_width + spacing * 4;
    let time_x = center_x - time_width / 2;

    let mut x = time_x;
    draw_digit(fb, hour_val / 10, x, y, pixel_size, color);
    x += digit_width + spacing;
    draw_digit(fb, hour_val % 10, x, y, pixel_size, color);
    x += digit_width + spacing;

    // コロン
    draw_colon(fb, x, y, pixel_size, color, true);
    x += colon_width + spacing;

    // 分 (00)
    draw_digit(fb, 0, x, y, pixel_size, color);
    x += digit_width + spacing;
    draw_digit(fb, 0, x, y, pixel_size, color);

    // アイコン表示（中央寄せ）
    let icon = get_weather_icon(hour.condition.code, hour.is_day != 0);
    let icon_size = 16 * 2; // 16px * scale 2
    let icon_x = center_x - icon_size / 2;
    let icon_y = y + 30;
    draw_weather_icon(fb, icon, icon_x, icon_y, 2);

    // 降水確率表示（中央寄せ）
    let rain_y = icon_y + 40;
    let rain_color = if hour.chance_of_rain >= 50 {
        Rgb888::new(100, 150, 255)
    } else {
        Rgb888::new(120, 120, 120)
    };

    let rain = hour.chance_of_rain;

    // 降水確率の幅を計算（数字 + %記号）
    let rain_digits = if rain >= 100 { 3 } else if rain >= 10 { 2 } else { 1 };
    let percent_width = digit_width; // P文字の幅
    let rain_width = digit_width * rain_digits + spacing * rain_digits + percent_width;
    let rain_x = center_x - rain_width / 2;

    let mut rx = rain_x;
    if rain >= 100 {
        draw_digit(fb, 1, rx, rain_y, pixel_size, rain_color);
        rx += digit_width + spacing;
        draw_digit(fb, 0, rx, rain_y, pixel_size, rain_color);
        rx += digit_width + spacing;
        draw_digit(fb, 0, rx, rain_y, pixel_size, rain_color);
        rx += digit_width + spacing;
    } else if rain >= 10 {
        draw_digit(fb, rain / 10, rx, rain_y, pixel_size, rain_color);
        rx += digit_width + spacing;
        draw_digit(fb, rain % 10, rx, rain_y, pixel_size, rain_color);
        rx += digit_width + spacing;
    } else {
        draw_digit(fb, rain, rx, rain_y, pixel_size, rain_color);
        rx += digit_width + spacing;
    }

    // % 記号（Pで代用）
    draw_digit(fb, 0, rx, rain_y, pixel_size, Rgb888::new(0, 0, 0)); // 消す
    // %を描画（簡易的に小さく）
    draw_percent(fb, rx, rain_y, pixel_size, rain_color);
}

// %記号を描画
fn draw_percent(fb: &mut FrameBuffer, x: i32, y: i32, pixel_size: i32, color: Rgb888) {
    use embedded_graphics::{
        prelude::*,
        primitives::{PrimitiveStyle, Rectangle},
    };

    // 簡易的な%表示
    let ps = pixel_size;

    // 上の丸
    Rectangle::new(
        Point::new(x, y),
        embedded_graphics::geometry::Size::new(ps as u32 * 2, ps as u32 * 2),
    )
    .into_styled(PrimitiveStyle::with_fill(color))
    .draw(fb)
    .unwrap();

    // 斜線
    for i in 0..5 {
        Rectangle::new(
            Point::new(x + (4 - i) * ps, y + (i + 1) * ps),
            embedded_graphics::geometry::Size::new(ps as u32, ps as u32),
        )
        .into_styled(PrimitiveStyle::with_fill(color))
        .draw(fb)
        .unwrap();
    }

    // 下の丸
    Rectangle::new(
        Point::new(x + ps * 3, y + ps * 5),
        embedded_graphics::geometry::Size::new(ps as u32 * 2, ps as u32 * 2),
    )
    .into_styled(PrimitiveStyle::with_fill(color))
    .draw(fb)
    .unwrap();
}

// 4時間分の予報を画面下部に表示
pub fn draw_forecast(fb: &mut FrameBuffer, forecast: &[Hour]) {
    let start_y = 360;
    let item_width = 180;
    let num_items = forecast.len().min(4) as i32;
    let total_width = item_width * num_items;
    let start_x = (800 - total_width) / 2;

    for (i, hour) in forecast.iter().take(4).enumerate() {
        // 各アイテムの中央X座標を計算
        let center_x = start_x + (i as i32) * item_width + item_width / 2;
        draw_forecast_item(fb, hour, center_x, start_y);
    }
}
