use rand::{seq::SliceRandom, thread_rng};

use crate::{Color, DitherMode, I2PState, Sprite};

use super::{
    dither_none_apply, dither_threshold_apply, DITHER_THRESHOLD_BAYER2X2,
    DITHER_THRESHOLD_BAYER4X4, DITHER_THRESHOLD_BAYER8X8, DITHER_THRESHOLD_CLUSTER4X4,
    DITHER_THRESHOLD_CLUSTER8X8,
};

pub(super) fn dither_kmeans(
    state: &mut I2PState,
    input: Vec<Color>,
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
            dither_none_apply(state, input, &mut output.data, width, height)
        }
    }

    state.quant_k = state.palette.len() as i32;
    let temp = output.clone();
    quant_compute_kmeans(state, &temp, 1);

    for i in 0..width * height {
        if output.data[i].alpha == 0 {
            output.data[i] = Default::default();
        } else {
            output.data[i] = state.palette[state.quant_assignment[i]];
        }
    }
}

fn quant_compute_kmeans(state: &mut I2PState, data: &Sprite, pal_in: i32) {
    state.quant_cluster_list.shrink_to(0);
    state
        .quant_cluster_list
        .resize(state.quant_k as usize, Default::default());
    state.quant_centroid_list.shrink_to(0);
    state
        .quant_centroid_list
        .resize(state.quant_k as usize, Default::default());
    state.quant_assignment.shrink_to(0);
    state.quant_assignment.resize(data.width * data.height, 0);
    let mut iter = 0;
    let max_iter = 16;
    let mut previous_variance = vec![1.0; state.quant_k as usize];
    let mut variance: f64;
    let mut delta: f64;
    let mut delta_max: f64 = 0.0;
    let threshold = 0.00005;

    loop {
        quant_get_cluster_centroid(state, data, pal_in, 1 << state.palette_weight);
        state.quant_cluster_list.shrink_to(0);
        state
            .quant_cluster_list
            .resize(state.quant_k as usize, Default::default());
        for i in 0..data.width * data.height {
            let color = data.data[i];
            state.quant_assignment[i] = quant_nearest_color_idx(color, &state.quant_centroid_list);
            state.quant_cluster_list[state.quant_assignment[i]].push(color);
        }

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
    let mean = quant_colors_mean(color_list, Color::new(0, 0, 0, 0), 0);
    let dist_sum: f64 = color_list
        .iter()
        .map(|c| {
            let dist = quant_distance(*c, mean);
            dist * dist
        })
        .sum();

    dist_sum / length as f64
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
                state.quant_centroid_list[i as usize] = quant_colors_mean(
                    &state.quant_cluster_list[i as usize],
                    Color::new(0, 0, 0, 0),
                    0,
                );
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
        b += color.blue as i32;
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

    Color::new(r as u8, g as u8, b as u8, 0)
}
