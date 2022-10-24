use http_req::request;
use serde::Deserialize;
use wasmedge_bindgen_macro::*;

static API_KEY: &'static str = "d7708b2a44c24775d4845c07a994e7a0";

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

pub fn _run(s: String) -> Result<String, String> {
    let city = match s.split_once(" ") {
        Some((command, city)) if command.eq("/w") || command.eq("/weather") => city,
        _ => return Ok(String::new()),
    }
    .trim();

    get_weather(city).map(|w| {
        format!(
            "Today: {},
Low temperature: {} °C,
High temperature: {} °C,
Wind Speed: {} km/h",
            w.weather
                .first()
                .unwrap_or(&Weather {
                    main: "Unknown".to_string()
                })
                .main,
            w.main.temp_min as i32,
            w.main.temp_max as i32,
            w.wind.speed as i32
        )
    })
}

#[derive(Deserialize)]
struct ApiResult {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
}

#[derive(Deserialize)]
struct Weather {
    main: String,
}

#[derive(Deserialize)]
struct Main {
    temp_max: f64,
    temp_min: f64,
}

#[derive(Deserialize)]
struct Wind {
    speed: f64,
}

fn get_weather(city: &str) -> Result<ApiResult, String> {
    let mut writer = Vec::new();
    let query_str = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={city}&units=metric&appid={API_KEY}"
    );

    request::get(query_str, &mut writer)
        .map_err(|e| e.to_string())
        .and_then(|_| {
            serde_json::from_slice::<ApiResult>(&writer).map_err(|_| {
                "Please check if you've typed the name of your city correctly".to_string()
            })
        })
}
