use core::convert::Infallible;
use cortex_m_semihosting::{debug, hprintln};
use embedded_hal::digital::{ErrorType, OutputPin};

pub struct GPIOState {
    state: bool,
}

impl ErrorType for GPIOState {
    type Error = Infallible;
}

impl GPIOState {
    pub fn new() -> Self {
        Self { state: false }
    }

    pub fn write_high(&mut self) {
        self.state = true;
        hprintln!("GPIO HIGH");
    }

    pub fn write_low(&mut self) {
        self.state = false;
        hprintln!("GPIO LOW");
    }
}

impl OutputPin for GPIOState {
    fn set_high(&mut self) -> Result<(), Infallible> {
        self.write_high();
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Infallible> {
        self.write_low();
        Ok(())
    }
}
