use anyhow::Context;
use clap::Parser as _;

#[derive(clap::Parser, Debug)]
struct Args {
    /// Latitude
    #[clap(long, allow_hyphen_values = true)]
    lat: f32,

    /// Longitude
    #[clap(long, allow_hyphen_values = true)]
    lon: f32,

    /// User-Agent
    #[clap(long)]
    user_agent: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let client = reqwest::blocking::Client::new();

    let point_url = format!("https://api.weather.gov/points/{},{}", args.lat, args.lon);

    let point_res: serde_json::Value = client
        .get(point_url)
        .header("user-agent", &args.user_agent)
        .send()
        .context("failed to request point")?
        .json()
        .context("failed to decode point")?;

    let forecast_url = point_res
        .get("properties")
        .context("point missing 'properties' key")?
        .get("forecast")
        .context("point missing 'properties.forecast' key")?
        .as_str()
        .context("point 'properties.forecast' is not string")?;

    let forecast_res: serde_json::Value = client
        .get(forecast_url)
        .header("user-agent", &args.user_agent)
        .send()
        .context("failed to request forecast")?
        .json()
        .context("failed to decode forecast")?;

    let forecast_text: &str = forecast_res
        .get("properties")
        .context("forecast missing 'properties' key")?
        .get("periods")
        .context("forecast missing 'properties.periods' key")?
        .get(0)
        .context("forecast missing 'properties.periods.[0]' element")?
        .get("detailedForecast")
        .context("forecast missing 'properties.periods.[0].detailedForecast' key")?
        .as_str()
        .context("forecast 'properties.periods.[0].detailedForecast' is not string")?;

    println!("{forecast_text}");

    Ok(())
}
