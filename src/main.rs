use anyhow::Context;
use clap::Parser as _;
use std::fmt::Display;

#[derive(clap::Parser, Debug)]
struct Args {
    /// Latitude
    #[clap(long, env = "FORECAST_LATITUDE", allow_hyphen_values = true)]
    lat: f32,

    /// Longitude
    #[clap(long, env = "FORECAST_LONGITUDE", allow_hyphen_values = true)]
    lon: f32,

    #[clap(long, env = "FORECAST_UNITS", default_value_t = Units::Us)]
    units: Units,

    #[clap(long, env = "FORECAST_USER_AGENT")]
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
        .context("request for point failed")?
        .json()
        .context("could not decode point")?;

    let forecast_res: GeoJson<GridpointForecast> = client
        .get(format!(
            "{}?units={}",
            point_res.properties.forecast, args.units
        ))
        .header("user-agent", &args.user_agent)
        .send()
        .context("request for forecast failed")?
        .json()
        .context("could not decode forecast")?;

    for period in forecast_res.properties.periods.iter().take(3) {
        println!(
            "{}: {}\n  {}\n",
            period.name, period.short_forecast, period.detailed_forecast
        );
    }

    Ok(())
}

// https://www.weather.gov/documentation/services-web-api

#[derive(clap::ValueEnum, Clone, Copy, Debug, serde::Serialize)]
enum Units {
    /// Imperial (ºF, mph)
    Us,
    /// Metric (ºC, km/h)
    Si,
}

impl Display for Units {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Us => write!(f, "us"),
            Self::Si => write!(f, "si"),
        }
    }
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
    name: String,
    short_forecast: String,
    detailed_forecast: String,
}
