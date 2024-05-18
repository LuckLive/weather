mod env;
use reqwest::blocking::get;
use serde_json::{ Value, from_str };

fn main() {
    let city: &str    = env::CITY;
    let api_key: &str = env::API_KEY;
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&APPID={}", city, api_key);
    // println!("URL: {}", url);
    // println!("\n");

    let response = get(url).unwrap();
    let response_text = response.text().unwrap();
    let json: Value = from_str(&response_text).expect("JSON was not well-formatted");

    let temp_kelvin = json["main"]["temp"].as_f64().unwrap();
    let temp_celsius = temp_kelvin - 273.15;
    let humidity = json["main"]["humidity"].as_f64().unwrap();
    let pressure = json["main"]["pressure"].as_f64().unwrap();
    let wind_speed = json["wind"]["speed"].as_f64().unwrap();

    println!("Wetter in {}:", city);
    println!("  Temperatur : {:.2}°C", temp_celsius);
    println!("  Luftfeuchte: {}%", humidity);
    println!("  Luftftdruck: {}hPa", pressure);
    println!("  Wind       : {:.2}km/h", wind_speed * 3.6);
}
