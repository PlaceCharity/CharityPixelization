use serde::{Deserialize, Serialize};

use crate::{Color, I2PState, Sprite};

use self::kmeans::dither_kmeans;

mod kmeans;

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
pub enum DitherMode {
    #[default]
    None,
    Bayer8x8,
    Bayer4x4,
    Bayer2x2,
    Cluster8x8,
    Cluster4x4,
    FloydComponent,
    FloydDistributed,
}

#[derive(Default, Deserialize, Serialize)]
pub enum ProcessMode {
    #[default]
    KMeans,
    RGB,
    CIE76,
    CIE94,
    CIEDE200,
    XYZ,
    YCC,
    YIQ,
    YUV,
}

pub fn dither_image(
    state: &mut I2PState,
    input: Vec<Color>,
    output: &mut Sprite,
    width: usize,
    height: usize,
) {
    match state.pixel_process_mode {
        ProcessMode::KMeans => {
            dither_kmeans(state, input, output, width, height);
        }
        ProcessMode::RGB => todo!(),
        ProcessMode::CIE76 => todo!(),
        ProcessMode::CIE94 => todo!(),
        ProcessMode::CIEDE200 => todo!(),
        ProcessMode::XYZ => todo!(),
        ProcessMode::YCC => todo!(),
        ProcessMode::YIQ => todo!(),
        ProcessMode::YUV => todo!(),
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
