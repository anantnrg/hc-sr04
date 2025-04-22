#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_probe as _;
use rtt_target::rprintln;
use stm32f1xx_hal::{pac, prelude::*};

use cortex_m::Peripherals as CorePeripherals;
use hc_sr04::{Error, HCSR04};

#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(72_000.kHz()).freeze(&mut flash.acr);

    let syst = cp.SYST;
    let dcb = cp.DCB;
    let dwt = cp.DWT;

    let delay = cortex_m::delay::Delay::new(syst, clocks.sysclk().to_Hz());

    let mut gpiob = dp.GPIOB.split();
    let trig = gpiob.pb7.into_push_pull_output(&mut gpiob.crl);
    let echo = gpiob.pb6.into_floating_input(&mut gpiob.crl);

    let timer = DwtTimer::new(dcb, dwt, 72);

    let mut sensor = HCSR04::new(trig, echo, delay, timer);

    loop {
        match sensor.dist() {
            Ok(distance_cm) => rprintln!("Distance: {:.2} cm", distance_cm),
            Err(Error::Timeout) => rprintln!("Timeout"),
            Err(Error::Gpio) => rprintln!("GPIO Err"),
        }

        cortex_m::asm::delay(8_000_000); // delay between reads
    }
}

pub struct DwtTimer {
    cpu_mhz: u32,
}

impl DwtTimer {
    pub fn new(
        mut dcb: cortex_m::peripheral::DCB,
        mut dwt: cortex_m::peripheral::DWT,
        cpu_mhz: u32,
    ) -> Self {
        dcb.enable_trace();
        unsafe {
            dwt.lar.write(0xC5ACCE55);
        }
        dwt.enable_cycle_counter();
        Self { cpu_mhz }
    }
}

impl hc_sr04::Timer for DwtTimer {
    fn now(&self) -> u32 {
        cortex_m::peripheral::DWT::cycle_count() / self.cpu_mhz
    }
}
