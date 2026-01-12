use dotenv::dotenv;
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};
use serde::{Deserialize, Serialize};
use std::env;

use crate::framebuffer::FrameBuffer;

// 天気APIのレスポンス全体
#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherReaponse {
    pub location: Location,
    pub current: Current,
    pub forecast: Forecast,
}

// 場所情報
#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
    pub tz_id: String,
    pub localtime_epoch: i64,
    pub localtime: String,
}

// 現在の天気情報
#[derive(Debug, Deserialize, Serialize)]
pub struct Current {
    pub temp_c: f64,
    pub is_day: u8,
    pub condition: Condition,
}

// 天気状態
#[derive(Debug, Deserialize, Serialize)]
pub struct Condition {
    pub code: u16,
}

// 予報情報
#[derive(Debug, Deserialize, Serialize)]
pub struct Forecast {
    pub forecastday: Vec<ForecastDay>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastDay {
    pub date: String,
    pub date_epoch: i64,
    pub day: Day,
    pub hour: Vec<Hour>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Day {}

// 時間ごとの予報
#[derive(Debug, Deserialize, Serialize)]
pub struct Hour {
    pub time: String,
    pub time_epoch: i64,
    pub temp_c: f64,
    pub is_day: u8,
    pub chance_of_rain: u8,
    pub condition: Condition,
}

// 天気情報加工後のJSON
#[derive(Debug, Deserialize, Serialize)]
pub struct Weather {
    pub current: Current,
    pub forecast: Vec<Hour>,
}

impl Weather {
    fn parse_weather(response: WeatherReaponse) -> Weather {
        let localtime_epoch = response.location.localtime_epoch;

        let forecast: Vec<Hour> = response
            .forecast
            .forecastday
            .into_iter()
            .flat_map(|day| day.hour)
            .filter(|h| h.time_epoch >= localtime_epoch)
            .take(4)
            .collect();

        Weather {
            current: response.current,
            forecast,
        }
    }
}

pub async fn get_weather() -> Result<Weather, Box<dyn std::error::Error + Send + Sync>> {
    let response = fetch_weather().await?;
    Ok(Weather::parse_weather(response))
}

pub async fn fetch_weather() -> Result<WeatherReaponse, Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();

    let api_key = env::var("WEATHERAPI_KEY")?;
    let location = env::var("WEATHER_LOCATION")?;

    let url = format!(
        "http://api.weatherapi.com/v1/forecast.json?key={}&q={}&days=2",
        api_key, location
    );

    let response = reqwest::get(&url)
        .await?
        .error_for_status()?
        .json::<WeatherReaponse>()
        .await?;

    Ok(response)
}

// 天気アイコンの種類
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WeatherIconType {
    Sun,
    Moon,
    Cloud,
    Rain,
    Snow,
    Thunder,
}

// 天気コードからアイコンを取得
pub fn get_weather_icon(code: u16, is_day: bool) -> WeatherIconType {
    match code {
        1000 => {
            if is_day {
                WeatherIconType::Sun
            } else {
                WeatherIconType::Moon
            }
        }
        1003 | 1006 | 1009 | 1030 | 1135 | 1147 => WeatherIconType::Cloud,
        1063 | 1072 | 1150 | 1153 | 1168 | 1171 | 1180 | 1183 | 1186 | 1189 | 1192 | 1195
        | 1198 | 1201 | 1240 | 1243 | 1246 => WeatherIconType::Rain,
        1066 | 1069 | 1114 | 1117 | 1204 | 1207 | 1210 | 1213 | 1216 | 1219 | 1222 | 1225
        | 1237 | 1249 | 1252 | 1255 | 1258 | 1261 | 1264 => WeatherIconType::Snow,
        1087 | 1273 | 1276 | 1279 | 1282 => WeatherIconType::Thunder,
        _ => WeatherIconType::Cloud,
    }
}

// 16x16 太陽アイコン
const ICON_SUN: [u16; 16] = [
    0b0000000100000000,
    0b0000000100000000,
    0b0010000100001000,
    0b0001000000010000,
    0b0000011111100000,
    0b0000111111110000,
    0b0000111111110000,
    0b0100111111110010,
    0b0000111111110000,
    0b0000111111110000,
    0b0000011111100000,
    0b0001000000010000,
    0b0010000100001000,
    0b0000000100000000,
    0b0000000100000000,
    0b0000000000000000,
];

// 16x16 月アイコン
const ICON_MOON: [u16; 16] = [
    0b0000000110000000,
    0b0000011100000000,
    0b0001100100000000,
    0b0010001000000000,
    0b0100001000000000,
    0b0100001000000000,
    0b1000001000000000,
    0b1000000100000000,
    0b1000000100000000,
    0b1000000010000000,
    0b1000000001000001,
    0b0100000000111110,
    0b0100000000000010,
    0b0010000000000100,
    0b0001100000011000,
    0b0000011111100000,
];

// 16x16 雲アイコン
const ICON_CLOUD: [u16; 16] = [
    0b0000000000000000,
    0b0000000000000000,
    0b0000011110000000,
    0b0000111111000000,
    0b0001111111100000,
    0b0011111111110000,
    0b0111111111111000,
    0b1111111111111100,
    0b1111111111111100,
    0b0111111111111000,
    0b0000000000000000,
    0b0000000000000000,
    0b0000000000000000,
    0b0000000000000000,
    0b0000000000000000,
    0b0000000000000000,
];

// 16x16 雨アイコン（雲+雨滴）
const ICON_RAIN: [u16; 16] = [
    0b0000000000000000,
    0b0000011110000000,
    0b0000111111000000,
    0b0001111111100000,
    0b0011111111110000,
    0b0111111111111000,
    0b1111111111111100,
    0b1111111111111100,
    0b0111111111111000,
    0b0000000000000000,
    0b0010010010010000,
    0b0010010010010000,
    0b0100100100100000,
    0b0000000000000000,
    0b0000000000000000,
    0b0000000000000000,
];

// 16x16 雪アイコン（雲+雪）
const ICON_SNOW: [u16; 16] = [
    0b0000000000000000,
    0b0000011110000000,
    0b0000111111000000,
    0b0001111111100000,
    0b0011111111110000,
    0b0111111111111000,
    0b1111111111111100,
    0b1111111111111100,
    0b0111111111111000,
    0b0000000000000000,
    0b0010001000100000,
    0b0000100010000000,
    0b0010001000100000,
    0b0000100010000000,
    0b0000000000000000,
    0b0000000000000000,
];

// 16x16 雷アイコン
const ICON_THUNDER: [u16; 16] = [
    0b0000000000000000,
    0b0000011110000000,
    0b0000111111000000,
    0b0001111111100000,
    0b0011111111110000,
    0b0111111111111000,
    0b1111111111111100,
    0b1111111111111100,
    0b0111111111111000,
    0b0000011100000000,
    0b0000111000000000,
    0b0001111110000000,
    0b0000011100000000,
    0b0000111000000000,
    0b0000110000000000,
    0b0000000000000000,
];

fn draw_pixel(fb: &mut FrameBuffer, x: i32, y: i32, size: i32, color: Rgb888) {
    Rectangle::new(Point::new(x, y), Size::new(size as u32, size as u32))
        .into_styled(PrimitiveStyle::with_fill(color))
        .draw(fb)
        .unwrap();
}

pub fn draw_weather_icon(fb: &mut FrameBuffer, icon: WeatherIconType, x: i32, y: i32, scale: i32) {
    let (bitmap, color) = match icon {
        WeatherIconType::Sun => (&ICON_SUN, Rgb888::new(255, 200, 50)),
        WeatherIconType::Moon => (&ICON_MOON, Rgb888::new(200, 200, 150)),
        WeatherIconType::Cloud => (&ICON_CLOUD, Rgb888::new(180, 180, 180)),
        WeatherIconType::Rain => (&ICON_RAIN, Rgb888::new(100, 150, 200)),
        WeatherIconType::Snow => (&ICON_SNOW, Rgb888::new(200, 220, 255)),
        WeatherIconType::Thunder => (&ICON_THUNDER, Rgb888::new(255, 255, 100)),
    };

    for (row, &bits) in bitmap.iter().enumerate() {
        for col in 0..16 {
            if (bits >> (15 - col)) & 1 == 1 {
                draw_pixel(
                    fb,
                    x + (col as i32) * scale,
                    y + (row as i32) * scale,
                    scale,
                    color,
                );
            }
        }
    }
}
