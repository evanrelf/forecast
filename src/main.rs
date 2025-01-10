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

    let forecast_text = &forecast_res
        .properties
        .periods
        .first()
        .context("forecast has no periods")?
        .detailed_forecast;

    println!("{forecast_text}");

    Ok(())
}

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
    detailed_forecast: String,
}
