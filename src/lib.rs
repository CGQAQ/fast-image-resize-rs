use napi::bindgen_prelude::*;
use napi_derive::napi;

use image::{codecs, ColorType, GenericImageView, ImageFormat};

use fast_image_resize::{
  DifferentTypesOfPixelsError, Image, ImageView, ImageViewMut, PixelType, ResizeAlg, Resizer,
};

use image::io::Reader as ImageReader;
use std::io::{self, BufWriter};
use std::{num::NonZeroU32, result};

#[cfg(all(
  not(debug_assertions),
  not(all(target_os = "windows", target_arch = "aarch64")),
  not(all(target_os = "linux", target_arch = "aarch64", target_env = "musl")),
))]
#[global_allocator]
static ALLOC: mimalloc_rust::GlobalMiMalloc = mimalloc_rust::GlobalMiMalloc;

#[inline]
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

  let format = input.format().unwrap();

  let input_image = input.decode().unwrap();

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
    input_image.pixel_type(),
  );

  _resize(
    &input_image.view(),
    &mut output_image.view_mut(),
    // ResizeAlg::Convolution(FilterType::Bilinear),
    Default::default(),
  )
  .unwrap();
  let mut result = BufWriter::new(Vec::new());
  let ref mut result_buf = result;
  match format {
    ImageFormat::Png => codecs::png::PngEncoder::new(result_buf)
      .encode(
        output_image.buffer(),
        output_image.width().into(),
        output_image.height().into(),
        ColorType::Rgba8,
      )
      .unwrap(),
    ImageFormat::Jpeg => codecs::jpeg::JpegEncoder::new(result_buf)
      .encode(
        output_image.buffer(),
        output_image.width().into(),
        output_image.height().into(),
        ColorType::Rgba8,
      )
      .unwrap(),
    ImageFormat::Gif => codecs::gif::GifEncoder::new(result_buf)
      .encode(
        output_image.buffer(),
        output_image.width().into(),
        output_image.height().into(),
        ColorType::Rgba8,
      )
      .unwrap(),
    // ImageFormat::WebP => codecs::webp::WebpEncoder::new(&mut result_buf)
    //   .encode(
    //     output_image.buffer(),
    //     output_image.width().into(),
    //     output_image.height().into(),
    //     ColorType::Rgba8,
    //   )
    //   .unwrap(),
    ImageFormat::WebP => unimplemented!("ImageFormat not supported"),
    ImageFormat::Pnm => codecs::pnm::PnmEncoder::new(result_buf)
      .encode(
        output_image.buffer(),
        output_image.width().into(),
        output_image.height().into(),
        ColorType::Rgba8,
      )
      .unwrap(),
    // ImageFormat::Tiff => codecs::tiff::TiffEncoder::new(result_buf)
    //   .encode(
    //     output_image.buffer(),
    //     output_image.width().into(),
    //     output_image.height().into(),
    //     ColorType::Rgba8,
    //   )
    //   .unwrap(),
    ImageFormat::Tiff => unimplemented!("ImageFormat not supported"),
    ImageFormat::Tga => codecs::tga::TgaEncoder::new(result_buf)
      .encode(
        output_image.buffer(),
        output_image.width().into(),
        output_image.height().into(),
        ColorType::Rgba8,
      )
      .unwrap(),
    // ImageFormat::Dds => codecs::dds::DdsEncoder::new(result_buf)
    //   .encode(
    //     output_image.buffer(),
    //     output_image.width().into(),
    //     output_image.height().into(),
    //     ColorType::Rgba8,
    //   )
    //   .unwrap(),
    ImageFormat::Dds => unimplemented!("ImageFormat not supported"),
    ImageFormat::Bmp => codecs::bmp::BmpEncoder::new(result_buf)
      .encode(
        output_image.buffer(),
        output_image.width().into(),
        output_image.height().into(),
        ColorType::Rgba8,
      )
      .unwrap(),
    ImageFormat::Ico => codecs::ico::IcoEncoder::new(result_buf)
      .encode(
        output_image.buffer(),
        output_image.width().into(),
        output_image.height().into(),
        ColorType::Rgba8,
      )
      .unwrap(),
    // ImageFormat::Hdr => codecs::hdr::HdrEncoder::new(result_buf)
    //   .encode(
    //     output_image.buffer(),
    //     <u32>::from(output_image.width()) as usize,
    //     <u32>::from(output_image.height()) as usize,
    //   )
    //   .unwrap(),
    ImageFormat::Hdr => unimplemented!("ImageFormat not supported"),
    ImageFormat::Farbfeld => {
      codecs::farbfeld::FarbfeldEncoder::new(result_buf)
        .encode(
          output_image.buffer(),
          output_image.width().into(),
          output_image.height().into(),
        )
        .unwrap();
    }
    // ImageFormat::Avif => codecs::avif::AvifEncoder::new(result_buf)
    //   .encode(
    //     output_image.buffer(),
    //     output_image.width().into(),
    //     output_image.height().into(),
    //   )
    //   .unwrap(),
    ImageFormat::Avif => unimplemented!("ImageFormat not supported"),
    ImageFormat::__NonExhaustive(_) => {
      panic!("Unsupported image format");
    }
  };

  Ok(result.into_inner().unwrap().into())
}
