use dht_mmap_rust::{Dht, DhtType};

fn main() {
    // The sensor is a DHT11 connected on pin 23
    let mut dht = Dht::new(DhtType::Dht11, 2).unwrap();

    // Important: DHT sensor reads fail sometimes. In an actual program, if a read fails you should retry multiple times until
    // the read succeeds.
    // For more information, see documentation on `read()`
    let reading = dht.read().unwrap();

    println!(
        "Temperature {} °C, Humidity {}%RH",
        reading.temperature(),
        reading.humidity()
    );
}