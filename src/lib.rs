use dither::dither_image;
pub use dither::{DitherMode, ProcessMode};
use palette::{rgb::Rgba, FromColor, Hsva, Srgb};
use sampling::{sample_image, SampleMode};
use serde::{Deserialize, Serialize};
pub use sprite::Sprite;

mod dither;
mod sampling;
mod sprite;

#[derive(Deserialize, Serialize)]
pub struct I2PState {
    pub upscale: i32,
    pub brightness: i32,
    pub contrast: i32,
    pub img_gamma: i32,
    pub saturation: i32,
    pub dither_amount: i32,
    pub alpha_threshold: i32,
    pub sharpen: i32,
    pub hue: i32,
    pub gauss: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub image_outline: i32,
    pub image_inline: i32,
    pub pixel_sample_mode: SampleMode,
    pub pixel_dither_mode: DitherMode,
    pub pixel_process_mode: ProcessMode,
    pub image_out_width: i32,
    pub image_out_height: i32,
    pub image_out_swidth: i32,
    pub image_out_sheight: i32,
    pub palette_weight: i32,
    pub palette: Vec<Color>,
    pub quant_cluster_list: Vec<Vec<Color>>,
    pub quant_centroid_list: Vec<Color>,
    pub quant_assignment: Vec<usize>,
    pub quant_k: i32,
}

impl Default for I2PState {
    fn default() -> Self {
        Self {
            upscale: 1,
            brightness: 0,
            contrast: 0,
            img_gamma: 100,
            saturation: 100,
            dither_amount: 64,
            alpha_threshold: 128,
            sharpen: 0,
            hue: 0,
            gauss: 1,
            offset_x: 0,
            offset_y: 0,
            image_outline: -1,
            image_inline: -1,
            pixel_sample_mode: Default::default(),
            pixel_dither_mode: Default::default(),
            pixel_process_mode: Default::default(),
            image_out_width: 128,
            image_out_height: 128,
            image_out_swidth: 2,
            image_out_sheight: 2,
            palette_weight: 2,
            palette: Default::default(),
            quant_cluster_list: Default::default(),
            quant_centroid_list: Default::default(),
            quant_assignment: Default::default(),
            quant_k: 16,
        }
    }
}

pub type Color = Rgba<Srgb, u8>;

pub fn process_image(s: &mut I2PState, input: &Sprite, output: &mut Sprite) {
    let mut temp = sample_image(s, input, output.width, output.height);
    let gamma_factor = s.img_gamma as f64 / 100.0;
    let contrast_factor =
        (259.0 * (255.0 + s.contrast as f64)) / (255.0 * (259.0 - s.contrast as f64));
    let saturation_factor = s.saturation as f64 / 100.0;
    let brightness_factor = s.brightness as f64 / 100.0;

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

            if s.hue != 0 {
                let hue = s.hue as f32;
                let mut hsv = Hsva::from_color(input.into_format::<f32, f32>());
                hsv.hue += hue;
                input = Rgba::from_color(hsv).into_format();
            }

            let r = input.red as f64;
            let g = input.green as f64;
            let b = input.blue as f64;
            input.red = 0x0.max(0xff.min(((rr * r) + (gr * g) + (br * b) + wr) as u8));
            input.green = 0x0.max(0xff.min(((rg * r) + (gg * g) + (bg * b) + wg) as u8));
            input.blue = 0x0.max(0xff.min(((rb * r) + (gb * g) + (bb * b) + wb) as u8));

            if s.img_gamma != 100 {
                input.red = 0x0
                    .max(0xff.min((255.0 * (input.red as f64 / 255.0).powf(gamma_factor)) as u8));
                input.green = 0x0
                    .max(0xff.min((255.0 * (input.green as f64 / 255.0).powf(gamma_factor)) as u8));
                input.blue = 0x0
                    .max(0xff.min((255.0 * (input.blue as f64 / 255.0).powf(gamma_factor)) as u8));
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
            if s.image_inline >= 0
                && temp.get_pixel(x, y).unwrap().alpha != 0
                && (temp.get_pixel(x, y - 1).unwrap_or_default().alpha == 0
                    || temp.get_pixel(x - 1, y).unwrap_or_default().alpha == 0
                    || temp.get_pixel(x + 1, y).unwrap_or_default().alpha == 0
                    || temp.get_pixel(x, y + 1).unwrap_or_default().alpha == 0)
            {
                output.set_pixel(x, y, s.palette[s.image_inline as usize]);
            }

            if s.image_outline >= 0
                && temp.get_pixel(x, y).unwrap().alpha != 0
                && (temp.get_pixel(x, y - 1).unwrap_or_default().alpha == 0
                    || temp.get_pixel(x - 1, y).unwrap_or_default().alpha == 0
                    || temp.get_pixel(x + 1, y).unwrap_or_default().alpha == 0
                    || temp.get_pixel(x, y + 1).unwrap_or_default().alpha == 0)
            {
                output.set_pixel(x, y, s.palette[s.image_outline as usize]);
            }
        }
    }
}
