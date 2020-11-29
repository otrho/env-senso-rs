# Environment Sensors Interface In Rust

This project is a small utilty to interface with the [Metriful Indoor Sensor Board](https://www.metriful.com/) from a Raspberry
PI SBC and to report its readings to an online dashboard at [IoTPlotter](https://iotplotter.com).

Other dashboards may be supported in the future.  I chose IoTPlotter to begin with since it's very
simple, open and free.

## Building

`env-senso-rs` is designed to be built on the RPi itself, which is slow and can download **100s of MB**
in library dependencies, but it's probably simpler than trying to get it built using a cross
compiler.

Rust can be installed on an RPi using [rustup](https://rustup.rs) as usual, and then `cargo build` will work as
expected.

## Running

The utility assumes the sensor was connected to the RPi GPIO as described in the Metriful
instructions.

It also requires an IoTPlotter feed ID and authorisation key to allow sending the readings online.

```console
$ cargo run -- -h
env-senso-rs 0.1
Toby Hutton <toby@grusly.com>
Read from environment sensors and send to iotplotter.com.

USAGE:
    env-senso-rs --feed <STRING> --key <STRING>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --feed <STRING>    The iotplotter.com feed ID for the URL.
    -k, --key <STRING>     The secret API auth key for the feed.
```

