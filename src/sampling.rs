use std::f64::consts::PI;

use serde::{Deserialize, Serialize};

use crate::{sprite::Sprite, Color, I2PState};

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

pub fn sample_image(s: &I2PState, input: &Sprite, width: usize, height: usize) -> Vec<Color> {
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

            let c00 = input
                .get_pixel(
                    0.max(ix as isize - 1) as usize,
                    0.max(iy as isize - 1) as usize,
                )
                .unwrap_or_default();
            let c10 = input
                .get_pixel(ix, 0.max(iy as isize - 1) as usize)
                .unwrap_or_default();
            let c20 = input
                .get_pixel(ix + 1, 0.max(iy as isize - 1) as usize)
                .unwrap_or_default();
            let c30 = input
                .get_pixel(ix + 2, 0.max(iy as isize - 1) as usize)
                .unwrap_or_default();

            let c01 = input
                .get_pixel(0.max(ix as isize - 1) as usize, iy)
                .unwrap_or_default();
            let c11 = input.get_pixel(ix, iy).unwrap_or_default();
            let c21 = input.get_pixel(ix + 1, iy).unwrap_or_default();
            let c31 = input.get_pixel(ix + 2, iy).unwrap_or_default();

            let c02 = input
                .get_pixel(0.max(ix as isize - 1) as usize, iy + 1)
                .unwrap_or_default();
            let c12 = input.get_pixel(ix, iy + 1).unwrap_or_default();
            let c22 = input.get_pixel(ix + 1, iy + 1).unwrap_or_default();
            let c32 = input.get_pixel(ix + 2, iy + 1).unwrap_or_default();

            let c03 = input
                .get_pixel(0.max(ix as isize - 1) as usize, iy + 2)
                .unwrap_or_default();
            let c13 = input.get_pixel(ix, iy + 2).unwrap_or_default();
            let c23 = input.get_pixel(ix + 1, iy + 2).unwrap_or_default();
            let c33 = input.get_pixel(ix + 2, iy + 2).unwrap_or_default();

            let c0 = cubic_hermite(
                c00.red as f32,
                c10.red as f32,
                c20.red as f32,
                c30.red as f32,
                six,
            );
            let c1 = cubic_hermite(
                c01.red as f32,
                c11.red as f32,
                c21.red as f32,
                c31.red as f32,
                six,
            );
            let c2 = cubic_hermite(
                c02.red as f32,
                c12.red as f32,
                c22.red as f32,
                c32.red as f32,
                six,
            );
            let c3 = cubic_hermite(
                c03.red as f32,
                c13.red as f32,
                c23.red as f32,
                c33.red as f32,
                six,
            );
            let val = cubic_hermite(c0, c1, c2, c3, siy);
            c.red = 0x0.max(0xff.min(val as usize)) as u8;

            let c0 = cubic_hermite(
                c00.green as f32,
                c10.green as f32,
                c20.green as f32,
                c30.green as f32,
                six,
            );
            let c1 = cubic_hermite(
                c01.green as f32,
                c11.green as f32,
                c21.green as f32,
                c31.green as f32,
                six,
            );
            let c2 = cubic_hermite(
                c02.green as f32,
                c12.green as f32,
                c22.green as f32,
                c32.green as f32,
                six,
            );
            let c3 = cubic_hermite(
                c03.green as f32,
                c13.green as f32,
                c23.green as f32,
                c33.green as f32,
                six,
            );
            let val = cubic_hermite(c0, c1, c2, c3, siy);
            c.green = 0x0.max(0xff.min(val as usize)) as u8;

            let c0 = cubic_hermite(
                c00.blue as f32,
                c10.blue as f32,
                c20.blue as f32,
                c30.blue as f32,
                six,
            );
            let c1 = cubic_hermite(
                c01.blue as f32,
                c11.blue as f32,
                c21.blue as f32,
                c31.blue as f32,
                six,
            );
            let c2 = cubic_hermite(
                c02.blue as f32,
                c12.blue as f32,
                c22.blue as f32,
                c32.blue as f32,
                six,
            );
            let c3 = cubic_hermite(
                c03.blue as f32,
                c13.blue as f32,
                c23.blue as f32,
                c33.blue as f32,
                six,
            );
            let val = cubic_hermite(c0, c1, c2, c3, siy);
            c.blue = 0x0.max(0xff.min(val as usize)) as u8;

            let c0 = cubic_hermite(
                c00.alpha as f32,
                c10.alpha as f32,
                c20.alpha as f32,
                c30.alpha as f32,
                six,
            );
            let c1 = cubic_hermite(
                c01.alpha as f32,
                c11.alpha as f32,
                c21.alpha as f32,
                c31.alpha as f32,
                six,
            );
            let c2 = cubic_hermite(
                c02.alpha as f32,
                c12.alpha as f32,
                c22.alpha as f32,
                c32.alpha as f32,
                six,
            );
            let c3 = cubic_hermite(
                c03.alpha as f32,
                c13.alpha as f32,
                c23.alpha as f32,
                c33.alpha as f32,
                six,
            );
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

    a_ * t * t * t + b_ * t * t + c_ * t + d_
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
                let p0 = input
                    .get_pixel(
                        0.max(ix as isize - 2) as usize,
                        0.max(iy as isize - 2) as usize + i,
                    )
                    .unwrap_or_default();
                let p1 = input
                    .get_pixel(
                        0.max(ix as isize - 1) as usize,
                        0.max(iy as isize - 2) as usize + i,
                    )
                    .unwrap_or_default();
                let p2 = input
                    .get_pixel(ix, 0.max(iy as isize - 2) as usize + i)
                    .unwrap_or_default();
                let p3 = input
                    .get_pixel(ix + 1, 0.max(iy as isize - 2) as usize + i)
                    .unwrap_or_default();
                let p4 = input
                    .get_pixel(ix + 2, 0.max(iy as isize - 2) as usize + i)
                    .unwrap_or_default();
                let p5 = input
                    .get_pixel(ix + 3, 0.max(iy as isize - 2) as usize + i)
                    .unwrap_or_default();

                r[i] = a0 * p0.red as f64
                    + a1 * p1.red as f64
                    + a2 * p2.red as f64
                    + a3 * p3.red as f64
                    + a4 * p4.red as f64
                    + a5 * p5.red as f64;
                g[i] = a0 * p0.green as f64
                    + a1 * p1.green as f64
                    + a2 * p2.green as f64
                    + a3 * p3.green as f64
                    + a4 * p4.green as f64
                    + a5 * p5.green as f64;
                b[i] = a0 * p0.blue as f64
                    + a1 * p1.blue as f64
                    + a2 * p2.blue as f64
                    + a3 * p3.blue as f64
                    + a4 * p4.blue as f64
                    + a5 * p5.blue as f64;
                a[i] = a0 * p0.alpha as f64
                    + a1 * p1.alpha as f64
                    + a2 * p2.alpha as f64
                    + a3 * p3.alpha as f64
                    + a4 * p4.alpha as f64
                    + a5 * p5.alpha as f64;
            }

            c.red = 0x0.max(0xff.min(
                (b0 * r[0] + b1 * r[1] + b2 * r[2] + b3 * r[3] + b4 * r[4] + b5 * r[5]) as usize,
            )) as u8;
            c.green = 0x0.max(0xff.min(
                (b0 * g[0] + b1 * g[1] + b2 * g[2] + b3 * g[3] + b4 * g[4] + b5 * g[5]) as usize,
            )) as u8;
            c.blue = 0x0.max(0xff.min(
                (b0 * b[0] + b1 * b[1] + b2 * b[2] + b3 * b[3] + b4 * b[4] + b5 * b[5]) as usize,
            )) as u8;
            c.alpha = 0x0.max(0xff.min(
                (b0 * a[0] + b1 * a[1] + b2 * a[2] + b3 * a[3] + b4 * a[4] + b5 * a[5]) as usize,
            )) as u8;

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
