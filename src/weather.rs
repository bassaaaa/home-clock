use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

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

pub async fn get_weather() -> Result<Weather, Box<dyn std::error::Error>> {
    let response = fetch_weather().await?;
    Ok(Weather::parse_weather(response))
}

pub async fn fetch_weather() -> Result<WeatherReaponse, Box<dyn std::error::Error>> {
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
