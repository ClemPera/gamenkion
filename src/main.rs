use epd_waveshare::{epd2in13_v2::*, prelude::*};
// use lib::epd_waveshare::{epd2in13_v2::*, prelude::*};

// use embedded_graphics::{
//   prelude::*, primitives::{Line, PrimitiveStyle},
// };

use embedded_graphics::{
    mono_font::{iso_8859_7::FONT_10X20, MonoTextStyle}, prelude::*, primitives::Rectangle, text::Text
};

use esp_idf_svc::hal::{delay::Ets, gpio::{AnyIOPin, PinDriver}, spi::{config::Config, SpiDriverConfig}};
use esp_idf_svc::hal::spi::SpiDeviceDriver;
use esp_idf_svc::hal::prelude::Peripherals;

use esp_idf_dht;

fn main() {
  // It is necessary to call this function once. Otherwise some patches to the runtime
  // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
  esp_idf_svc::sys::link_patches();
  
  // Bind the log crate to the ESP Logging facilities
  esp_idf_svc::log::EspLogger::initialize_default();
  
  let peripherals = Peripherals::take().unwrap();
  let pins = peripherals.pins;

  let mut sensor = PinDriver::input_output_od(pins.gpio21).unwrap();
  let dht = esp_idf_dht::read(&mut sensor).unwrap();

  let spi = peripherals.spi2;
  let sclk = pins.gpio12;
  let sdo = pins.gpio11;
  let busy_in = PinDriver::input(pins.gpio16).unwrap();
  let dc = PinDriver::output(pins.gpio18).unwrap();
  let rst = PinDriver::output(pins.gpio17).unwrap();
  // let cs = PinDriver::output(pins.gpio8).unwrap(); 

  let mut device = SpiDeviceDriver::new_single(
    spi, 
    sclk, 
    sdo, 
    Option::<AnyIOPin>::None,
    Some(pins.gpio8),
    &SpiDriverConfig::default(), 
    &Config::new().baudrate(112500.into())
  ).unwrap();

  let mut delay = Ets;
  
  // Setup EPD
  let mut epd = Epd2in13::new(&mut device, busy_in, dc, rst, &mut delay, None).unwrap();
  
  // Use display graphics from embedded-graphics
  let mut display = Display2in13::default();
  
  display.clear(Color::White).unwrap();
  display.set_rotation(DisplayRotation::Rotate90);
  // display.fill_solid(&Rectangle::new(Point::new(0, 0), Size::new(30, 200)), Color::Black).unwrap();
  // Use embedded graphics for drawing a line
  // let _ = Line::new(Point::new(0, 120), Point::new(0, 295))
  // .into_styled(PrimitiveStyle::with_stroke(Color::White, 5))
  // .draw(&mut display);

  //Draw text
  let style = MonoTextStyle::new(&FONT_10X20, Color::Black);
  Text::new(&format!("{}°C", dht.temperature), Point::new(10, 80), style).draw(&mut display).unwrap();
  Text::new(&format!("{}%", dht.humidity), Point::new(10, 40), style).draw(&mut display).unwrap();
  
  // // Display updated frame
  // epd.update_frame(&mut device, &display.buffer(), &mut delay).unwrap();
  // epd.display_frame(&mut device, &mut delay).unwrap();

  epd.update_and_display_frame(&mut device, &display.buffer(), &mut delay).unwrap();
  
  // // Set the EPD to sleep
  epd.sleep(&mut device, &mut delay).unwrap();
}
