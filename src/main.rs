mod sensors;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Will panic if args are bad.
    let args = clap::App::new("env-senso-rs")
        .version("0.1")
        .author("Toby Hutton <toby@grusly.com>")
        .about("Read from environment sensors and send to iotplotter.com.")
        .arg(
            clap::Arg::new("feed")
                .short('f')
                .long("feed")
                .value_name("STRING")
                .about("The iotplotter.com feed ID for the URL.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            clap::Arg::new("key")
                .short('k')
                .long("key")
                .value_name("STRING")
                .about("The secret API auth key for the feed.")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    // .unwrap() is safe since both args are .required(true).
    let key = args.value_of("feed").unwrap();
    let feed = args.value_of("key").unwrap();

    let readings: sensors::Readings = sensors::read_sensors()?;

    let json_data = json::object! {
        data: {
            temperature:  [{ value: readings.temperature  }],
            humidity:     [{ value: readings.humidity     }],
            air_pressure: [{ value: readings.air_pressure }],
            gas:          [{ value: readings.gas          }],
        }
    };

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    let resp = client
        .post(&("http://iotplotter.com/api/v2/feed/".to_owned() + feed))
        .header("api-key", key)
        .body(json_data.dump())
        .send()?;

    if resp.status().is_success() {
        println!("{}", resp.text()?);
    } else {
        println!("{:?}", resp.status());
    }

    Ok(())
}
