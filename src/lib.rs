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
}
