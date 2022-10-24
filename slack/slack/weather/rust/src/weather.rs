use wasmedge_bindgen_macro::*;

use http_req::request;
use serde_json::Value;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

pub fn _run(s: String) -> Result<String, String> {
    if s.trim().is_empty() {
        return Err("Wrong location submitted".to_string());
    }

    get_weather(&s)
}

pub fn get_weather(city: &String) -> Result<String, String> {
    let mut writer = Vec::new();
    let API_key = "d7708b2a44c24775d4845c07a994e7a0";
    let query_str = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={city}&units=metric&appid={API_key}"
    );

    let _ = request::get(query_str, &mut writer).unwrap();

    let payload: Value = serde_json::from_str(&String::from_utf8(writer).unwrap()).unwrap();
    match payload.get("cod").unwrap() == "404" {
        true => Err("Please check if you've typed the name of your city correctly".to_string()),
        false => Ok(make_output(payload)),
    }
}

pub fn make_output(payload: Value) -> String {
    let main_weather = payload.get("weather").unwrap()[0]["main"]
        .as_str()
        .unwrap()
        .to_string();
    let temp = payload.get("main").unwrap();
    let low = temp.get("temp_min").unwrap().as_f64().unwrap() as i32;
    let high = temp.get("temp_max").unwrap().as_f64().unwrap() as i32;
    let wind_speed = payload
        .get("wind")
        .unwrap()
        .get("speed")
        .unwrap()
        .as_f64()
        .unwrap() as i32;

    let centigrade = char::from_u32(0x00002103).unwrap();
    format!(
        r#"Today: {},
Low temperature: {} {centigrade},
High temperature: {} {centigrade},
Wind Speed: {} km/h"#,
        main_weather, low, high, wind_speed
    )
}

