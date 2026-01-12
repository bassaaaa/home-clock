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
    HeavyRain,
    Snow,
    Thunder,
}

// 天気コードからアイコンを取得
pub fn get_weather_icon(code: u16, is_day: bool) -> WeatherIconType {
    match code {
        // 晴れ
        1000 => {
            if is_day {
                WeatherIconType::Sun
            } else {
                WeatherIconType::Moon
            }
        }
        // 曇り、霧（晴れ時々曇りも含む）
        1003 | 1006 | 1009 | 1030 | 1135 | 1147 => WeatherIconType::Cloud,
        // 小雨〜並の雨
        1063 | 1072 | 1150 | 1153 | 1168 | 1180 | 1183 | 1186 | 1189 | 1198 | 1240 => {
            WeatherIconType::Rain
        }
        // 大雨、激しい雨
        1171 | 1192 | 1195 | 1201 | 1243 | 1246 => WeatherIconType::HeavyRain,
        // 雪、みぞれ、あられ
        1066 | 1069 | 1114 | 1117 | 1204 | 1207 | 1210 | 1213 | 1216 | 1219 | 1222 | 1225
        | 1237 | 1249 | 1252 | 1255 | 1258 | 1261 | 1264 => WeatherIconType::Snow,
        // 雷
        1087 | 1273 | 1276 | 1279 | 1282 => WeatherIconType::Thunder,
        _ => WeatherIconType::Cloud,
    }
}

// PNG アイコンデータ（コンパイル時に埋め込み）
const ICON_SUN_PNG: &[u8] = include_bytes!("../assets/icons/sun.png");
const ICON_MOON_PNG: &[u8] = include_bytes!("../assets/icons/moon.png");
const ICON_CLOUD_PNG: &[u8] = include_bytes!("../assets/icons/cloud.png");
const ICON_RAIN_PNG: &[u8] = include_bytes!("../assets/icons/rain.png");
const ICON_HEAVY_RAIN_PNG: &[u8] = include_bytes!("../assets/icons/heavy_rain.png");
const ICON_SNOW_PNG: &[u8] = include_bytes!("../assets/icons/snow.png");
const ICON_THUNDER_PNG: &[u8] = include_bytes!("../assets/icons/thunder.png");

fn draw_pixel(fb: &mut FrameBuffer, x: i32, y: i32, size: i32, color: Rgb888) {
    Rectangle::new(Point::new(x, y), Size::new(size as u32, size as u32))
        .into_styled(PrimitiveStyle::with_fill(color))
        .draw(fb)
        .unwrap();
}

fn draw_png_icon(fb: &mut FrameBuffer, png_data: &[u8], x: i32, y: i32, scale: i32) {
    use image::GenericImageView;

    let img = image::load_from_memory(png_data).expect("Failed to load PNG icon");

    for (px, py, pixel) in img.pixels() {
        let [r, g, b, a] = pixel.0;
        // 透明度が128以上のピクセルのみ描画
        if a > 128 {
            draw_pixel(
                fb,
                x + (px as i32) * scale,
                y + (py as i32) * scale,
                scale,
                Rgb888::new(r, g, b),
            );
        }
    }
}

pub fn draw_weather_icon(fb: &mut FrameBuffer, icon: WeatherIconType, x: i32, y: i32, scale: i32) {
    let png_data = match icon {
        WeatherIconType::Sun => ICON_SUN_PNG,
        WeatherIconType::Moon => ICON_MOON_PNG,
        WeatherIconType::Cloud => ICON_CLOUD_PNG,
        WeatherIconType::Rain => ICON_RAIN_PNG,
        WeatherIconType::HeavyRain => ICON_HEAVY_RAIN_PNG,
        WeatherIconType::Snow => ICON_SNOW_PNG,
        WeatherIconType::Thunder => ICON_THUNDER_PNG,
    };

    draw_png_icon(fb, png_data, x, y, scale);
}
