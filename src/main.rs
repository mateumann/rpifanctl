// Rust implementation of ideas from this article:
// https://www.instructables.com/Control-a-Cooling-Fan-on-a-Raspberry-Pi-3/
use clap::{value_t, App, AppSettings, Arg, SubCommand};
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

fn calculate_duty_cycle(
    temperature_steps: Vec<f64>,
    fan_speed_steps: Vec<f64>,
    temperature: f64,
) -> f64 {
    if temperature < temperature_steps[0] {
        return 0.0;
    }
    for i in 0..temperature_steps.len() - 1 {
        if temperature_steps[i] <= temperature && temperature < temperature_steps[i+1] {
            return (fan_speed_steps[i+1] - fan_speed_steps[i]) /
            (temperature_steps[i+1] - temperature_steps[i]) *
            (temperature - temperature_steps[i]) +
            fan_speed_steps[i];
        }
    }
    return *fan_speed_steps.last().unwrap();
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Raspberry Pi Fan Control")
        .version("0.1")
        .about("Set Raspberry Pi Fan based on CPU temperature")
        .arg(
            Arg::with_name("pwm")
                .short("p")
                .long("pwm")
                .value_name("PWM CHANNEL")
                .help("Select PWM Channel either 0 or 1")
                .default_value("0")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("poll_frequency")
                .short("d")
                .long("delay")
                .value_name("DELAY IN SECONDS")
                .help("How often temperature should be polled")
                .default_value("1.5")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("thermal_zone")
                .short("z")
                .long("thermal-zone")
                .value_name("THERMAL ZONE")
                .help("Thermal zone to probe")
                .default_value("0")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("temperature_steps")
                .short("t")
                .long("temperature-steps")
                .value_name("TEMPERATURE STEPS")
                .help("Temperature steps in °C")
                .default_value("50,60,80")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("fan_speed_steps")
                .short("f")
                .long("fan-steps")
                .value_name("FAN SPEED STEPS")
                .help("Fan speed steps in percent")
                .default_value("30,70,100")
                .takes_value(true),
        )
        .get_matches();

    let pwm = value_t!(matches.value_of("pwm"), u8).unwrap_or(0);
    let poll_frequency = value_t!(matches.value_of("poll_frequency"), f64).unwrap_or_default();
    let thermal_zone = value_t!(matches.value_of("thermal_zone"), u8).unwrap_or_default();
    let temperature_steps_raw = matches.value_of("temperature_steps").unwrap_or_default();
    let fan_speed_steps_raw = matches.value_of("fan_speed_steps").unwrap_or_default();
    let temperature_steps: Vec<f64> = temperature_steps_raw
        .split(|c| c == ',' || c == ' ')
        .map(|x| x.parse().unwrap())
        .collect();
    let fan_speed_steps: Vec<f64> = fan_speed_steps_raw
        .split(|c| c == ',' || c == ' ')
        .map(|x| x.parse().unwrap())
        .collect();

    // let pwm = Pwm::with_frequency(Channel::Pwm0, 25.0, 0.0, Polarity::Normal, true)?;

    // Sleep for 1500 ms (1.5 s)
    // thread::sleep(Duration::from_millis(1500));

    let temperature: f64 = get_temperature(format!("/sys/class/thermal/thermal_zone{}/temp", thermal_zone).as_str());
    let duty_cycle: f64 = calculate_duty_cycle(temperature_steps, fan_speed_steps, temperature);

    // pwm.set_duty_cycle(duty_cycle)?;

    println!(
        "Current pwm={}, poll_frequency={}, temperature={}, duty_cycle={}",
        pwm, poll_frequency, temperature, duty_cycle
    );

    Ok(())
}
