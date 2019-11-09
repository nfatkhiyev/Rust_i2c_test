use rppal::gpio::Gpio;
use rppal::gpio::InputPin;

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

mod pinout;
use pinout::*;

fn main() {
    let maybe_input_pin_1 = initialize_gpio_pin(pinout::RPI_GPIO_INT_PIN).ok();
    let maybe_device = initialize_i2c_device(pinout::I2C_EXPANDER_1).ok();
    
    let mut device = maybe_device.unwrap();
    let input_pin_state = maybe_input_pin_1.unwrap();

    loop{
        if input_pin_state.is_high(){
            read_i2c(&mut device, pinout::GPIO_A);
        }
    }
}

fn initialize_gpio_pin(pin_number: u8) -> Result<InputPin, ()>{
    let mut gpio = Gpio::new().ok();
    let mut maybe_pin = gpio.unwrap().get(pin_number);
    match maybe_pin{
        Ok(pin) => {
            let input_pin = pin.into_input_pulldown();
            Ok(input_pin)
        }

        Err(shit) => {
                eprintln!("{:?}", shit);
                println!("shit went down");
                Err(())
        }

    }
    
}

fn initialize_i2c_device(dev_address: u16) -> Result<LinuxI2CDevice, LinuxI2CError>{
    let mut dev = LinuxI2CDevice::new("/dev/i2c-1", dev_address)?;
    dev.smbus_write_byte_data(pinout::IODIRA, 0xff)?;

    dev.smbus_write_byte_data(pinout::DEFVALA, 0x00)?;
    dev.smbus_write_byte_data(pinout::DEFVALB, 0x00)?;

    dev.smbus_write_byte_data(pinout::INTCONA, 0xff)?;

    dev.smbus_write_byte_data(pinout::GPINTAEN, 0xff)?;

    Ok(dev)
}

fn read_i2c(dev: &mut LinuxI2CDevice, register: u8) -> Result<(), LinuxI2CError>{
    //let mut buf: [u8; 13] = [0; 13];
    let mut state_bool: bool = false;
    match dev.smbus_read_byte_data(register){
        Ok(state) => {
            match state {
                1 => state_bool = true,
                0 => state_bool = false,
                _ => {
                        println!("shit went down and also {}", state);
                }
            }
        } 
        Err(shit) => {
            eprintln!("{:?}", shit);
            println!("shit went down");
        }
    }
    println!("{}", state_bool);

    Ok(())
}
