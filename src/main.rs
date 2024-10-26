// use dht_mmap_rust::{Dht, DhtType};
// use serde_json::json;

// fn main() {
//     // The sensor is a DHT11 connected on pin 23
//     let mut dht = Dht::new(DhtType::Dht11, 2).unwrap();

//     // Important: DHT sensor reads fail sometimes. In an actual program, if a read fails you should retry multiple times until
//     // the read succeeds.
//     // For more information, see documentation on `read()`
//     let reading = dht.read().unwrap();

//     let temperature_c = reading.temperature();
//     let temperature_f = temperature_c * 9.0 / 5.0 + 32.0;

//     println!(
//         "Temperature: \n\t{:.2} C\n\t{:.2} F\nHumidity:\n\t {}%",
//         temperature_c,
//         temperature_f,
//         reading.humidity()
//     );

//     let response = json!({
//         "temperature": reading.temperature(),
//         "humidity": reading.humidity()
//     });

//     println!("{}", response);
// }

use clap::{Command, Arg};
use dht_mmap_rust::{Dht, DhtType};
use serde_json::json;

fn main() {
    // Initialize the CLI app
    let matches = Command::new("DHT11 CLI")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Reads temperature and humidity from a DHT11 sensor")
        .arg(
            Arg::new("humidity")
                .short('h')
                .long("humidity")
                .help("Prints the humidity"),
        )
        .arg(
            Arg::new("celsius")
                .short('c')
                .long("celsius")
                .help("Prints the temperature in Celsius"),
        )
        .arg(
            Arg::new("fahrenheit")
                .short('f')
                .long("fahrenheit")
                .help("Prints the temperature in Fahrenheit"),
        )
        .get_matches();

    // Initialize the DHT11 sensor
    let mut dht = Dht::new(DhtType::Dht11, 2).unwrap();
    let reading = dht.read().unwrap();

    // Handle CLI options
    if matches.contains_id("humidity") {
        println!("Humidity: {}%", reading.humidity());
    }

    if matches.contains_id("celsius") {
        println!("Temperature: {:.2} C", reading.temperature());
    }

    if matches.contains_id("fahrenheit") {
        let temperature_c = reading.temperature();
        let temperature_f = temperature_c * 9.0 / 5.0 + 32.0;
        println!("Temperature: {:.2} F", temperature_f);
    }

    // Print JSON response if no specific option is provided
    if !matches.contains_id("humidity") && !matches.contains_id("celsius") && !matches.contains_id("fahrenheit") {
        let response = json!({
            "temperature_c": reading.temperature(),
            "temperature_f": reading.temperature() * 9.0 / 5.0 + 32.0,
            "humidity": reading.humidity()
        });
        println!("{}", response);
    }
}