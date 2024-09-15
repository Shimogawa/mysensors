use std::time::{self, Duration, Instant};

use anyhow::{anyhow, Result};
use rppal::gpio::{Gpio, IoPin, Level, Mode};

use crate::utils::FixedF8;

const TIMEOUT: Duration = Duration::from_millis(500);
const BIT_1_DELAY: Duration = Duration::from_micros(50);

#[derive(Debug, Clone)]
pub struct SensorData {
    pub temperature: f32,
    pub humidity: f32,
}

#[derive(Debug)]
pub struct DHT11 {
    gpio: Gpio,
    pin: IoPin,
    last_read: Instant,
    last_data: Option<SensorData>,
}

impl DHT11 {
    pub fn new(pin_id: u8) -> Result<Self> {
        let gpio = Gpio::new()?;
        let pin = gpio.get(pin_id)?.into_io(Mode::Output);
        Ok(Self {
            gpio,
            pin,
            last_read: Instant::now(),
            last_data: None,
        })
    }

    pub fn read(&mut self) -> Result<SensorData> {
        if self.last_data.is_some() && self.last_read.elapsed() < Duration::from_secs(1) {
            return Ok(self.last_data.clone().unwrap());
        }
        self.pin.set_mode(Mode::Output);
        self.send_and_sleep(Level::High, Duration::from_millis(50));
        self.send_and_sleep(Level::Low, Duration::from_millis(20));
        self.pin.set_high();
        // self.test();
        self.pin.set_mode(Mode::Input);
        self.wait_for_signal(Level::Low, TIMEOUT)?;
        self.wait_for_signal(Level::High, Duration::from_micros(100))?;
        self.wait_for_signal(Level::Low, Duration::from_micros(100))?;
        let data = self.read_data()?;
        self.last_read = Instant::now();
        self.last_data = Some(data.clone());
        Ok(data)
    }

    fn read_data(&mut self) -> Result<SensorData> {
        let humidity1 = self.read_u8()?;
        let humidity2 = self.read_u8()?;
        let temperature1 = self.read_u8()?;
        let temperature2 = self.read_u8()?;
        let checksum = self.read_u8()?;
        if humidity1
            .wrapping_add(humidity2)
            .wrapping_add(temperature1)
            .wrapping_add(temperature2)
            != checksum
        {
            return Err(anyhow!("checksum error"));
        }
        let temp = (temperature1 as f32) + (temperature2 as f32 / 10.0);
        let temp = if temperature2 & 0x80 != 0 {
            -temp
        } else {
            temp
        };
        Ok(SensorData {
            humidity: (humidity1 as f32) + (humidity2 as f32 / 10.0),
            temperature: temp,
        })
    }

    fn read_u8(&mut self) -> Result<u8> {
        let mut value = 0u8;
        for i in 0..8 {
            let bit = self.read_bit()?;
            value |= bit << (7 - i);
        }
        Ok(value)
    }

    fn read_bit(&mut self) -> Result<u8> {
        self.wait_for_signal(Level::High, TIMEOUT)?;
        let duration = self.wait_for_signal(Level::Low, TIMEOUT)?;
        if duration > BIT_1_DELAY {
            Ok(1)
        } else {
            Ok(0)
        }
    }

    fn send_and_sleep(&mut self, signal: Level, duration: Duration) {
        if signal == Level::High {
            self.pin.set_high();
        } else {
            self.pin.set_low();
        }
        std::thread::sleep(duration);
    }

    fn wait_for_signal(&mut self, level: Level, timeout: Duration) -> Result<Duration> {
        let start = time::Instant::now();
        while self.pin.read() != level {
            if start.elapsed() > timeout {
                return Err(anyhow!("timeout waiting for signal"));
            }
        }
        Ok(start.elapsed())
    }

    fn test(&mut self) {
        self.pin.set_mode(Mode::Input);
        let mut now = Instant::now();
        let mut prev = Level::High;
        let mut count = 0;
        loop {
            let level = self.pin.read();
            if level != prev {
                let elapsed = now.elapsed();
                println!("{}: {} {:?} -> {}", count, prev, elapsed, level);
                now = Instant::now();
                count += 1;
                prev = level;
            }
        }
    }

    // pub fn read_interrupt(&mut self) -> Result<SensorData, Error> {}
}
