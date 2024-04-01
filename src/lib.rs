#![warn(clippy::pedantic)]

use std::io::{BufWriter, Cursor};

use anyhow::Result;
use dither::dither_image;
pub use dither::{DistanceMode, DitherMode};
use image::{load_from_memory, write_buffer_with_format, ColorType, GenericImageView, ImageBuffer};
use palette::{
    rgb::{FromHexError, Rgba},
    FromColor, Hsva, Srgb,
};
use sampling::{sample_image, SampleMode};
pub use sprite::Sprite;

mod dither;
mod sampling;
mod sprite;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "wasm")]
pub use wasm_bindgen_rayon::init_thread_pool;

pub struct I2PState<'a> {
    pub brightness: f64,
    pub contrast: f64,
    pub gamma: f64,
    pub saturation: f64,
    pub hue: f64,
    pub dither_amount: i32,
    pub alpha_threshold: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub image_outline: Option<usize>,
    pub image_inline: Option<usize>,
    pub pixel_sample_mode: SampleMode,
    pub pixel_dither_mode: DitherMode,
    pub pixel_distance_mode: DistanceMode,
    pub image_out_width: i32,
    pub image_out_height: i32,
    pub palette_weight: i32,
    pub palette: &'a [Color],
}


#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct PixelizationOptions {
    pub brightness: Option<f64>,
    pub contrast: Option<f64>,
    pub gamma: Option<f64>,
    pub saturation: Option<f64>,
    pub hue: Option<f64>,
    pub dither_amount: i32,
    pub alpha_threshold: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub image_outline: Option<usize>,
    pub image_inline: Option<usize>,
    pub pixel_sample_mode: SampleMode,
    pub pixel_dither_mode: DitherMode,
    pub pixel_distance_mode: DistanceMode,
    pub image_out_width: i32,
    pub image_out_height: i32,
    pub palette_weight: i32,
}

impl Default for I2PState<'_> {
    fn default() -> Self {
        Self {
            brightness: 0.0,
            contrast: 0.0,
            gamma: 100.0,
            saturation: 100.0,
            hue: 0.0,
            dither_amount: 64,
            alpha_threshold: 128,
            offset_x: 0,
            offset_y: 0,
            image_outline: None,
            image_inline: None,
            pixel_sample_mode: SampleMode::default(),
            pixel_dither_mode: DitherMode::default(),
            pixel_distance_mode: DistanceMode::default(),
            image_out_width: 128,
            image_out_height: 128,
            palette_weight: 2,
            palette: Default::default(),
        }
    }
}

pub type Color = Rgba<Srgb, u8>;
pub struct Components(f64, f64, f64);

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn process_image_wasm(input: &[u8], palette: Vec<String>, options: PixelizationOptions) -> Result<Vec<u8>, JsError> {
    let result = process_image(input, &palette, options);
    result.map_err(|e| JsError::new(&format!("{e}")))
}

pub fn process_image(input: &[u8], palette: &[String], options: PixelizationOptions) -> Result<Vec<u8>> {
    let image = load_from_memory(input)?;
    let palette: Vec<Color> = palette
        .iter()
        .map(|c| c.parse::<Color>())
        .collect::<Result<Vec<_>, FromHexError>>()?;

    let mut state = I2PState {
        brightness: options.brightness.unwrap_or(0.0),
        contrast: options.contrast.unwrap_or(0.0),
        gamma: options.gamma.unwrap_or(100.0),
        saturation: options.saturation.unwrap_or(100.0),
        hue: options.hue.unwrap_or(0.0),
        dither_amount: options.dither_amount,
        alpha_threshold: options.alpha_threshold,
        offset_x: options.offset_x,
        offset_y: options.offset_y,
        image_outline: options.image_outline,
        image_inline: options.image_inline,
        pixel_sample_mode: options.pixel_sample_mode,
        pixel_dither_mode: options.pixel_dither_mode,
        pixel_distance_mode: options.pixel_distance_mode,
        image_out_width: options.image_out_width,
        image_out_height: options.image_out_height,
        palette_weight: options.palette_weight,
        palette: &palette,
    };

    let mut output = Sprite {
        width: image.width() as usize,
        height: image.height() as usize,
        data: vec![Color::default(); image.width() as usize * image.height() as usize],
    };
    for (x, y, pixel) in image.pixels() {
        let pixel = pixel.0;
        output.set_pixel(
            x as usize,
            y as usize,
            Color::new(pixel[0], pixel[1], pixel[2], pixel[3]),
        );
    }

    let input = output.clone();
    process_sprite(&mut state, &input, &mut output);

    let mut imgbuf: ImageBuffer<image::Rgba<u8>, Vec<_>> =
        ImageBuffer::new(output.width as u32, output.height as u32);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let sprite_pixel = output.get_pixel(x as usize, y as usize).unwrap();
        *pixel = image::Rgba([
            sprite_pixel.red,
            sprite_pixel.green,
            sprite_pixel.blue,
            sprite_pixel.alpha,
        ]);
    }

    let mut output_image = Cursor::new(Vec::new());
    write_buffer_with_format(
        &mut BufWriter::new(&mut output_image),
        &imgbuf,
        imgbuf.width(),
        imgbuf.height(),
        ColorType::Rgba8,
        image::ImageFormat::Png,
    )?;
    Ok(output_image.into_inner())
}

pub fn process_sprite(s: &mut I2PState, input: &Sprite, output: &mut Sprite) {
    let mut temp = sample_image(s, input, output.width, output.height);
    let gamma_factor = s.gamma / 100.0;
    let contrast_factor = (259.0 * (255.0 + s.contrast)) / (255.0 * (259.0 - s.contrast));
    let saturation_factor = s.saturation / 100.0;
    let brightness_factor = s.brightness / 100.0;

    let t = (1.0 - contrast_factor) / 2.0;
    let sr = (1.0 - saturation_factor) * 0.3086;
    let sg = (1.0 - saturation_factor) * 0.6094;
    let sb = (1.0 - saturation_factor) * 0.0820;

    let rr = contrast_factor * (sr + saturation_factor);
    let rg = contrast_factor * sr;
    let rb = contrast_factor * sr;

    let gr = contrast_factor * sg;
    let gg = contrast_factor * (sg + saturation_factor);
    let gb = contrast_factor * sg;

    let br = contrast_factor * sb;
    let bg = contrast_factor * sb;
    let bb = contrast_factor * (sb + saturation_factor);

    let wr = (t + brightness_factor) * 255.0;
    let wg = (t + brightness_factor) * 255.0;
    let wb = (t + brightness_factor) * 255.0;

    for y in 0..output.height {
        for x in 0..output.width {
            let mut input = temp[y * output.width + x];
            let a = input.alpha;

            if s.hue != 0.0 {
                let hue = s.hue;
                let mut hsv = Hsva::from_color(input.into_format::<f64, f64>());
                hsv.hue += hue;
                input = Rgba::from_color(hsv).into_format();
            }

            let r = f64::from(input.red);
            let g = f64::from(input.green);
            let b = f64::from(input.blue);
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            {
                input.red = 0x0.max(0xff.min(((rr * r) + (gr * g) + (br * b) + wr) as u8));
                input.green = 0x0.max(0xff.min(((rg * r) + (gg * g) + (bg * b) + wg) as u8));
                input.blue = 0x0.max(0xff.min(((rb * r) + (gb * g) + (bb * b) + wb) as u8));
            }

            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            if f64::abs(s.gamma - 100.0) > f64::EPSILON {
                input.red = 0x0.max(
                    0xff.min((255.0 * (f64::from(input.red) / 255.0).powf(gamma_factor)) as u8),
                );
                input.green = 0x0.max(
                    0xff.min((255.0 * (f64::from(input.green) / 255.0).powf(gamma_factor)) as u8),
                );
                input.blue = 0x0.max(
                    0xff.min((255.0 * (f64::from(input.blue) / 255.0).powf(gamma_factor)) as u8),
                );
            }

            input.alpha = a;
            temp[y * output.width + x] = input;
        }
    }

    dither_image(s, temp, output, output.width, output.height);

    post_process_image(s, output);
}

fn post_process_image(s: &mut I2PState, output: &mut Sprite) {
    let temp = output.clone();

    for y in 0..output.height {
        for x in 0..output.width {
            if let Some(inline) = s.image_inline {
                if temp.get_pixel(x, y).unwrap().alpha != 0
                    && (temp.get_pixel(x, y - 1).unwrap_or_default().alpha == 0
                        || temp.get_pixel(x - 1, y).unwrap_or_default().alpha == 0
                        || temp.get_pixel(x + 1, y).unwrap_or_default().alpha == 0
                        || temp.get_pixel(x, y + 1).unwrap_or_default().alpha == 0)
                {
                    output.set_pixel(x, y, s.palette[inline]);
                }
            }

            if let Some(outline) = s.image_outline {
                if temp.get_pixel(x, y).unwrap().alpha != 0
                    && (temp.get_pixel(x, y - 1).unwrap_or_default().alpha == 0
                        || temp.get_pixel(x - 1, y).unwrap_or_default().alpha == 0
                        || temp.get_pixel(x + 1, y).unwrap_or_default().alpha == 0
                        || temp.get_pixel(x, y + 1).unwrap_or_default().alpha == 0)
                {
                    output.set_pixel(x, y, s.palette[outline]);
                }
            }
        }
    }
}
