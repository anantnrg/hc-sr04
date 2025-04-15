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

    pub fn dist(&mut self) -> Result<u32, ()> {
        self.trig.set_low().map_err(|_| ())?;
        self.delay.delay_us(2);
        self.trig.set_high().map_err(|_| ())?;
        self.delay.delay_us(10);
        self.trig.set_low().map_err(|_| ())?;

        let mut timeout = 30_000;

        while self.echo.is_low().map_err(|_| ())? {
            timeout -= 1;
            if timeout == 0 {
                return Err(());
            }
        }
        Ok(0)
    }
}
