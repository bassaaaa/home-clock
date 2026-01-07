mod weather {
    include!("../weather.rs");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let weather = weather::get_weather().await?;
    println!("{}", serde_json::to_string_pretty(&weather)?);
    Ok(())
}