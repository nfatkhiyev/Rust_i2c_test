extern crate i2cdev;

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

const I2C_EXPANDER_1: u16 = 0x20;
const GPIO_B_0: u8 = 0x13;


fn main() {
    read_i2c(GPIO_B_0).ok();
}

fn read_i2c(register: u8) -> Result<(), LinuxI2CError>{
    //let mut buf: [u8; 13] = [0; 13];
    let mut dev = LinuxI2CDevice::new("/dev/i2c-1", I2C_EXPANDER_1)?;
    loop{
        let state = dev.smbus_read_byte_data(register).unwrap();
        println!("{}", state);
    }
}
