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

    let point_res: GeoJson<Point> = client
        .get(format!(
            "https://api.weather.gov/points/{},{}",
            args.lat, args.lon
        ))
        .header("user-agent", &args.user_agent)
        .send()
        .context("failed to request point")?
        .json()
        .context("failed to decode point")?;

    let forecast_res: GeoJson<GridpointForecast> = client
        .get(point_res.properties.forecast)
        .header("user-agent", &args.user_agent)
        .send()
        .context("failed to request forecast")?
        .json()
        .context("failed to decode forecast")?;

    for period in forecast_res.properties.periods.iter().take(3) {
        println!("{}\n  {}\n", period.name, period.detailed_forecast);
    }

    Ok(())
}

// https://www.weather.gov/documentation/services-web-api

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeoJson<T> {
    properties: T,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Point {
    forecast: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct GridpointForecast {
    periods: Vec<GridpointForecastPeriod>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct GridpointForecastPeriod {
    name: String,
    detailed_forecast: String,
}
