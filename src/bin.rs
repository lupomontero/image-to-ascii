extern crate clap;
extern crate image;
extern crate image_to_ascii;


use clap::{Arg, App};
use image::GenericImageView;
use image_to_ascii::from_data;


pub fn main() {
  let matches = App::new("image-to-ascii")
    .version("0.1.0")
    .author("Lupo Montero <lupomontero@gmail.com>")
    .about("Convert image to ACII text")
    .arg(Arg::with_name("fname")
      .required(true)
      .takes_value(true)
      .index(1)
      .help("Path to image to be converted"))
    .arg(Arg::with_name("width")
      .short("w")
      .long("width")
      .takes_value(true)
      .help("Column width of resulting ASCII text"))
    .arg(Arg::with_name("mode")
      .short("m")
      .long("mode")
      .takes_value(true)
      .help("Output mode: plain, html or ansi"))
    .get_matches();

  let fname = matches.value_of("fname").unwrap();
  let width: u32 = matches.value_of("width").unwrap_or("80").parse().unwrap();
  let mode = matches.value_of("mode").unwrap_or("plain");

  let img = image::open(fname).unwrap();
  let dimensions = img.dimensions();
  // Compute height based on desired width
  let height = (dimensions.1 as f32 * (width as f32 / dimensions.0 as f32)) as u32;
  // Resize image to desired size.
  let resized = img.thumbnail(width, height);
  // Read width and height from resized image as these may not match exactly the desired values we
  // passed to `img.thumbnail()`.
  let resized_dimensions = resized.dimensions();
  let mut data: Vec<u8> = Vec::new();

  for pixel in resized.pixels() {
    data.push(pixel.2.data[0]);
    data.push(pixel.2.data[1]);
    data.push(pixel.2.data[2]);
    data.push(pixel.2.data[3]);
  }

  print!("{}", from_data(resized_dimensions.0, resized_dimensions.1, &data, mode));
}
