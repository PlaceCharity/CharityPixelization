#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]
// the sort of cast lints im allowing in this file are problems if we're dealing with massive images, which isn't going to happen

use std::f64::consts::PI;

use crate::{sprite::Sprite, Color, I2PState};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[derive(Default, Clone, Copy)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
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
    match s.sample_options.sample_mode {
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
    let off_x = f64::from(s.sample_options.offset_x) / 100.0;
    let off_y = f64::from(s.sample_options.offset_y) / 100.0;
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
    let off_x = f64::from(s.sample_options.offset_x) / 100.0;
    let off_y = f64::from(s.sample_options.offset_y) / 100.0;
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
    let off_x = f64::from(s.sample_options.offset_x) / 100.0;
    let off_y = f64::from(s.sample_options.offset_y) / 100.0;
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

#[allow(clippy::similar_names)]
fn sample_linear(s: &I2PState, input: &Sprite, width: usize, height: usize) -> Vec<Color> {
    let mut output = Vec::with_capacity(width * height);
    let f_w = (input.width - 1) as f32 / width as f32;
    let f_h = (input.height - 1) as f32 / height as f32;
    let f_off_x = s.sample_options.offset_x as f32 / 100.0;
    let f_off_y = s.sample_options.offset_y as f32 / 100.0;
    for y in 0..height {
        for x in 0..width {
            let ix = ((x as f32 + f_off_x) * f_w) as usize;
            let iy = ((y as f32 + f_off_y) * f_h) as usize;
            let s_ix = ((x as f32 + f_off_x) * f_w) - ix as f32;
            let s_iy = ((y as f32 + f_off_y) * f_h) - iy as f32;

            let mut c: Color = Color::default();

            let c1 = input.get_pixel(ix, iy).unwrap_or_default();
            let c2 = input.get_pixel(ix + 1, iy).unwrap_or_default();
            let c3 = input.get_pixel(ix, iy + 1).unwrap_or_default();
            let c4 = input.get_pixel(ix + 1, iy + 1).unwrap_or_default();

            let c1t = (1.0 - s_ix) * f32::from(c1.red) + s_ix * f32::from(c2.red);
            let c2t = (1.0 - s_ix) * f32::from(c3.red) + s_ix * f32::from(c4.red);
            c.red = ((1.0 - s_iy) * c1t + s_iy * c2t) as u8;

            let c1t = (1.0 - s_ix) * f32::from(c1.green) + s_ix * f32::from(c2.green);
            let c2t = (1.0 - s_ix) * f32::from(c3.green) + s_ix * f32::from(c4.green);
            c.green = ((1.0 - s_iy) * c1t + s_iy * c2t) as u8;

            let c1t = (1.0 - s_ix) * f32::from(c1.blue) + s_ix * f32::from(c2.blue);
            let c2t = (1.0 - s_ix) * f32::from(c3.blue) + s_ix * f32::from(c4.blue);
            c.blue = ((1.0 - s_iy) * c1t + s_iy * c2t) as u8;

            let c1t = (1.0 - s_ix) * f32::from(c1.alpha) + s_ix * f32::from(c2.alpha);
            let c2t = (1.0 - s_ix) * f32::from(c3.alpha) + s_ix * f32::from(c4.alpha);
            c.alpha = ((1.0 - s_iy) * c1t + s_iy * c2t) as u8;

            output.push(c);
        }
    }
    output
}

#[allow(clippy::too_many_lines, clippy::similar_names)]
fn sample_bicubic(s: &I2PState, input: &Sprite, width: usize, height: usize) -> Vec<Color> {
    let mut output = Vec::with_capacity(width * height);
    let f_w = (input.width - 1) as f32 / width as f32;
    let f_h = (input.height - 1) as f32 / height as f32;
    let f_off_x = s.sample_options.offset_x as f32 / 100.0;
    let f_off_y = s.sample_options.offset_y as f32 / 100.0;
    for y in 0..height {
        for x in 0..width {
            let ix = ((x as f32 + f_off_x) * f_w) as usize;
            let iy = ((y as f32 + f_off_y) * f_h) as usize;
            let six = ((x as f32 + f_off_x) * f_w) - ix as f32;
            let siy = ((y as f32 + f_off_y) * f_h) - iy as f32;

            let mut c: Color = Color::default();

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
                f32::from(c00.red),
                f32::from(c10.red),
                f32::from(c20.red),
                f32::from(c30.red),
                six,
            );
            let c1 = cubic_hermite(
                f32::from(c01.red),
                f32::from(c11.red),
                f32::from(c21.red),
                f32::from(c31.red),
                six,
            );
            let c2 = cubic_hermite(
                f32::from(c02.red),
                f32::from(c12.red),
                f32::from(c22.red),
                f32::from(c32.red),
                six,
            );
            let c3 = cubic_hermite(
                f32::from(c03.red),
                f32::from(c13.red),
                f32::from(c23.red),
                f32::from(c33.red),
                six,
            );
            let val = cubic_hermite(c0, c1, c2, c3, siy);
            c.red = 0x0.max(0xff.min(val as usize)) as u8;

            let c0 = cubic_hermite(
                f32::from(c00.green),
                f32::from(c10.green),
                f32::from(c20.green),
                f32::from(c30.green),
                six,
            );
            let c1 = cubic_hermite(
                f32::from(c01.green),
                f32::from(c11.green),
                f32::from(c21.green),
                f32::from(c31.green),
                six,
            );
            let c2 = cubic_hermite(
                f32::from(c02.green),
                f32::from(c12.green),
                f32::from(c22.green),
                f32::from(c32.green),
                six,
            );
            let c3 = cubic_hermite(
                f32::from(c03.green),
                f32::from(c13.green),
                f32::from(c23.green),
                f32::from(c33.green),
                six,
            );
            let val = cubic_hermite(c0, c1, c2, c3, siy);
            c.green = 0x0.max(0xff.min(val as usize)) as u8;

            let c0 = cubic_hermite(
                f32::from(c00.blue),
                f32::from(c10.blue),
                f32::from(c20.blue),
                f32::from(c30.blue),
                six,
            );
            let c1 = cubic_hermite(
                f32::from(c01.blue),
                f32::from(c11.blue),
                f32::from(c21.blue),
                f32::from(c31.blue),
                six,
            );
            let c2 = cubic_hermite(
                f32::from(c02.blue),
                f32::from(c12.blue),
                f32::from(c22.blue),
                f32::from(c32.blue),
                six,
            );
            let c3 = cubic_hermite(
                f32::from(c03.blue),
                f32::from(c13.blue),
                f32::from(c23.blue),
                f32::from(c33.blue),
                six,
            );
            let val = cubic_hermite(c0, c1, c2, c3, siy);
            c.blue = 0x0.max(0xff.min(val as usize)) as u8;

            let c0 = cubic_hermite(
                f32::from(c00.alpha),
                f32::from(c10.alpha),
                f32::from(c20.alpha),
                f32::from(c30.alpha),
                six,
            );
            let c1 = cubic_hermite(
                f32::from(c01.alpha),
                f32::from(c11.alpha),
                f32::from(c21.alpha),
                f32::from(c31.alpha),
                six,
            );
            let c2 = cubic_hermite(
                f32::from(c02.alpha),
                f32::from(c12.alpha),
                f32::from(c22.alpha),
                f32::from(c32.alpha),
                six,
            );
            let c3 = cubic_hermite(
                f32::from(c03.alpha),
                f32::from(c13.alpha),
                f32::from(c23.alpha),
                f32::from(c33.alpha),
                six,
            );
            let val = cubic_hermite(c0, c1, c2, c3, siy);
            c.alpha = 0x0.max(0xff.min(val as usize)) as u8;

            output.push(c);
        }
    }
    output
}

#[allow(clippy::many_single_char_names)]
fn cubic_hermite(a: f32, b: f32, c: f32, d: f32, t: f32) -> f32 {
    let a_ = -a / 2.0 + (3.0 * b) / 2.0 - (3.0 * c) / 2.0 + d / 2.0;
    let b_ = a - (5.0 * b) / 2.0 + 2.0 * c - d / 2.0;
    let c_ = -a / 2.0 + c / 2.0;
    let d_ = b;

    a_ * t * t * t + b_ * t * t + c_ * t + d_
}

#[allow(clippy::many_single_char_names)]
fn sample_lanczos(s: &I2PState, input: &Sprite, width: usize, height: usize) -> Vec<Color> {
    let mut output = Vec::with_capacity(width * height);
    let f_w = (input.width - 1) as f64 / width as f64;
    let f_h = (input.height - 1) as f64 / height as f64;
    let f_off_x = s.sample_options.offset_x as f32 / 100.0;
    let f_off_y = s.sample_options.offset_y as f32 / 100.0;
    for y in 0..height {
        for x in 0..width {
            let ix = ((x as f64 + f64::from(f_off_x)) * f_w) as usize;
            let iy = ((y as f64 + f64::from(f_off_y)) * f_h) as usize;
            let sx = ((x as f64 + f64::from(f_off_x)) * f_w) - ix as f64;
            let sy = ((y as f64 + f64::from(f_off_y)) * f_h) - iy as f64;

            let mut c = Color::default();

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

                r[i] = a0 * f64::from(p0.red)
                    + a1 * f64::from(p1.red)
                    + a2 * f64::from(p2.red)
                    + a3 * f64::from(p3.red)
                    + a4 * f64::from(p4.red)
                    + a5 * f64::from(p5.red);
                g[i] = a0 * f64::from(p0.green)
                    + a1 * f64::from(p1.green)
                    + a2 * f64::from(p2.green)
                    + a3 * f64::from(p3.green)
                    + a4 * f64::from(p4.green)
                    + a5 * f64::from(p5.green);
                b[i] = a0 * f64::from(p0.blue)
                    + a1 * f64::from(p1.blue)
                    + a2 * f64::from(p2.blue)
                    + a3 * f64::from(p3.blue)
                    + a4 * f64::from(p4.blue)
                    + a5 * f64::from(p5.blue);
                a[i] = a0 * f64::from(p0.alpha)
                    + a1 * f64::from(p1.alpha)
                    + a2 * f64::from(p2.alpha)
                    + a3 * f64::from(p3.alpha)
                    + a4 * f64::from(p4.alpha)
                    + a5 * f64::from(p5.alpha);
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
