#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::{delay::Delay, pac, prelude::*};

use hc_sr04::{Error, HCSR04};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(8.mhz()).freeze(&mut flash.acr);
    let mut delay = Delay::new(cp.SYST, clocks);

    let mut gpioa = dp.GPIOA.split();

    let trig = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    let echo = gpioa.pa1.into_floating_input(&mut gpioa.crl);

    let mut sensor = HCSR04::new(trig, echo, delay);

    loop {
        match sensor.dist() {
            Ok(distance_cm) => info!("Distance: {} cm", distance_cm),
            Err(Error::Timeout) => warn!("Sensor timeout"),
            Err(Error::Gpio) => error!("GPIO error"),
        }

        cortex_m::asm::delay(8_000_000);
    }
}
