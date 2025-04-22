#![no_std]

use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::digital::v2::{InputPin, OutputPin};

const SPEED_OF_SOUND_CM_PER_US: f32 = 0.0343;

pub struct HCSR04<Trig, Echo, Delay, Tmr> {
    pub trig: Trig,
    pub echo: Echo,
    pub delay: Delay,
    pub timer: Tmr,
}

impl<Trig, Echo, Delay, Tmr> HCSR04<Trig, Echo, Delay, Tmr>
where
    Trig: OutputPin,
    Echo: InputPin,
    Delay: DelayUs<u16>,
    Tmr: Timer,
{
    pub fn new(trig: Trig, echo: Echo, delay: Delay, timer: Tmr) -> Self {
        Self {
            trig,
            echo,
            delay,
            timer,
        }
    }

    pub fn dist(&mut self) -> Result<f32, Error> {
        // Fire 10us trigger pulse
        self.trig.set_low().map_err(|_| Error::Gpio)?;
        self.delay.delay_us(2_000);
        self.trig.set_high().map_err(|_| Error::Gpio)?;
        self.delay.delay_us(10);
        self.trig.set_low().map_err(|_| Error::Gpio)?;

        // Wait for echo to go high
        let start_wait = self.timer.now();
        while self.echo.is_low().map_err(|_| Error::Gpio)? {
            if self.timer.now().wrapping_sub(start_wait) > 30_000 {
                return Err(Error::Timeout);
            }
        }

        // Measure pulse duration
        let start = self.timer.now();
        while self.echo.is_high().map_err(|_| Error::Gpio)? {
            if self.timer.now().wrapping_sub(start) > 30_000 {
                return Err(Error::Timeout);
            }
        }
        let pulse_duration_us = self.timer.now().wrapping_sub(start);

        let distance_cm = (pulse_duration_us as f32 * SPEED_OF_SOUND_CM_PER_US) / 2.0;
        Ok(distance_cm)
    }
}

#[derive(Debug)]
pub enum Error {
    Timeout,
    Gpio,
}

pub trait Timer {
    /// Get the current time in microseconds
    fn now(&self) -> u32;
}
