use rand::{seq::SliceRandom, thread_rng};

use crate::{Color, DitherMode, I2PState, Sprite};

use super::{
    dither_none_apply, dither_threshold_apply, DITHER_THRESHOLD_BAYER2X2,
    DITHER_THRESHOLD_BAYER4X4, DITHER_THRESHOLD_BAYER8X8, DITHER_THRESHOLD_CLUSTER4X4,
    DITHER_THRESHOLD_CLUSTER8X8,
};

pub(super) fn dither_kmeans(
    state: &mut I2PState,
    input: &[Color],
    output: &mut Sprite,
    width: usize,
    height: usize,
) {
    match state.pixel_dither_mode {
        DitherMode::Bayer8x8 => dither_threshold_apply(
            state,
            input,
            &mut output.data,
            width,
            height,
            &DITHER_THRESHOLD_BAYER8X8,
            3,
        ),
        DitherMode::Bayer4x4 => dither_threshold_apply(
            state,
            input,
            &mut output.data,
            width,
            height,
            &DITHER_THRESHOLD_BAYER4X4,
            2,
        ),
        DitherMode::Bayer2x2 => dither_threshold_apply(
            state,
            input,
            &mut output.data,
            width,
            height,
            &DITHER_THRESHOLD_BAYER2X2,
            1,
        ),
        DitherMode::Cluster8x8 => dither_threshold_apply(
            state,
            input,
            &mut output.data,
            width,
            height,
            &DITHER_THRESHOLD_CLUSTER8X8,
            3,
        ),
        DitherMode::Cluster4x4 => dither_threshold_apply(
            state,
            input,
            &mut output.data,
            width,
            height,
            &DITHER_THRESHOLD_CLUSTER4X4,
            2,
        ),
        DitherMode::None | DitherMode::FloydDistributed | DitherMode::FloydComponent => {
            dither_none_apply(state, input, &mut output.data);
        }
    }

    let temp = output.clone();
    let assignments = quant_compute_kmeans(state, &temp, 1);

    for (col, assignment) in output.data.iter_mut().zip(assignments) {
        if col.alpha == 0 {
            continue;
        }

        *col = state.palette[assignment];
    }
}

fn quant_compute_kmeans(state: &mut I2PState, data: &Sprite, pal_in: i32) -> Vec<usize> {
    let mut quant_cluster_list = vec![Vec::default(); state.palette.len()];
    let mut quant_centroid_list = vec![Color::default(); state.palette.len()];
    let mut quant_assignment = vec![0; data.width * data.height];
    let mut iter = 0;
    let max_iter = 16;
    let mut previous_variance = vec![1.0; state.palette.len()];
    let mut variance: f64;
    let mut delta: f64;
    let mut delta_max: f64 = 0.0;
    let threshold = 0.00005;

    loop {
        quant_get_cluster_centroid(
            state,
            &quant_cluster_list,
            &mut quant_centroid_list,
            data,
            pal_in,
            1 << state.palette_weight,
        );
        quant_cluster_list.shrink_to(0);
        quant_cluster_list.resize(state.palette.len(), Vec::default());
        for i in 0..data.width * data.height {
            let color = data.data[i];
            quant_assignment[i] = quant_nearest_color_idx(color, &quant_centroid_list);
            quant_cluster_list[quant_assignment[i]].push(color);
        }

        for i in 0..state.palette.len() {
            variance = quant_colors_variance(&quant_cluster_list[i]);
            delta = (previous_variance[i] - variance).abs();
            delta_max = delta_max.max(delta);
            previous_variance[i] = variance;
        }

        iter += 1;
        if delta_max < threshold || iter > max_iter {
            break;
        }
    }

    quant_assignment
}

fn quant_colors_variance(color_list: &[Color]) -> f64 {
    let length = color_list.len();
    let mean = quant_colors_mean(color_list, Color::new(0, 0, 0, 0), 0);
    let dist_sum: f64 = color_list
        .iter()
        .map(|c| {
            let dist = quant_distance(*c, mean);
            dist * dist
        })
        .sum();

    #[allow(clippy::cast_precision_loss)]
    {
        dist_sum / length as f64
    }
}

fn quant_nearest_color_idx(color: Color, color_list: &[Color]) -> usize {
    let mut dist_min = f64::MAX;
    let mut dist: f64;
    let mut idx = 0;

    for (i, list_col) in color_list.iter().enumerate() {
        dist = quant_distance(color, *list_col);
        if dist < dist_min {
            dist_min = dist;
            idx = i;
        }
    }

    idx
}

fn quant_distance(color0: Color, color1: Color) -> f64 {
    let mr = 0.5 * (f64::from(color0.red) + f64::from(color1.red));
    let dr = f64::from(color0.red) - f64::from(color1.red);
    let dg = f64::from(color0.green) - f64::from(color1.green);
    let db = f64::from(color0.blue) - f64::from(color1.blue);
    let distance = (2.0 * dr * dr)
        + (4.0 * dg * dg)
        + (3.0 * db * db)
        + (mr * ((dr * dr) - (db * db)) / 256.0);
    distance.sqrt() / (3.0 * 255.0)
}

fn quant_get_cluster_centroid(
    state: &mut I2PState,
    quant_cluster_list: &[Vec<Color>],
    quant_centroid_list: &mut [Color],
    data: &Sprite,
    pal_in: i32,
    palette_weight: i32,
) {
    for i in 0..state.palette.len() {
        if !quant_cluster_list[i].is_empty() {
            if pal_in != 0 {
                quant_centroid_list[i] =
                    quant_colors_mean(&quant_cluster_list[i], state.palette[i], palette_weight);
            } else {
                quant_centroid_list[i] =
                    quant_colors_mean(&quant_cluster_list[i], Color::new(0, 0, 0, 0), 0);
            }
        } else if pal_in != 0 {
            quant_centroid_list[i] = state.palette[i];
        } else {
            quant_centroid_list[i] = quant_pick_random_color(data);
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

#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
fn quant_colors_mean(color_list: &[Color], color: Color, palette_weight: i32) -> Color {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;

    for color in color_list {
        r += i32::from(color.red);
        g += i32::from(color.green);
        b += i32::from(color.blue);
    }

    let mut weight_color = palette_weight;
    if weight_color != 0 {
        weight_color = color_list.len() as i32 / weight_color;
    }
    let length = color_list.len() as i32 + weight_color;
    r += i32::from(color.red) * weight_color;
    g += i32::from(color.green) * weight_color;
    b += i32::from(color.blue) * weight_color;

    if length != 0 {
        r /= length;
        g /= length;
        b /= length;
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    Color::new(r as u8, g as u8, b as u8, 0)
}
