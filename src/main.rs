// Rust implementation of ideas from this article:
// https://www.instructables.com/Control-a-Cooling-Fan-on-a-Raspberry-Pi-3/
use clap::{App, Arg, AppSettings, SubCommand, value_t};
use rppal::pwm::{Channel, Polarity, Pwm};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::thread;
use std::time::Duration;

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

fn calculate_duty_cycle(temperature: f64) -> f64 {
    return 0.0;
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Raspberry Pi Fan Control")
                          .version("0.1")
                          .author("Mateusz")
                          .about("Set Raspberry Pi Fan based on CPU temperature")
                          .arg(Arg::with_name("pwm")
                               .short("p")
                               .long("pwm")
                               .value_name("PWM CHANNEL")
                               .help("Select PWM Channel either 0 or 1")
                               .default_value("0")
                               .takes_value(true))
                          .arg(Arg::with_name("poll_frequency")
                               .short("d")
                               .long("delay")
                               .value_name("DELAY IN SECONDS")
                               .help("How often temperature should be polled")
                               .default_value("1.5")
                               .takes_value(true))
                          .arg(Arg::with_name("temperature_steps")
                               .short("t")
                               .long("temperature")
                               .value_name("TEMPERATURE STEPS")
                               .help("Temperature steps in °C")
                               .default_value("50,60,80")
                               .takes_value(true))
                          .arg(Arg::with_name("fan_speed_steps")
                               .short("f")
                               .long("fan")
                               .value_name("FAN SPEED STEPS")
                               .help("Fan speed steps in percent")
                               .default_value("30,70,100")
                               .takes_value(true))
                          .get_matches();

    let pwm = value_t!(matches.value_of("pwm"), u8).unwrap_or(0);
    let poll_frequency = value_t!(matches.value_of("poll_frequency"), f64).unwrap_or_default();
    let temperature_steps = matches.value_of("temperature_steps").unwrap_or_default();
    let fan_speed_steps = matches.value_of("fan_speed_steps").unwrap_or_default();
    let v: Vec<&str> = temperature_steps.split(|c| c == ',' || c == ' ').collect();

    // let pwm = Pwm::with_frequency(Channel::Pwm0, 25.0, 0.0, Polarity::Normal, true)?;

    // Sleep for 1500 ms (1.5 s)
    // thread::sleep(Duration::from_millis(1500));

    let temperature: f64 = get_temperature("/sys/class/thermal/thermal_zone0/temp");
    let duty_cycle: f64 = calculate_duty_cycle(temperature);

    // pwm.set_duty_cycle(duty_cycle)?;

    println!("Current pwm={}, poll_frequency={}, temperature_steps={}, fan_speed_steps={}, v={}, temperature={}, duty_cycle={}", pwm, poll_frequency, temperature_steps, fan_speed_steps, v, temperature, duty_cycle);

    Ok(())
}
