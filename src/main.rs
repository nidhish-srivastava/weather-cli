use std::io; // Importing the standard input/output library for user input

use serde::Deserialize; // Importing serde for JSON deserialization

use colored::*; // Importing colored crate for text coloring

// Struct to deserialise the JSON response from the openweather map api
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>, // contains the weather information,
    name: String,
    wind: Wind,
    main: Main,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

// Function  to get the weather info from OpenWeatherMap API
fn get_weather_info(
    city: &str,
    country_code: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    // for handling errors returning the result enum which has OK and error
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );
    // Sending a blocking GET request to the API endpoint
    let response = reqwest::blocking::get(&url)?; // ?: is used to propagate errors in funcions that return a Result or Option,it unwraps and returns a value(in case of Ok(value) or Some(value))
                                                  // Parsing the JSON response into WeatherResponse struct
    let response_json = response.json::<WeatherResponse>()?;
    Ok(response_json) // Returning the deserialized response
}

fn display_weather_info(response: &WeatherResponse) {
    // Extracting weather information from the response
    let description = &response.weather[0].description;
    let temperature = response.main.temp;
    let humidity = response.main.humidity;
    let pressure = response.main.pressure;
    let wind_speed = response.wind.speed;

    // Formatting weather information into a string
    let weather_text = format!(
        "Weather in {}: {} {}
        > Temperature: {:.1}°C, 
        > Humidity: {:.1}%, 
        > Pressure: {:.1} hPa, 
        > Wind Speed: {:.1} m/s",
        response.name,
        description,
        get_temperature_emoji(temperature),
        temperature,
        humidity,
        pressure,
        wind_speed,
    );

    // Coloring the weather text based on weather conditions
    let weather_text_colored = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => {
            weather_text.dimmed()
        }
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };

    // Printing the colored weather information
    println!("{}", weather_text_colored);
}

// Function to get emoji based on temperature
fn get_temperature_emoji(temperature: f64) -> &'static str {
    // static lifetime means data will live for the entire duration of the program
    if temperature < 0.0 {
        "❄️"
    } else if temperature >= 0.0 && temperature < 10.0 {
        "☁️"
    } else if temperature >= 10.0 && temperature < 20.0 {
        "⛅"
    } else if temperature >= 20.0 && temperature < 30.0 {
        "🌤️"
    } else {
        "🔥"
    }
}

fn main() {
    println!("{}", "Welcome to Weather Station!".bright_yellow()); // Displaying welcome message

    loop {
        println!("{}", "Please enter the name of the city:".bright_green()); // Prompting user to enter city name
        let mut city = String::new();
        io::stdin()
            .read_line(&mut city)
            .expect("Failed to read input"); // Reading user input for city name
        let city = city.trim();
        println!(
            "{}",
            "Please enter the country code (e.g., US for United States):".bright_green()
        ); // Prompting user to enter country code
        let mut country_code = String::new();

        io::stdin()
            .read_line(&mut country_code)
            .expect("Failed to read input"); // Reading user input for country code
        let country_code = country_code.trim();
        let api_key = "2cbf657f635789a8bbb2e8c8233cdf6e";

        match get_weather_info(&city, &country_code, api_key) {
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(error) => {
                eprintln!("Error : {}", error);
            }
        }
        println!(
            "{}",
            "Do you want to search for weather in another city?(yes,no) : "
        );
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim().to_lowercase();
        if input != "yes" {
            println!("Thank you for using our software");
            break;
        }
    }
}
