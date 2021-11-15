#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use image::codecs::png::PngEncoder;
use image::{ColorType, GenericImageView};
use napi::{bindgen_prelude::*, CallContext, Env, JsNull, JsNumber, JsString};
use napi_derive::napi;

use fast_image_resize::{
  DifferentTypesOfPixelsError, Image, ImageRowsMut, ImageView, ImageViewMut, MulDiv, PixelType,
  ResizeAlg, Resizer,
};

use image::io::Reader as ImageReader;
use std::io::{self, BufWriter};
use std::{num::NonZeroU32, result};

#[cfg(all(
  any(windows, unix),
  target_arch = "x86_64",
  not(target_env = "musl"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn _resize(
  input: ImageView,
  mut output: ImageViewMut,
  algorithm: ResizeAlg,
) -> result::Result<(), DifferentTypesOfPixelsError> {
  let mut resizer = Resizer::new(algorithm);

  resizer.resize(&input, &mut output)
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

  let mut input_image = Image::from_vec_u8(
    NonZeroU32::new(input_image.width()).unwrap(),
    NonZeroU32::new(input_image.height()).unwrap(),
    input_image.to_rgba8().into_raw(),
    PixelType::U8x4,
  )
  .unwrap();

  // Create MulDiv instance
  let alpha_mul_div: MulDiv = Default::default();
  // Multiple RGB channels of source image by alpha channel
  alpha_mul_div
    .multiply_alpha_inplace(&mut input_image.view_mut())
    .unwrap();

  let mut output_image = Image::new(
    NonZeroU32::new(output_width).unwrap(),
    NonZeroU32::new(output_height).unwrap(),
    input_image.pixel_type(),
  );

  _resize(
    input_image.view(),
    output_image.view_mut(),
    ResizeAlg::Nearest,
  )
  .unwrap();

  // Divide RGB channels of destination image by alpha
  alpha_mul_div
    .divide_alpha_inplace(&mut output_image.view_mut())
    .unwrap();

  let mut result_buf = BufWriter::new(Vec::new());
  PngEncoder::new(&mut result_buf)
    .encode(
      output_image.buffer(),
      output_width,
      output_height,
      ColorType::Rgba8,
    )
    .unwrap();

  Ok(result_buf.buffer().into())
}
