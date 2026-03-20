#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_hal::{
    clock::CpuClock,
    gpio::{Io, Level, Output, OutputConfig},
    main,
    time::{Duration, Instant,Rate},
    spi::{
        Mode, 
        master::{Config,Spi},
    },
};
use rtt_target::rprintln;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    esp_hal::system::software_reset()
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]


#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Set GPIO0 as an output, and set its state high initially.
    let mut led = Output::new(peripherals.GPIO20, Level::High, OutputConfig::default());

    //Get the I2C driver
    let mut lcd_spi = Spi::new(
        peripherals.SPI2, //SPI2 is only usable
        Config::default()
            .with_frequency(Rate::from_khz(200)) //lcd max is 400 kHz
            .with_mode(Mode::_3),
    )
    .unwrap()
    .with_sck(peripherals.GPIO15)
    .with_miso(peripherals.GPIO23)
    .with_mosi(peripherals.GPIO22)
    .with_cs(peripherals.GPIO21);

    loop {
        led.toggle();
        // Wait for half a second
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }
}
