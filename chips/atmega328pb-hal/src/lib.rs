#![no_std]

pub use avr_device::atmega328pb as pac;

/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use avr_device::entry;

pub use avr_hal_generic::{clock, delay};

pub mod port;
pub mod adc;
pub mod pwm;
pub mod wdt;

pub mod prelude {
    pub use avr_hal_generic::prelude::*;
    pub use crate::port::PortExt as _;
}

/// I2C Bus
pub mod i2c {
    use crate::port::{portc, porte};
    pub use avr_hal_generic::i2c::*;

    avr_hal_generic::impl_twi_i2c! {
        /// I2C based on ATmega328P's TWI peripheral
        pub struct I2c0 {
            peripheral: crate::pac::TWI0,
            pins: {
                sda: portc::PC4,
                scl: portc::PC5,
            },
            registers: {
                control: twcr {
                    enable: twen,
                    ack: twea,
                    int: twint,
                    start: twsta,
                    stop: twsto,
                },
                status: twsr {
                    prescaler: twps,
                    status: tws,
                },
                bitrate: twbr,
                data: twdr,
            },
        }
    }

    avr_hal_generic::impl_twi_i2c! {
        /// I2C based on ATmega328P's TWI peripheral
        pub struct I2c1 {
            peripheral: crate::pac::TWI1,
            pins: {
                sda: porte::PE0,
                scl: porte::PE1,
            },
            registers: {
                control: twcr {
                    enable: twen,
                    ack: twea,
                    int: twint,
                    start: twsta,
                    stop: twsto,
                },
                status: twsr {
                    prescaler: twps,
                    status: tws,
                },
                bitrate: twbr,
                data: twdr,
            },
        }
    }

}

pub mod spi {
    //! Implementation of the Rust Embedded-HAL SPI FullDuplex trait for AVR.
    //!
    //! The interface can be instantiated with the `new` method, and used directly
    //! or passed into a driver.  Example usage:
    //!
    //! ```
    //! pins.d10.into_output(&mut pins.ddr);// SS must be set to output mode
    //! // create SPI interface
    //! let mut spi = Spi0::new(
    //!     dp.SPI,// SPI peripheral
    //!     pins.d13.into_output(&mut pins.ddr),// SCLK
    //!     pins.d11.into_output(&mut pins.ddr),// MOSI output pin
    //!     pins.d12.into_pull_up_input(&mut pins.ddr),// MISO input pin
    //!     Settings::default(),
    //! );
    //!
    //! // Send a byte
    //! let sent = 0b10101010;
    //! spi.send(sent).unwrap();
    //! let response = spi.read().unwrap();
    //! ```
    //! In the example above, all of the settings are left at the default.  You can
    //! also instantiate a Settings object with the other options available.

    use crate::port::{portb, portc, porte};
    pub use avr_hal_generic::spi::*;

    avr_hal_generic::impl2_spi! {
        pub struct Spi0 {
            peripheral: crate::pac::SPI0,
            pins: {
                sclk: portb::PB5,
                mosi: portb::PB3,
                miso: portb::PB4,
            },
        }
    }

    avr_hal_generic::impl2_spi! {
        pub struct Spi1 {
            peripheral: crate::pac::SPI1,
            pins: {
                sclk: portc::PC1,
                mosi: porte::PE3,
                miso: portc::PC0,
            },
        }
    }

}

/// Serial interface using USART
pub mod usart {
    use crate::port::{portb, portd};
    pub use avr_hal_generic::serial::*;

    avr_hal_generic::impl_usart! {
        /// Serial interface based on ATmega328PB's USART0 peripheral
        ///
        /// Maximum baudrate seems to be 57600
        pub struct Usart0 {
            peripheral: crate::pac::USART0,
            pins: {
                rx: portd::PD0,
                tx: portd::PD1,
            },
            register_suffix: 0,
        }
    }

    avr_hal_generic::impl_usart! {
        /// Serial interface based on ATmega328PB's USART0 peripheral
        ///
        /// Maximum baudrate seems to be 57600
        pub struct Usart1 {
            peripheral: crate::pac::USART1,
            pins: {
                rx: portb::PB4,
                tx: portb::PB3,
            },
            register_suffix: 1,
        }
    }

}
