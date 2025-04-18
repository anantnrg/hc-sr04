#![no_std]

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::{InputPin, OutputPin};

/// Speed of sound in cm/us (343 m/s => 0.0343 cm/ns * 1000 = 0.0343 cm/us)
const SPEED_OF_SOUND_CM_PER_US: f32 = 0.0343;

pub struct HCSR04<Trig, Echo, Delay> {
    pub trig: Trig,
    pub echo: Echo,
    pub delay: Delay,
}

impl<Trig, Echo, Delay> HCSR04<Trig, Echo, Delay>
where
    Trig: OutputPin,
    Echo: InputPin,
    Delay: DelayMs<u16>,
{
    /// Create a new HC-SR04 instance with `trig` and `echo` pins, and a delay provider
    pub fn new(trig: Trig, echo: Echo, delay: Delay) -> Self {
        Self { trig, echo, delay }
    }

    pub fn dist(&mut self) -> Result<f32, Error> {
        // Send 10us pulse
        self.trig.set_low().map_err(|_| Error::Gpio)?;
        self.delay.delay_ms(2);
        self.trig.set_high().map_err(|_| Error::Gpio)?;
        self.delay.delay_ms(10);
        self.trig.set_low().map_err(|_| Error::Gpio)?;

        // Wait for echo to go HIGH
        let mut timeout = 30_000;
        while self.echo.is_low().map_err(|_| Error::Gpio)? {
            timeout -= 1;
            if timeout == 0 {
                return Err(Error::Timeout);
            }
        }

        // Count echo HIGH time
        let mut pulse_duration_us = 0u32;
        while self.echo.is_high().map_err(|_| Error::Gpio)? {
            pulse_duration_us += 1;
            self.delay.delay_ms(1);
            if pulse_duration_us > 30_000 {
                return Err(Error::Timeout);
            }
        }

        // Calculate distance: d = (t * v) / 2
        let distance_cm = (pulse_duration_us as f32 * SPEED_OF_SOUND_CM_PER_US) / 2.0;
        Ok(distance_cm)
    }
}

/// Errors that can happen during a measurement
#[derive(Debug)]
pub enum Error {
    Timeout,
    Gpio,
}

#[defmt::timestamp]
fn timestamp() -> u64 {
    // Return a timestamp, maybe from a timer or cycle counter
    0 // TEMP: replace this with actual time if needed
}
