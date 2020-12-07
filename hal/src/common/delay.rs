//! Delays

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::SYST;

use crate::clock::GenericClockController;
use crate::time::Hertz;

use embedded_hal_alpha::blocking::delay::{DelayMs as DelayMsAlpha, DelayUs as DelayUsAlpha};
use hal::blocking::delay::{DelayMs, DelayUs};

/// System timer (SysTick) as a delay provider
pub struct Delay {
    sysclock: Hertz,
    syst: SYST,
}

impl Delay {
    /// Configures the system timer (SysTick) as a delay provider
    pub fn new(mut syst: SYST, clocks: &mut GenericClockController) -> Self {
        syst.set_clock_source(SystClkSource::Core);

        Delay {
            syst,
            sysclock: clocks.gclk0().into(),
        }
    }

    /// Releases the system timer (SysTick) resource
    pub fn free(self) -> SYST {
        self.syst
    }
}

impl DelayMs<u32> for Delay {
    fn delay_ms(&mut self, ms: u32) {
        self.delay_us(ms * 1_000);
    }
}

impl DelayMsAlpha<u32> for Delay {
    type Error = ();

    fn try_delay_ms(&mut self, ms: u32) -> Result<(), Self::Error> {
        Ok(self.delay_us(ms * 1_000))
    }
}

impl DelayMs<u16> for Delay {
    fn delay_ms(&mut self, ms: u16) {
        self.delay_ms(ms as u32);
    }
}

impl DelayMsAlpha<u16> for Delay {
    type Error = ();

    fn try_delay_ms(&mut self, ms: u16) -> Result<(), Self::Error> {
        Ok(self.delay_ms(ms as u32))
    }
}

impl DelayMs<u8> for Delay {
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(ms as u32);
    }
}

impl DelayMsAlpha<u8> for Delay {
    type Error = ();

    fn try_delay_ms(&mut self, ms: u8) -> Result<(), Self::Error> {
        Ok(self.delay_ms(ms as u32))
    }
}

impl DelayUs<u32> for Delay {
    fn delay_us(&mut self, us: u32) {
        // The SysTick Reload Value register supports values between 1 and 0x00FFFFFF.
        const MAX_RVR: u32 = 0x00FF_FFFF;

        let mut total_rvr = us * (self.sysclock.0 / 1_000_000);

        while total_rvr != 0 {
            let current_rvr = if total_rvr <= MAX_RVR {
                total_rvr
            } else {
                MAX_RVR
            };

            self.syst.set_reload(current_rvr);
            self.syst.clear_current();
            self.syst.enable_counter();

            // Update the tracking variable while we are waiting...
            total_rvr -= current_rvr;

            while !self.syst.has_wrapped() {}

            self.syst.disable_counter();
        }
    }
}

impl DelayUsAlpha<u32> for Delay {
    type Error = ();

    fn try_delay_us(&mut self, us: u32) -> Result<(), Self::Error> {
        // The SysTick Reload Value register supports values between 1 and 0x00FFFFFF.
        const MAX_RVR: u32 = 0x00FF_FFFF;

        let mut total_rvr = us * (self.sysclock.0 / 1_000_000);

        while total_rvr != 0 {
            let current_rvr = if total_rvr <= MAX_RVR {
                total_rvr
            } else {
                MAX_RVR
            };

            self.syst.set_reload(current_rvr);
            self.syst.clear_current();
            self.syst.enable_counter();

            // Update the tracking variable while we are waiting...
            total_rvr -= current_rvr;

            while !self.syst.has_wrapped() {}

            self.syst.disable_counter();
        }

        Ok(())
    }
}

impl DelayUs<u16> for Delay {
    fn delay_us(&mut self, us: u16) {
        self.delay_us(us as u32)
    }
}

impl DelayUsAlpha<u16> for Delay {
    type Error = ();

    fn try_delay_us(&mut self, us: u16) -> Result<(), Self::Error> {
        Ok(self.delay_us(us as u32))
    }
}

impl DelayUs<u8> for Delay {
    fn delay_us(&mut self, us: u8) {
        self.delay_us(us as u32)
    }
}

impl DelayUsAlpha<u8> for Delay {
    type Error = ();

    fn try_delay_us(&mut self, us: u8) -> Result<(), Self::Error> {
        Ok(self.delay_us(us as u32))
    }
}
