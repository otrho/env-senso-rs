use rppal::{gpio, i2c};

const GPIO_READY_PIN: u8 = 17; // RPi Board Pin 11 == BCM GPIO17.

const I2C_ADDR: u16 = 0x71;
const I2C_RESET_CMD: u8 = 0xe2;
const I2C_REQUEST_DATA: u8 = 0xe1;
const I2C_AIR_DATA_READ: u8 = 0x10;
const I2C_AIR_DATA_READ_SIZE: usize = 12;

const TEMP_SIGN_MASK: u8 = 0x80;
const TEMP_VALUE_MASK: u8 = 0x7F;

#[derive(Clone)]
pub struct Readings {
    pub temperature: f32,
    pub humidity: f32,
    pub air_pressure: i32,
    pub gas: i32,
}

pub fn read_sensors() -> Result<Readings, Box<dyn std::error::Error>> {
    // Get the ready pin and wait for it to be low.
    let ready_pin = gpio::Gpio::new()?.get(GPIO_READY_PIN)?.into_input();
    let wait_until_ready = || {
        while !ready_pin.is_low() {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    };
    wait_until_ready();

    // Get the I2C bus.
    let mut i2c_bus = i2c::I2c::with_bus(1)?;
    i2c_bus.set_slave_address(I2C_ADDR)?;

    // Send a reset command.
    i2c_bus.smbus_send_byte(I2C_RESET_CMD)?;
    std::thread::sleep(std::time::Duration::from_millis(10));
    wait_until_ready();

    // Write an on-demand request for air data.
    i2c_bus.smbus_send_byte(I2C_REQUEST_DATA)?;
    std::thread::sleep(std::time::Duration::from_millis(10));
    wait_until_ready();

    // Read the air data values all at once.
    let mut air_data = [0u8; I2C_AIR_DATA_READ_SIZE];
    i2c_bus.block_read(I2C_AIR_DATA_READ, &mut air_data)?;

    // +----+----+----+----+----+----+----+----+----+----+----+----+
    // | 11 | 10 |  9 |  8 |  7 |  6 |  5 |  4 |  3 |  2 |  1 |  0 |
    // +----+----+----+----+----+----+----+----+----+----+----+----+
    // | gas               | humidity| pressure          | temp    |
    // +----+----+----+----+----+----+----+----+----+----+----+----+

    let decode_i32 = |base_idx: usize| {
        let mut value = 0_i32;
        for byte_idx in base_idx..(base_idx + 4) {
            value |= (air_data[byte_idx] as i32) << (8 * (byte_idx - base_idx));
        }
        value
    };

    let mut temperature: f32 = (air_data[0] & TEMP_VALUE_MASK) as f32 + (air_data[1] as f32 / 10.0);
    if air_data[0] & TEMP_SIGN_MASK != 0 {
        temperature = -temperature;
    }
    let air_pressure = decode_i32(2);
    let humidity: f32 = air_data[6] as f32 + (air_data[1] as f32 / 10.0);
    let gas = decode_i32(8);

    Ok(Readings {
        temperature,
        humidity,
        air_pressure,
        gas
    })
}


