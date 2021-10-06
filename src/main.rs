// Rust implementation of ideas from this article:
// https://www.instructables.com/Control-a-Cooling-Fan-on-a-Raspberry-Pi-3/
use rppal::pwm::{Channel, Polarity, Pwm};
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::Read;
use std::error::Error;

fn get_temperature(path: &str) -> f64 {
    let mut file =
        File::open(path).unwrap_or_else(|_| panic!("Could not open temperature file '{}'", path));
    let mut buf = String::with_capacity(6);
    file.read_to_string(&mut buf)
        .expect("Failed to read temperature file");
    let temp_int: i32 = buf
        .strip_suffix('\n')
        .unwrap_or(&buf)
        .parse()
        .expect("Temperature is not a 32-bit integer");
    temp_int as f64 / 1000.0
}

fn main() -> Result<(), Box<dyn Error>> {
    let pwm = Pwm::with_frequency(
        Channel::Pwm0,
        25.0,
        0.0,
        Polarity::Normal,
        true,
    )?;

    // Sleep for 1500 ms (1.5 s)
    thread::sleep(Duration::from_millis(1500));

    let temperature: f64 = get_temperature("/sys/class/thermal/thermal_zone0/temp");

    println!("Current temperature is {}", temperature);

    Ok(())
}
