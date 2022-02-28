use serde_derive::{Deserialize, Serialize};
use reqwest::Url;
use exitfailure::ExitFailure;

#[derive(Serialize, Deserialize, Debug)]
pub struct W {
    coord: Coord,
    weather: Weather,
    base: String,
    pub(crate) main: Main,
}

#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    details: Details
}

#[derive(Serialize, Deserialize, Debug)]
struct Details {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: i32,
    pub humidity: i32,
}

impl W {
    pub async fn get(city: &String) -> Result<Self, ExitFailure> {
        let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid=4845f22236e074cdac59ae174aa580a3", city);
        let url = Url::parse(&*url)?;
        let resp = reqwest::get(url).await?.json::<W>().await?;
        Ok(resp)
    }
}
