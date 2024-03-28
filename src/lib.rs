use std::{f64::consts::PI, vec};

use palette::{rgb::Rgba, FromColor, Hsva, Srgb};
use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

const DITHER_THRESHOLD_BAYER8X8: [f32; 64] = [
    0.0 / 64.0,
    32.0 / 64.0,
    8.0 / 64.0,
    40.0 / 64.0,
    2.0 / 64.0,
    34.0 / 64.0,
    10.0 / 64.0,
    42.0 / 64.0,
    48.0 / 64.0,
    16.0 / 64.0,
    56.0 / 64.0,
    24.0 / 64.0,
    50.0 / 64.0,
    18.0 / 64.0,
    58.0 / 64.0,
    26.0 / 64.0,
    12.0 / 64.0,
    44.0 / 64.0,
    4.0 / 64.0,
    36.0 / 64.0,
    14.0 / 64.0,
    46.0 / 64.0,
    6.0 / 64.0,
    38.0 / 64.0,
    60.0 / 64.0,
    28.0 / 64.0,
    52.0 / 64.0,
    20.0 / 64.0,
    62.0 / 64.0,
    30.0 / 64.0,
    54.0 / 64.0,
    22.0 / 64.0,
    3.0 / 64.0,
    35.0 / 64.0,
    11.0 / 64.0,
    43.0 / 64.0,
    1.0 / 64.0,
    33.0 / 64.0,
    9.0 / 64.0,
    41.0 / 64.0,
    51.0 / 64.0,
    19.0 / 64.0,
    59.0 / 64.0,
    27.0 / 64.0,
    49.0 / 64.0,
    17.0 / 64.0,
    57.0 / 64.0,
    25.0 / 64.0,
    15.0 / 64.0,
    47.0 / 64.0,
    7.0 / 64.0,
    39.0 / 64.0,
    13.0 / 64.0,
    45.0 / 64.0,
    5.0 / 64.0,
    37.0 / 64.0,
    63.0 / 64.0,
    31.0 / 64.0,
    55.0 / 64.0,
    23.0 / 64.0,
    61.0 / 64.0,
    29.0 / 64.0,
    53.0 / 64.0,
    21.0 / 64.0,
];
const DITHER_THRESHOLD_BAYER4X4: [f32; 16] = [
    0.0 / 16.0,
    8.0 / 16.0,
    2.0 / 16.0,
    10.0 / 16.0,
    12.0 / 16.0,
    4.0 / 16.0,
    14.0 / 16.0,
    6.0 / 16.0,
    3.0 / 16.0,
    11.0 / 16.0,
    1.0 / 16.0,
    9.0 / 16.0,
    15.0 / 16.0,
    7.0 / 16.0,
    13.0 / 16.0,
    5.0 / 16.0,
];
const DITHER_THRESHOLD_BAYER2X2: [f32; 4] = [0.0 / 4.0, 2.0 / 4.0, 3.0 / 4.0, 1.0 / 4.0];
const DITHER_THRESHOLD_CLUSTER8X8: [f32; 64] = [
    24.0 / 64.0,
    10.0 / 64.0,
    12.0 / 64.0,
    26.0 / 64.0,
    35.0 / 64.0,
    47.0 / 64.0,
    49.0 / 64.0,
    37.0 / 64.0,
    8.0 / 64.0,
    0.0 / 64.0,
    2.0 / 64.0,
    14.0 / 64.0,
    45.0 / 64.0,
    59.0 / 64.0,
    61.0 / 64.0,
    51.0 / 64.0,
    22.0 / 64.0,
    6.0 / 64.0,
    4.0 / 64.0,
    16.0 / 64.0,
    43.0 / 64.0,
    57.0 / 64.0,
    63.0 / 64.0,
    53.0 / 64.0,
    30.0 / 64.0,
    20.0 / 64.0,
    18.0 / 64.0,
    28.0 / 64.0,
    33.0 / 64.0,
    41.0 / 64.0,
    55.0 / 64.0,
    39.0 / 64.0,
    34.0 / 64.0,
    46.0 / 64.0,
    48.0 / 64.0,
    36.0 / 64.0,
    25.0 / 64.0,
    11.0 / 64.0,
    13.0 / 64.0,
    27.0 / 64.0,
    44.0 / 64.0,
    58.0 / 64.0,
    60.0 / 64.0,
    50.0 / 64.0,
    9.0 / 64.0,
    1.0 / 64.0,
    3.0 / 64.0,
    15.0 / 64.0,
    42.0 / 64.0,
    56.0 / 64.0,
    62.0 / 64.0,
    52.0 / 64.0,
    23.0 / 64.0,
    7.0 / 64.0,
    5.0 / 64.0,
    17.0 / 64.0,
    32.0 / 64.0,
    40.0 / 64.0,
    54.0 / 64.0,
    38.0 / 64.0,
    31.0 / 64.0,
    21.0 / 64.0,
    19.0 / 64.0,
    29.0 / 64.0,
];
const DITHER_THRESHOLD_CLUSTER4X4: [f32; 16] = [
    12.0 / 16.0,
    5.0 / 16.0,
    6.0 / 16.0,
    13.0 / 16.0,
    4.0 / 16.0,
    0.0 / 16.0,
    1.0 / 16.0,
    7.0 / 16.0,
    11.0 / 16.0,
    3.0 / 16.0,
    2.0 / 16.0,
    8.0 / 16.0,
    15.0 / 16.0,
    10.0 / 16.0,
    9.0 / 16.0,
    14.0 / 16.0,
];

#[derive(Default, Deserialize, Serialize)]
pub enum SampleMode {
    #[default]
    Round,
    Floor,
    Ceiling,
    Linear,
    Bicubic,
    Lanczos,
}

#[derive(Default, Deserialize, Serialize)]
pub enum ProcessMode {
    #[default]
    None,
    Bayer8x8,
    Bayer4x4,
    Bayer2x2,
    Cluster8x8,
    Cluster4x4,
    FloydComponent,
    FlotdDistributed
}

#[derive(Default, Deserialize, Serialize)]
pub enum DistanceMode {
    #[default]
    KMeans,
    RGB,
    CIE76,
    CIE94,
    CIEDE200,
    XYZ,
    YCC,
    YIQ,
    YUV
}

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
    pub pixel_process_mode: ProcessMode,
    pub pixel_distance_mode: DistanceMode,
    pub image_out_width: i32,
    pub image_out_height: i32,
    pub image_out_swidth: i32,
    pub image_out_sheight: i32,
    pub palette_weight: i32,
    pub palette: Vec<Color>,
    pub quant_cluster_list: Vec<Vec<Color>>,
    pub quant_centroid_list: Vec<Color>,
    pub quant_assignment: Vec<i32>,
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
            pixel_process_mode: Default::default(),
            pixel_distance_mode: Default::default(),
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

#[derive(Clone)]
pub struct Sprite {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Color>,
}

impl Sprite {
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Color> {
        self.data.get(y * self.width + x).copied()
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if let Some(col) = self.data.get_mut(y * self.width + x) {
            *col = color;
        }
    }
}

pub fn process_image(s: &mut I2PState, input: &Sprite, output: &mut Sprite) {
    let mut temp = sample_image(s, input, output.width, output.height);
    let gamma_factor = s.img_gamma as f32 / 100.0;
    let contrast_factor =
        (259.0 * (255.0 + s.contrast as f32)) / (255.0 * (259.0 - s.contrast as f32));
    let saturation_factor = s.saturation as f32 / 100.0;
    let brightness_factor = s.brightness as f32 / 100.0;

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

            let r = input.red as f32;
            let g = input.green as f32;
            let b = input.blue as f32;
            input.red = 0x0.max(0xff.min(((rr * r) + (gr * g) + (br * b) + wr) as u8));
            input.green = 0x0.max(0xff.min(((rg * r) + (gg * g) + (bg * b) + wg) as u8));
            input.blue = 0x0.max(0xff.min(((rb * r) + (gb * g) + (bb * b) + wb) as u8));

            if s.img_gamma != 100 {
                input.red = 0x0
                    .max(0xff.min((255.0 * (input.red as f32 / 255.0).powf(gamma_factor)) as u8));
                input.green = 0x0
                    .max(0xff.min((255.0 * (input.green as f32 / 255.0).powf(gamma_factor)) as u8));
                input.blue = 0x0
                    .max(0xff.min((255.0 * (input.blue as f32 / 255.0).powf(gamma_factor)) as u8));
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

fn sample_image(s: &I2PState, input: &Sprite, width: usize, height: usize) -> Vec<Color> {
    match s.pixel_sample_mode {
        SampleMode::Round => sample_round(s, input, width, height),
        SampleMode::Floor => sample_floor(s, input, width, height),
        SampleMode::Ceiling => sample_ceil(s, input, width, height),
        SampleMode::Linear => sample_linear(s, input, width, height),
        SampleMode::Bicubic => sample_bicubic(s, input, width, height),
        SampleMode::Lanczos => sample_lanczos(s, input, width, height),
    }
}

fn sample_round(s: &I2PState, input: &Sprite, width: usize, height: usize) -> Vec<Color> {
    let mut output = Vec::with_capacity(width * height);
    let w = (input.width - 1) as f64 / width as f64;
    let h = (input.height - 1) as f64 / height as f64;
    let off_x = s.offset_x as f64 / 100.0;
    let off_y = s.offset_y as f64 / 100.0;
    for y in 0..height {
        for x in 0..width {
            let dx = x as f64 + off_x;
            let dy = y as f64 + off_y;

            output.push(
                input
                    .get_pixel((dx * w).round() as usize, (dy * h).round() as usize)
                    .unwrap_or_default(),
            );
        }
    }
    output
}

fn sample_floor(s: &I2PState, input: &Sprite, width: usize, height: usize) -> Vec<Color> {
    let mut output = Vec::with_capacity(width * height);
    let w = (input.width - 1) as f64 / width as f64;
    let h = (input.height - 1) as f64 / height as f64;
    let off_x = s.offset_x as f64 / 100.0;
    let off_y = s.offset_y as f64 / 100.0;
    for y in 0..height {
        for x in 0..width {
            let dx = x as f64 + off_x;
            let dy = y as f64 + off_y;

            output.push(
                input
                    .get_pixel((dx * w).floor() as usize, (dy * h).floor() as usize)
                    .unwrap_or_default(),
            );
        }
    }
    output
}

fn sample_ceil(s: &I2PState, input: &Sprite, width: usize, height: usize) -> Vec<Color> {
    let mut output = Vec::with_capacity(width * height);
    let w = (input.width - 1) as f64 / width as f64;
    let h = (input.height - 1) as f64 / height as f64;
    let off_x = s.offset_x as f64 / 100.0;
    let off_y = s.offset_y as f64 / 100.0;
    for y in 0..height {
        for x in 0..width {
            let dx = x as f64 + off_x;
            let dy = y as f64 + off_y;

            output.push(
                input
                    .get_pixel((dx * w).ceil() as usize, (dy * h).ceil() as usize)
                    .unwrap_or_default(),
            );
        }
    }
    output
}

fn sample_linear(s: &I2PState, input: &Sprite, width: usize, height: usize) -> Vec<Color> {
    let mut output = Vec::with_capacity(width * height);
    let f_w = (input.width - 1) as f32 / width as f32;
    let f_h = (input.height - 1) as f32 / height as f32;
    let f_off_x = s.offset_x as f32 / 100.0;
    let f_off_y = s.offset_y as f32 / 100.0;
    for y in 0..height {
        for x in 0..width {
            let ix = ((x as f32 + f_off_x) * f_w) as usize;
            let iy = ((y as f32 + f_off_y) * f_h) as usize;
            let six = ((x as f32 + f_off_x) * f_w) - ix as f32;
            let siy = ((y as f32 + f_off_y) * f_h) - iy as f32;

            let mut c: Color = Default::default();

            let c1 = input.get_pixel(ix, iy).unwrap_or_default();
            let c2 = input.get_pixel(ix + 1, iy).unwrap_or_default();
            let c3 = input.get_pixel(ix, iy + 1).unwrap_or_default();
            let c4 = input.get_pixel(ix + 1, iy + 1).unwrap_or_default();

            let c1t = (1.0 - six) * c1.red as f32 + six * c2.red as f32;
            let c2t = (1.0 - six) * c3.red as f32 + six * c4.red as f32;
            c.red = ((1.0 - siy) * c1t + siy * c2t) as u8;

            let c1t = (1.0 - six) * c1.green as f32 + six * c2.green as f32;
            let c2t = (1.0 - six) * c3.green as f32 + six * c4.green as f32;
            c.green = ((1.0 - siy) * c1t + siy * c2t) as u8;

            let c1t = (1.0 - six) * c1.blue as f32 + six * c2.blue as f32;
            let c2t = (1.0 - six) * c3.blue as f32 + six * c4.blue as f32;
            c.blue = ((1.0 - siy) * c1t + siy * c2t) as u8;

            let c1t = (1.0 - six) * c1.alpha as f32 + six * c2.alpha as f32;
            let c2t = (1.0 - six) * c3.alpha as f32 + six * c4.alpha as f32;
            c.alpha = ((1.0 - siy) * c1t + siy * c2t) as u8;

            output.push(c);
        }
    }
    output
}

fn sample_bicubic(s: &I2PState, input: &Sprite, width: usize, height: usize) -> Vec<Color> {
    let mut output = Vec::with_capacity(width * height);
    let f_w = (input.width - 1) as f32 / width as f32;
    let f_h = (input.height - 1) as f32 / height as f32;
    let f_off_x = s.offset_x as f32 / 100.0;
    let f_off_y = s.offset_y as f32 / 100.0;
    for y in 0..height {
        for x in 0..width {
            let ix = ((x as f32 + f_off_x) * f_w) as usize;
            let iy = ((y as f32 + f_off_y) * f_h) as usize;
            let six = ((x as f32 + f_off_x) * f_w) - ix as f32;
            let siy = ((y as f32 + f_off_y) * f_h) - iy as f32;

            let mut c: Color = Default::default();

            let c00 = input.get_pixel(0.max(ix as isize - 1) as usize, 0.max(iy as isize - 1) as usize).unwrap_or_default();
            let c10 = input.get_pixel(ix, 0.max(iy as isize - 1) as usize).unwrap_or_default();
            let c20 = input.get_pixel(ix + 1, 0.max(iy as isize - 1) as usize).unwrap_or_default();
            let c30 = input.get_pixel(ix + 2, 0.max(iy as isize - 1) as usize).unwrap_or_default();

            let c01 = input.get_pixel(0.max(ix as isize - 1) as usize, iy).unwrap_or_default();
            let c11 = input.get_pixel(ix, iy).unwrap_or_default();
            let c21 = input.get_pixel(ix + 1, iy).unwrap_or_default();
            let c31 = input.get_pixel(ix + 2, iy).unwrap_or_default();
            
            let c02 = input.get_pixel(0.max(ix as isize - 1) as usize, iy + 1).unwrap_or_default();
            let c12 = input.get_pixel(ix, iy + 1).unwrap_or_default();
            let c22 = input.get_pixel(ix + 1, iy + 1).unwrap_or_default();
            let c32 = input.get_pixel(ix + 2, iy + 1).unwrap_or_default();
            
            let c03 = input.get_pixel(0.max(ix as isize - 1) as usize, iy + 2).unwrap_or_default();
            let c13 = input.get_pixel(ix, iy + 2).unwrap_or_default();
            let c23 = input.get_pixel(ix + 1, iy + 2).unwrap_or_default();
            let c33 = input.get_pixel(ix + 2, iy + 2).unwrap_or_default();

            let c0 = cubic_hermite(c00.red as f32, c10.red as f32, c20.red as f32, c30.red as f32, six);
            let c1 = cubic_hermite(c01.red as f32, c11.red as f32, c21.red as f32, c31.red as f32, six);
            let c2 = cubic_hermite(c02.red as f32, c12.red as f32, c22.red as f32, c32.red as f32, six);
            let c3 = cubic_hermite(c03.red as f32, c13.red as f32, c23.red as f32, c33.red as f32, six);
            let val = cubic_hermite(c0, c1, c2, c3, siy);
            c.red = 0x0.max(0xff.min(val as usize)) as u8;

            let c0 = cubic_hermite(c00.green as f32, c10.green as f32, c20.green as f32, c30.green as f32, six);
            let c1 = cubic_hermite(c01.green as f32, c11.green as f32, c21.green as f32, c31.green as f32, six);
            let c2 = cubic_hermite(c02.green as f32, c12.green as f32, c22.green as f32, c32.green as f32, six);
            let c3 = cubic_hermite(c03.green as f32, c13.green as f32, c23.green as f32, c33.green as f32, six);
            let val = cubic_hermite(c0, c1 ,c2, c3, siy);
            c.green = 0x0.max(0xff.min(val as usize)) as u8;

            let c0 = cubic_hermite(c00.blue as f32, c10.blue as f32, c20.blue as f32, c30.blue as f32, six);
            let c1 = cubic_hermite(c01.blue as f32, c11.blue as f32, c21.blue as f32, c31.blue as f32, six);
            let c2 = cubic_hermite(c02.blue as f32, c12.blue as f32, c22.blue as f32, c32.blue as f32, six);
            let c3 = cubic_hermite(c03.blue as f32, c13.blue as f32, c23.blue as f32, c33.blue as f32, six);
            let val = cubic_hermite(c0, c1, c2, c3, siy);
            c.blue = 0x0.max(0xff.min(val as usize)) as u8;

            let c0 = cubic_hermite(c00.alpha as f32, c10.alpha as f32, c20.alpha as f32, c30.alpha as f32, six);
            let c1 = cubic_hermite(c01.alpha as f32, c11.alpha as f32, c21.alpha as f32, c31.alpha as f32, six);
            let c2 = cubic_hermite(c02.alpha as f32, c12.alpha as f32, c22.alpha as f32, c32.alpha as f32, six);
            let c3 = cubic_hermite(c03.alpha as f32, c13.alpha as f32, c23.alpha as f32, c33.alpha as f32, six);
            let val = cubic_hermite(c0, c1, c2, c3, siy);
            c.alpha = 0x0.max(0xff.min(val as usize)) as u8;

            output.push(c);
        }
    }
    output
}

fn cubic_hermite(a: f32, b: f32, c: f32, d: f32, t: f32) -> f32 {
   let a_ = -a / 2.0 + (3.0 * b) / 2.0 - (3.0 * c) / 2.0 + d / 2.0;   
   let b_ = a - (5.0 * b) / 2.0 + 2.0 * c - d / 2.0; 
   let c_ = -a / 2.0 + c / 2.0;
   let d_ = b;

   return a_ * t * t * t + b_ * t * t + c_ * t + d_;
}

fn sample_lanczos(s: &I2PState, input: &Sprite, width: usize, height: usize) -> Vec<Color> {
    let mut output = Vec::with_capacity(width * height);
    let f_w = (input.width - 1) as f64 / width as f64;
    let f_h = (input.height - 1) as f64 / height as f64;
    let f_off_x = s.offset_x as f32 / 100.0;
    let f_off_y = s.offset_y as f32 / 100.0;
    for y in 0..height {
        for x in 0..width {
            let ix = ((x as f64 + f_off_x as f64) * f_w) as usize;
            let iy = ((y as f64 + f_off_y as f64) * f_h) as usize;
            let sx = ((x as f64 + f_off_x as f64) * f_w) - ix as f64;
            let sy = ((y as f64 + f_off_y as f64) * f_h) - iy as f64;

            let mut c: Color = Default::default();

            let a0 = lanczos(sx + 2.0);
            let a1 = lanczos(sx + 1.0);
            let a2 = lanczos(sx);
            let a3 = lanczos(sx - 1.0);
            let a4 = lanczos(sx - 2.0);
            let a5 = lanczos(sx - 3.0);
            let b0 = lanczos(sy + 2.0);
            let b1 = lanczos(sy + 1.0);
            let b2 = lanczos(sy);
            let b3 = lanczos(sy - 1.0);
            let b4 = lanczos(sy - 2.0);
            let b5 = lanczos(sy - 3.0);

            let mut r = [0.0; 6];
            let mut g = [0.0; 6];
            let mut b = [0.0; 6];
            let mut a = [0.0; 6];

            for i in 0..6 {
                let p0 = input.get_pixel(0.max(ix as isize - 2) as usize, 0.max(iy as isize - 2) as usize + i).unwrap_or_default();
                let p1 = input.get_pixel(0.max(ix as isize - 1) as usize, 0.max(iy as isize - 2) as usize + i).unwrap_or_default();
                let p2 = input.get_pixel(ix, 0.max(iy as isize - 2) as usize + i).unwrap_or_default();
                let p3 = input.get_pixel(ix + 1, 0.max(iy as isize - 2) as usize + i).unwrap_or_default();
                let p4 = input.get_pixel(ix + 2, 0.max(iy as isize - 2) as usize + i).unwrap_or_default();
                let p5 = input.get_pixel(ix + 3, 0.max(iy as isize - 2) as usize + i).unwrap_or_default();

                r[i] = a0 * p0.red as f64 + a1 * p1.red as f64 + a2 * p2.red as f64 + a3 * p3.red as f64 + a4 * p4.red as f64 + a5 * p5.red as f64;
                g[i] = a0 * p0.green as f64 + a1 * p1.green as f64 + a2 * p2.green as f64 + a3 * p3.green as f64 + a4 * p4.green as f64 + a5 * p5.green as f64;
                b[i] = a0 * p0.blue as f64 + a1 * p1.blue as f64 + a2 * p2.blue as f64 + a3 * p3.blue as f64 + a4 * p4.blue as f64 + a5 * p5.blue as f64;
                a[i] = a0 * p0.alpha as f64 + a1 * p1.alpha as f64 + a2 * p2.alpha as f64 + a3 * p3.alpha as f64 + a4 * p4.alpha as f64 + a5 * p5.alpha as f64;
            }

            c.red = 0x0.max(0xff.min((b0 * r[0] + b1 * r[1] + b2 * r[2] + b3 * r[3] + b4 * r[4] + b5 * r[5]) as usize)) as u8;
            c.green = 0x0.max(0xff.min((b0 * g[0] + b1 * g[1] + b2 * g[2] + b3 * g[3] + b4 * g[4] + b5 * g[5]) as usize)) as u8;
            c.blue = 0x0.max(0xff.min((b0 * b[0] + b1 * b[1] + b2 * b[2] + b3 * b[3] + b4 * b[4] + b5 * b[5]) as usize)) as u8;
            c.alpha = 0x0.max(0xff.min((b0 * a[0] + b1 * a[1] + b2 * a[2] + b3 * a[3] + b4 * a[4] + b5 * a[5]) as usize)) as u8;

            output.push(c);
        }
    }
    output
}

fn lanczos(v: f64) -> f64 {
    if v == 0.0 {
        return 1.0;
    }
    if v > 3.0 {
        return 0.0;
    }
    if v < -3.0 {
        return 0.0;
    }
    (3.0 * f64::sin(PI * v) * f64::sin(PI * v / 3.0)) / (PI * PI * v * v)
}

fn dither_image(
    state: &mut I2PState,
    input: Vec<Color>,
    output: &mut Sprite,
    width: usize,
    height: usize,
) {
    match state.pixel_distance_mode {
        DistanceMode::KMeans => {
            dither_kmeans(state, input, output, width, height);
        }
        DistanceMode::RGB => todo!(),
        DistanceMode::CIE76 => todo!(),
        DistanceMode::CIE94 => todo!(),
        DistanceMode::CIEDE200 => todo!(),
        DistanceMode::XYZ => todo!(),
        DistanceMode::YCC => todo!(),
        DistanceMode::YIQ => todo!(),
        DistanceMode::YUV => todo!(),
    }
}

fn dither_kmeans(state: &mut I2PState, input: Vec<palette::Alpha<palette::rgb::Rgb<palette::rgb::Rgb, u8>, u8>>, output: &mut Sprite, width: usize, height: usize) {
    match state.pixel_process_mode {
        ProcessMode::Bayer8x8 => dither_threshold_apply(
            state,
            input,
            &mut output.data,
            width,
            height,
            &DITHER_THRESHOLD_BAYER8X8,
            3,
        ),
        ProcessMode::Bayer4x4 => dither_threshold_apply(
            state,
            input,
            &mut output.data,
            width,
            height,
            &DITHER_THRESHOLD_BAYER4X4,
            2,
        ),
        ProcessMode::Bayer2x2 => dither_threshold_apply(
            state,
            input,
            &mut output.data,
            width,
            height,
            &DITHER_THRESHOLD_BAYER2X2,
            1,
        ),
        ProcessMode::Cluster8x8 => dither_threshold_apply(
            state,
            input,
            &mut output.data,
            width,
            height,
            &DITHER_THRESHOLD_CLUSTER8X8,
            3,
        ),
        ProcessMode::Cluster4x4 => dither_threshold_apply(
            state,
            input,
            &mut output.data,
            width,
            height,
            &DITHER_THRESHOLD_CLUSTER4X4,
            2,
        ),
        ProcessMode::None | ProcessMode::FlotdDistributed | ProcessMode::FloydComponent => dither_none_apply(state, input, &mut output.data, width, height),
    }

    state.quant_k = state.palette.len() as i32;
    let temp = output.clone();
    quant_compute_kmeans(state, &temp, 1);

    for i in 0..width * height {
        if output.data[i].alpha == 0 {
            output.data[i] = Default::default();
        } else {
            output.data[i] = state.palette[state.quant_assignment[i] as usize];
        }
    }
}

fn dither_none_apply(
    state: &mut I2PState,
    input: Vec<Color>,
    output: &mut [Color],
    _width: usize,
    _height: usize,
) {
    for (cin, output) in input.iter().zip(output) {
            if cin.alpha < state.alpha_threshold as u8 {
                 *output = Default::default();
                 continue;
            }

            *output = *cin;
            output.alpha = 255;
    }
}

fn dither_threshold_apply(
    state: &I2PState,
    input: Vec<Color>,
    output: &mut [Color],
    width: usize,
    height: usize,
    threshold: &[f32],
    dim: u8,
) {
    let amount = state.dither_amount as f32 / 1000.0;

    for y in 0..height {
        for x in 0..width {
            let input = input[y * width + x];
            if input.alpha < state.alpha_threshold as u8 {
                output[y * width + x] = Default::default();
                continue;
            }

            let r#mod = (1 << dim) - 1;
            let threshold_id = ((y & r#mod) << dim) + (x & r#mod);
            let c = Color::new(
                0x0.max(0xff.min(
                    (input.red as f32 + 255.0 * amount * (threshold[threshold_id] - 0.5)) as u8,
                )),
                0x0.max(0xff.min(
                    (input.green as f32 + 255.0 * amount * (threshold[threshold_id] - 0.5)) as u8,
                )),
                0x0.max(0xff.min(
                    (input.blue as f32 + 255.0 * amount * (threshold[threshold_id] - 0.5)) as u8,
                )),
                255,
            );
            output[y * width + x] = c;
        }
    }
}

fn quant_compute_kmeans(state: &mut I2PState, data: &Sprite, pal_in: i32) {
    state.quant_cluster_list.shrink_to(0);
    state.quant_cluster_list.resize(state.quant_k as usize, Default::default());
    state.quant_centroid_list.shrink_to(0);
    state.quant_centroid_list.resize(state.quant_k as usize, Default::default());
    state.quant_assignment.shrink_to(0);
    state.quant_assignment.resize(data.width * data.height, 0);
    let mut iter = 0;
    let max_iter = 16;
    let mut previous_variance = vec![1.0; state.quant_k as usize];
    let mut variance: f64;
    let mut delta: f64;
    let mut delta_max: f64;
    let threshold = 0.00005;

    loop {
        quant_get_cluster_centroid(state, data, pal_in, 1 << state.palette_weight);
        state.quant_cluster_list.shrink_to(0);
        state.quant_cluster_list.resize(state.quant_k as usize, Default::default());
        for i in 0..data.width * data.height {
            let color = data.data[i];
            state.quant_assignment[i] =
                quant_nearest_color_idx(state, color, &state.quant_centroid_list);
            state.quant_cluster_list[state.quant_assignment[i] as usize].push(color);
        }

        delta_max = 0.0;
        for i in 0..state.quant_k {
            variance = quant_colors_variance(&state.quant_cluster_list[i as usize]);
            delta = (previous_variance[i as usize] - variance).abs();
            delta_max = delta_max.max(delta);
            previous_variance[i as usize] = variance;
        }

        iter += 1;
        if delta_max < threshold || iter > max_iter {
            break;
        }
    }
}

fn quant_colors_variance(color_list: &[Color]) -> f64 {
    let length = color_list.len();
    let mean = quant_colors_mean(color_list, Color::default(), 0);
    let dist_sum: f64 = color_list
        .iter()
        .map(|c| {
            let dist = quant_distance(&c, mean);
            dist * dist
        })
        .sum();

    dist_sum / length as f64
}

fn quant_nearest_color_idx(state: &I2PState, color: Color, color_list: &[Color]) -> i32 {
    let mut dist_min = 4095.0;
    let mut dist: f64;
    let mut idx = 0;

    for i in 0..state.quant_k {
        dist = quant_distance(&color, color_list[i as usize]);
        if dist < dist_min {
            dist_min = dist;
            idx = i;
        }
    }

    idx
}

fn quant_distance(color0: &Color, color1: Color) -> f64 {
    let mr = 0.5 * (color0.red as f64 + color1.red as f64);
    let dr = color0.red as f64 - color1.red as f64;
    let dg = color0.green as f64 - color1.green as f64;
    let db = color0.blue as f64 - color1.blue as f64;
    let distance = (2.0 * dr * dr)
        + (4.0 * dg * dg)
        + (3.0 * db * db)
        + (mr * ((dr * dr) - (db * db)) / 256.0);
    distance.sqrt() / (3.0 * 255.0)
}

fn quant_get_cluster_centroid(
    state: &mut I2PState,
    data: &Sprite,
    pal_in: i32,
    palette_weight: i32,
) {
    for i in 0..state.quant_k {
        if !state.quant_cluster_list[i as usize].is_empty() {
            if pal_in != 0 {
                state.quant_centroid_list[i as usize] = quant_colors_mean(
                    &state.quant_cluster_list[i as usize],
                    state.palette[i as usize],
                    palette_weight,
                );
            } else {
                state.quant_centroid_list[i as usize] =
                    quant_colors_mean(&state.quant_cluster_list[i as usize], Default::default(), 0);
            }
        } else if pal_in != 0 {
            state.quant_centroid_list[i as usize] = state.palette[i as usize];
        } else {
            state.quant_centroid_list[i as usize] = quant_pick_random_color(data);
        }
    }
}

fn quant_pick_random_color(data: &Sprite) -> Color {
    data.data
        .as_slice()
        .choose(&mut thread_rng())
        .copied()
        .unwrap_or_default()
}

fn quant_colors_mean(color_list: &[Color], color: Color, palette_weight: i32) -> Color {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;

    for color in color_list {
        r += color.red as i32;
        g += color.green as i32;
        b += color.green as i32;
    }

    let mut weight_color = palette_weight;
    if weight_color != 0 {
        weight_color = color_list.len() as i32 / weight_color;
    }
    let length = color_list.len() as i32 + weight_color;
    r += color.red as i32 * weight_color;
    g += color.green as i32 * weight_color;
    b += color.blue as i32 * weight_color;

    if length != 0 {
        r /= length;
        g /= length;
        b /= length;
    }

    Color::new(r as u8, g as u8, b as u8, 255)
}
