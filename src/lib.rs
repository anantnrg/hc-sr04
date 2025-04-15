#![no_std]

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin};

pub struct HCSR04<Trig, Echo, Delay> {
    pub trig: Trig,
    pub echo: Echo,
    pub delay: Delay,
}

impl<Trig, Echo, Delay> HCSR04<Trig, Echo, Delay>
where
    Trig: OutputPin,
    Echo: InputPin,
    Delay: DelayNs,
{
    /// Create a new HC-SR04 instance with `trig` and `echo` pins, and a delay provider
    pub fn new(trig: Trig, echo: Echo, delay: Delay) -> Self {
        Self { trig, echo, delay }
    }

    pub fn dist(&mut self) -> Result<u32, Error> {
        self.trig.set_low().map_err(|_| Error::Gpio)?;
        self.delay.delay_us(2);
        self.trig.set_high().map_err(|_| Error::Gpio)?;
        self.delay.delay_us(10);
        self.trig.set_low().map_err(|_| Error::Gpio)?;

        let mut timeout = 30_000;

        while self.echo.is_low().map_err(|_| Error::Gpio)? {
            timeout -= 1;
            if timeout == 0 {
                return Err(Error::Timeout);
            }
        }

        let mut count = 0;

        while self.echo.is_high().map_err(|_| Error::Gpio)? {
            count += 1;
            if count > 30_000 {
                return Err(Error::Timeout);
            }
        }
        Ok(count / 58)
    }
}

/// Errors that can happen during a measurement
#[derive(Debug)]
pub enum Error {
    Timeout,
    Gpio,
}
