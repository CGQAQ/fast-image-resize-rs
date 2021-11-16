#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use image::{ GenericImageView};
use napi::{bindgen_prelude::*, CallContext, Env, JsNull, JsNumber, JsString};
use napi_derive::napi;

use fast_image_resize::{DifferentTypesOfPixelsError, FilterType, Image, ImageRowsMut, ImageView, ImageViewMut, MulDiv, PixelType, ResizeAlg, Resizer};

use image::io::Reader as ImageReader;
use std::io::{self, BufWriter};
use std::{num::NonZeroU32, result};
use png::{Encoder, ColorType};

#[cfg(all(
  any(windows, unix),
  target_arch = "x86_64",
  not(target_env = "musl"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn _resize(
  input: &ImageView,
  output: &mut ImageViewMut,
  algorithm: ResizeAlg,
) -> result::Result<(), DifferentTypesOfPixelsError> {
  let mut resizer = Resizer::new(algorithm);

  resizer.resize(input, output)
}

#[napi]
pub fn resize(input: Buffer, output_width: u32, output_height: u32) -> Result<Buffer> {
  assert!(output_width > 0);
  assert!(output_height > 0);

  let input: ImageReader<io::Cursor<&[u8]>> =
    ImageReader::new(io::Cursor::new(<&[u8]>::from(&input)))
      .with_guessed_format()
      .unwrap();
  let input_image = input.decode().unwrap();

  println!("x{}y{}x2{}y2{}", input_image.width(), input_image.height(), output_width, output_height);

  let input_image = Image::from_vec_u8(
    NonZeroU32::new(input_image.width()).unwrap(),
    NonZeroU32::new(input_image.height()).unwrap(),
    input_image.to_rgba8().into_raw(),
    PixelType::U8x4,
  )
  .unwrap();

  let mut output_image = Image::new(
    NonZeroU32::new(output_width).unwrap(),
    NonZeroU32::new(output_height).unwrap(),
    PixelType::U8x4,
  );

  println!("before: {}   {}   {:?}", output_image.buffer().len(), input_image.buffer().len(), input_image.pixel_type());
  _resize(
    &input_image.view(),
    &mut output_image.view_mut(),
    ResizeAlg::Convolution(FilterType::Bilinear),
  )
  .unwrap();

  println!("{}", output_image.buffer().len());
  let mut result_buf = BufWriter::new(Vec::new());
  println!("after111: {}", result_buf.buffer().len());

  let mut writer = Encoder::new(&mut result_buf, output_width, output_height);
  writer.set_color(ColorType::RGBA);
  writer.set_depth(png::BitDepth::Eight);
  let mut writer = writer.write_header().unwrap();

  writer.write_image_data(output_image.buffer().into()).unwrap();

  drop(writer);

  println!("after: {}", result_buf.buffer().len());

  Ok(result_buf.buffer().into())
}
