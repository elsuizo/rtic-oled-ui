// use embedded_hal as hal;

// use hal::serial::Write;
use nb::block;
use stm32f1xx_hal::pac::USART1;

use stm32f1xx_hal::serial::Tx;

pub struct Logger {
    tx_pin: Tx<USART1>,
}

// TODO(elsuizo:2021-11-18): RX ??? maybe???
/// a UART logger interface
impl Logger {
    pub fn new(tx_pin: Tx<USART1>) -> Self {
        Self { tx_pin }
    }

    pub fn log(&mut self, data: &str) -> Result<(), ()> {
        self.send("LOG: ".as_bytes())?;
        self.send(data.as_bytes())?;
        self.send("\r\n".as_bytes())
    }

    pub fn warn(&mut self, data: &str) -> Result<(), ()> {
        self.send("WRN: ".as_bytes())?;
        self.send(data.as_bytes())?;
        self.send("\r\n".as_bytes())
    }

    pub fn error(&mut self, data: &str) -> Result<(), ()> {
        self.send("ERR: ".as_bytes())?;
        self.send(data.as_bytes())?;
        self.send("\r\n".as_bytes())
    }

    pub fn send(&mut self, buf: &[u8]) -> Result<(), ()> {
        for &byte in buf {
            if byte == 0x00 {
                continue;
            }
            block!(self.tx_pin.write(byte)).ok();
        }
        Ok(())
    }
}
