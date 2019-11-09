use rppal::gpio::Gpio;
use rppal::gpio::InputPin;
use rppal::gpio::Trigger;
use rppal::gpio::Level;

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

use std::{thread, time};

mod pinout;
use pinout::*;

fn main() {
    let gpio = Gpio::new().expect("A new GPIO should have been created");
    let maybe_input_pin_1 = gpio.get(RPI_GPIO_INT_PIN).expect("A new interrupt pin should be created");

    let mut i2c_device_1 = LinuxI2CDevice::new("/dev/i2c-1", pinout::I2C_EXPANDER_1).expect("A new i2c device should have been created");

    initialize_i2c_device(&mut i2c_device_1).expect("An i2c should have been initialized");

    let mut input_pin = maybe_input_pin_1.into_input_pulldown();

    input_pin.set_async_interrupt(Trigger::RisingEdge, move |level: Level|{
        let buttons = read_i2c(&mut i2c_device_1, pinout::INTFA).expect("the buttons should have been read");
        println!("this is fucking working");
        for x in 0..8{
            let bool_test = buttons & (buttons << x) != 0;
            println!("{}",bool_test);
        }
        //thread::sleep(time::Duration::from_secs(1));
    });

    loop{   
    }

    //loop{
    //    if input_pin_state.is_high(){
    //        read_i2c(&mut i2c_device_1, pinout::INTFA);
    //    }
    //}
}

fn initialize_i2c_device(dev: &mut LinuxI2CDevice) -> Result<(), LinuxI2CError>{
    dev.smbus_write_byte_data(pinout::IODIRB, 0x00)?;
    dev.smbus_write_byte_data(pinout::DEFVALA, 0x00)?;
    dev.smbus_write_byte_data(pinout::DEFVALB, 0xff)?;
    dev.smbus_write_byte_data(pinout::INTCONA, 0xff)?;
    dev.smbus_write_byte_data(pinout::IOCON, 0x02)?;
    dev.smbus_write_byte_data(pinout::GPINTAEN, 0xff)?;
    Ok(())
}

fn read_i2c(dev: &mut LinuxI2CDevice, register: u8) -> Result<u8, LinuxI2CError>{
    let pin_to_read = dev.smbus_read_byte_data(register)?;
    dev.smbus_write_byte_data(pinout::OLATB, !pin_to_read)?;
    let pin_value = dev.smbus_read_byte_data(pinout::INTCAPA)?;
    println!("{}", pin_to_read);
    println!("a button has been pressed");
    thread::sleep(time::Duration::from_millis(100));
    dev.smbus_write_byte_data(pinout::OLATB, 0xff)?;

    Ok(pin_to_read)
}
