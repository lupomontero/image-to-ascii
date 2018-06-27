#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;


pub fn floor(num: f32) -> i32 {
  num.floor() as i32
}


pub fn bound(value: i32, interval: [i32; 2]) -> u8 {
  if value < interval[0] {
    return interval[0] as u8;
  }
  if value > interval[1] {
    return interval[1] as u8;
  }
  return value as u8;
}


pub fn get_pixel_at_offset(data: &[u8], offset: usize) -> [u8; 4] {
  [
    data[offset],
    data[offset + 1],
    data[offset + 2],
    data[offset + 3],
  ]
}


pub fn get_contrasted_color(pixel: [u8; 4], contrast: i32) -> [u8; 4] {
  // calculate contrast factor
  // http://www.dfstudios.co.uk/articles/image-processing-algorithms-part-5/
  let contrast_factor = (259.0 * (contrast as f32 + 255.0)) / (255.0 * (259.0 - contrast as f32));

  return [
    bound(floor((pixel[0] as f32 - 128.0) * contrast_factor) + 128, [0, 255]),
    bound(floor((pixel[1] as f32 - 128.0) * contrast_factor) + 128, [0, 255]),
    bound(floor((pixel[2] as f32 - 128.0) * contrast_factor) + 128, [0, 255]),
    pixel[3],
  ];
}


//
// Compute pixel brightness
//
// See: http://stackoverflow.com/questions/596216/formula-to-determine-brightness-of-rgb-color
//
pub fn compute_pixel_brightness(pixel: [u8; 4]) -> f32 {
  (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32) / 255.0
}


pub fn brightness_to_char(brightness: f32) -> char {
  let chars = [' ', '.', ',', ':', ';', 'i', '1', 't', 'f', 'L', 'C', 'G', '0', '8', '@'];
  let chars_len_as_float = chars.len() as f32;
  let idx = (chars_len_as_float - 1.0) - (brightness * (chars_len_as_float - 1.0)).round();
  return chars[idx as usize];
}


pub fn pixel_to_char(data: &[u8], offset: u32, contrast: i32) -> char {
  return brightness_to_char(
    compute_pixel_brightness(
      get_contrasted_color(
        get_pixel_at_offset(data, offset as usize),
        contrast
      )
    )
  );
}


pub fn pixel_to_char_with_color(data: &[u8], offset: u32, contrast: i32) -> String {
  let pixel = get_pixel_at_offset(data, offset as usize);
  let style = format!("color: rgba({}, {}, {}, {})", pixel[0], pixel[1], pixel[2], pixel[3]);
  let character = brightness_to_char(
    compute_pixel_brightness(
      get_contrasted_color(pixel, contrast)
    )
  );
  return String::from(format!("<span style=\"{}\">{}</span>", style, character));
}


#[wasm_bindgen]
pub extern fn from_data(width: u32, height: u32, data: &[u8], mode: &str) -> String {
  // make sure data is divisible by 4 (rgba pixels)
  if data.len() % 4 != 0 {
    panic!("Crash and burn");
  }

  let mut result = String::from("");
  let mut y = 0;

  while y < height {
    if y > 0 {
      result.push('\n');
    }

    let mut x = 0;

    while x < width {
      if mode == "html" {
        result.push_str(&pixel_to_char_with_color(data, (y * width + x) * 4, 1));
      } else {
        result.push(pixel_to_char(data, (y * width + x) * 4, 1));
      }
      x += 1;
    }

    y += 2; // every other row because letters are not square
  }

  return result;
}


#[cfg(test)]
mod tests {
  use bound;
  use get_pixel_at_offset;
  use get_contrasted_color;
  use compute_pixel_brightness;
  use brightness_to_char;
  use from_data;

  #[test]
  fn bound_should_return_number_within_bounds() {
    assert_eq!(bound(11, [0, 10]), 10);
    assert_eq!(bound(15, [0, 8]), 8);
    assert_eq!(bound(-5, [0, 10]), 0);
    assert_eq!(bound(3, [0, 5]), 3);
  }

  #[test]
  fn get_pixel_at_offset_should_get_pixel() {
    let pixel = get_pixel_at_offset(&[1, 2, 3, 4, 5, 6, 7, 8], 0);
    assert_eq!(pixel[0], 1);
    assert_eq!(pixel[1], 2);
    assert_eq!(pixel[2], 3);
    assert_eq!(pixel[3], 4);
  }

  #[test]
  fn get_contrasted_color_should() {
    let pixel = get_contrasted_color([1, 200, 100, 4], 10);
    assert_eq!(pixel[0], 0);
    assert_eq!(pixel[1], 205);
    assert_eq!(pixel[2], 97);
    assert_eq!(pixel[3], 4);
  }

  #[test]
  fn compute_pixel_brightness_should() {
    assert_eq!(compute_pixel_brightness([1, 1, 1, 1]), 0.003921569);
    assert_eq!(compute_pixel_brightness([127, 127, 127, 127]), 0.49803922);
    assert_eq!(compute_pixel_brightness([255, 255, 255, 255]), 1.0);
  }

  #[test]
  fn brightness_to_char_should() {
    assert_eq!(brightness_to_char(compute_pixel_brightness([1, 1, 1, 1])), '@');
    assert_eq!(brightness_to_char(compute_pixel_brightness([127, 127, 127, 255])), 't');
    assert_eq!(brightness_to_char(compute_pixel_brightness([255, 255, 255, 255])), ' ');
  }

  #[test]
  fn from_data_should() {
    let data = [1, 1, 1, 1, 127, 127, 127, 255, 255, 255, 255, 255];
    let string = from_data(3, 1, &data);
    assert_eq!(string, "<span style=\"color: rgba(1, 1, 1, 1)\">@</span><span style=\"color: rgba(127, 127, 127, 255)\">t</span><span style=\"color: rgba(255, 255, 255, 255)\"> </span>");
  }
}
