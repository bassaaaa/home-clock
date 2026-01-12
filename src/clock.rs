use chrono::Weekday;
use embedded_graphics::pixelcolor::Rgb888;

use crate::font::{draw_colon, draw_digit, draw_hyphen, draw_text, DIGIT_WIDTH};
use crate::framebuffer::FrameBuffer;

fn get_weekday_str(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Mon => "MON",
        Weekday::Tue => "TUE",
        Weekday::Wed => "WED",
        Weekday::Thu => "THU",
        Weekday::Fri => "FRI",
        Weekday::Sat => "SAT",
        Weekday::Sun => "SUN",
    }
}

pub fn draw_date(fb: &mut FrameBuffer, year: u16, month: u8, day: u8, weekday: Weekday) {
    let pixel_size = 3;
    let digit_width = DIGIT_WIDTH as i32 * pixel_size;
    let hyphen_width = DIGIT_WIDTH as i32 * pixel_size;
    let letter_width = DIGIT_WIDTH as i32 * pixel_size;
    let spacing = pixel_size;

    // YYYY-MM-DD WEEKDAYの全体幅を計算
    let weekday_str = get_weekday_str(weekday);
    let weekday_width = (weekday_str.len() as i32) * (letter_width + spacing) - spacing;
    let date_width = digit_width * 8 + hyphen_width * 2 + spacing * 9;
    let total_width = date_width + spacing * 2 + weekday_width;

    let start_x = (800 - total_width) / 2;
    let start_y = 40;

    let color = Rgb888::new(180, 180, 180);

    let mut x = start_x;

    // 年 (4桁)
    draw_digit(
        fb,
        ((year / 1000) % 10) as u8,
        x,
        start_y,
        pixel_size,
        color,
    );
    x += digit_width + spacing;
    draw_digit(fb, ((year / 100) % 10) as u8, x, start_y, pixel_size, color);
    x += digit_width + spacing;
    draw_digit(fb, ((year / 10) % 10) as u8, x, start_y, pixel_size, color);
    x += digit_width + spacing;
    draw_digit(fb, (year % 10) as u8, x, start_y, pixel_size, color);
    x += digit_width + spacing;

    // ハイフン
    draw_hyphen(fb, x, start_y, pixel_size, color);
    x += hyphen_width + spacing;

    // 月
    draw_digit(fb, month / 10, x, start_y, pixel_size, color);
    x += digit_width + spacing;
    draw_digit(fb, month % 10, x, start_y, pixel_size, color);
    x += digit_width + spacing;

    // ハイフン
    draw_hyphen(fb, x, start_y, pixel_size, color);
    x += hyphen_width + spacing;

    // 日
    draw_digit(fb, day / 10, x, start_y, pixel_size, color);
    x += digit_width + spacing;
    draw_digit(fb, day % 10, x, start_y, pixel_size, color);
    x += digit_width + spacing * 3;

    // 曜日
    draw_text(fb, weekday_str, x, start_y, pixel_size, color);
}

pub fn draw_time(fb: &mut FrameBuffer, hour: u8, minute: u8, blink: bool) {
    let pixel_size = 16;
    let digit_width = DIGIT_WIDTH as i32 * pixel_size;
    let colon_width = 2 * pixel_size;
    let spacing = pixel_size;

    // HH:MM の全体幅を計算
    let total_width = digit_width * 4 + colon_width + spacing * 4;
    let start_x = (800 - total_width) / 2;
    let start_y = 120;

    let color = Rgb888::new(255, 255, 255);

    let mut x = start_x;

    // 時
    draw_digit(fb, hour / 10, x, start_y, pixel_size, color);
    x += digit_width + spacing;
    draw_digit(fb, hour % 10, x, start_y, pixel_size, color);
    x += digit_width + spacing;

    // コロン
    draw_colon(fb, x, start_y, pixel_size, color, blink);
    x += colon_width + spacing;

    // 分
    draw_digit(fb, minute / 10, x, start_y, pixel_size, color);
    x += digit_width + spacing;
    draw_digit(fb, minute % 10, x, start_y, pixel_size, color);
}
