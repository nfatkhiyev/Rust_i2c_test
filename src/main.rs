use rppal::gpio::Gpio;
use rppal::gpio::InputPin;

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

mod pinout;
use pinout::*;

fn main() {
    let gpio = Gpio::new().expect("A new GPIO should have been created");
    let maybe_input_pin_1 = gpio.get(RPI_GPIO_INT_PIN).expect("A new interrupt pin should be created");

    let mut i2c_device_1 = LinuxI2CDevice::new("/dev/i2c-1", pinout::I2C_EXPANDER_1).expect("A new i2c device should have been created");

    initialize_i2c_device(&mut i2c_device_1).expect("An i2c should have been initialized");

    let input_pin_state = maybe_input_pin_1.into_input_pulldown();

    loop{
        if input_pin_state.is_high(){
            read_i2c(&mut i2c_device_1, pinout::GPIO_A);
        }
    }
}

fn initialize_i2c_device(dev: &mut LinuxI2CDevice) -> Result<(), LinuxI2CError>{
    dev.smbus_write_byte_data(pinout::IODIRA, 0xff)?;
    dev.smbus_write_byte_data(pinout::DEFVALA, 0x00)?;
    dev.smbus_write_byte_data(pinout::DEFVALB, 0x00)?;
    dev.smbus_write_byte_data(pinout::INTCONA, 0xff)?;
    dev.smbus_write_byte_data(pinout::GPINTAEN, 0xff)?;
    Ok(())
}

fn read_i2c(dev: &mut LinuxI2CDevice, register: u8) -> Result<(), LinuxI2CError>{
    //let mut buf: [u8; 13] = [0; 13];
    let pin_to_read = dev.smbus_read_byte_data(register)?;
    println!("{}", pin_to_read);
    println!("a button has been pressed");

    Ok(())
}
