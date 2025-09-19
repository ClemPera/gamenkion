use epd_waveshare::{epd2in13_v2::*, prelude::*};
// use lib::epd_waveshare::{epd2in13_v2::*, prelude::*};

// use embedded_graphics::{
//   prelude::*, primitives::{Line, PrimitiveStyle},
// };

use embedded_graphics::{
    image::ImageRaw, mono_font::{ascii::FONT_9X18_BOLD, mapping, DecorationDimensions, MonoFont, MonoTextStyle}, pixelcolor::BinaryColor, prelude::*, primitives::{Line, PrimitiveStyle, Rectangle}, text::Text
};

use esp_idf_svc::hal::{delay::Ets, gpio::{AnyIOPin, PinDriver}, spi::{config::Config, SpiDriverConfig}};
use esp_idf_svc::hal::spi::SpiDeviceDriver;
use esp_idf_svc::hal::prelude::Peripherals;

use u8g2_fonts::{fonts::u8g2_font_ncenB24_tf, types::{FontColor, HorizontalAlignment, VerticalPosition}, Content, FontRenderer};

use std::{thread, time};

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

  let font = FontRenderer::new::<u8g2_font_ncenB24_tf>();

  font.render_aligned(
      format!("{}°C", dht.temperature).as_str(),
      display.bounding_box().center() + Point::new(-50, 10),
      VerticalPosition::Baseline,
      HorizontalAlignment::Center,
      FontColor::Transparent(Color::Black),
      &mut display,
  ).unwrap();

  font.render_aligned(
      format!("{} %", dht.humidity).as_str(),
      display.bounding_box().center() + Point::new(60, 10),
      VerticalPosition::Baseline,
      HorizontalAlignment::Center,
      FontColor::Transparent(Color::Black),
      &mut display,
  ).unwrap();

  //Draw text
  // let style = MonoTextStyle::new(, Color::Black);
  // Text::new(&format!("{}°C", dht.temperature), Point::new(20, 60), style).draw(&mut display).unwrap();
  // Text::new(&format!("{} %", dht.humidity), Point::new(150, 60), style).draw(&mut display).unwrap();

  let buf = display.buffer().len();
  log::info!("buffer1: {buf:?}");

  // display.fill_solid(&Rectangle::new(Point::new(10, 50), Size::new(80, 30)), Color::Black).unwrap();
  // display.fill_solid(&Rectangle::new(Point::new(140, 50), Size::new(55, 30)), Color::Black).unwrap();

  let buf = display.buffer().len();
  log::info!("buffer2: {buf:?}");

  //TOOD: this crash :(
  epd.update_and_display_frame(&mut device, &display.buffer(), &mut delay).unwrap();

  // epd.sleep(&mut device, &mut delay).unwrap();

  // thread::sleep(time::Duration::from_millis(1000));
  
  // epd.wake_up(&mut device, &mut delay).unwrap();

  // let buf = display.buffer().len();
  // log::info!("buffer3: {buf:?}");
  
  // display.fill_solid(&Rectangle::new(Point::new(10, 50), Size::new(80, 30)), Color::Black).unwrap();

  // let buf = display.buffer().len();
  // log::info!("buffer4: {buf:?}");
  
  // epd.update_partial_frame(&mut device, &mut delay, &display.buffer(), 10, 50, 80, 30).unwrap();

  // display.fill_solid(&Rectangle::new(Point::new(140, 50), Size::new(55, 30)), Color::Black).unwrap();
  // epd.update_partial_frame(&mut device, &mut delay, &display.buffer(), 10, 50, 80, 30).unwrap();

  // epd.display_frame(&mut device, &mut delay).unwrap();
  // epd.sleep(&mut device, &mut delay).unwrap();
}
