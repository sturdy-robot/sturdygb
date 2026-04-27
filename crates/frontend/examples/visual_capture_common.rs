// SPDX-FileCopyrightText: 2026 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use std::path::Path;

use image::{Rgb, RgbImage};
use sturdygb_core::test_roms::{CapturedScreen, GB_SCREEN_HEIGHT, GB_SCREEN_WIDTH};

#[derive(Clone, Copy, Debug, Default, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DmgPalette {
    #[default]
    Greyscale,
    ClassicGreen,
    Pocket,
}

impl DmgPalette {
    fn colors(self) -> [(u8, u8, u8); 4] {
        match self {
            Self::Greyscale => [(255, 255, 255), (192, 192, 192), (96, 96, 96), (0, 0, 0)],
            Self::ClassicGreen => [(224, 248, 208), (136, 192, 112), (52, 104, 86), (8, 24, 32)],
            Self::Pocket => [(232, 232, 232), (160, 160, 160), (88, 88, 88), (16, 16, 16)],
        }
    }
}

pub fn sanitize_file_stem(name: &str) -> String {
    let mut stem = String::with_capacity(name.len());
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            stem.push(ch.to_ascii_lowercase());
        } else if !stem.ends_with('-') {
            stem.push('-');
        }
    }
    stem.trim_matches('-').to_string()
}

pub fn save_captured_screen_png(
    screen: &CapturedScreen,
    palette: DmgPalette,
    output_path: &Path,
) -> Result<(), String> {
    let mut image = RgbImage::new(GB_SCREEN_WIDTH as u32, GB_SCREEN_HEIGHT as u32);

    match screen {
        CapturedScreen::Dmg(pixels) => {
            let palette_colors = palette.colors();
            for (index, shade) in pixels.iter().enumerate() {
                let x = (index % GB_SCREEN_WIDTH) as u32;
                let y = (index / GB_SCREEN_WIDTH) as u32;
                let (red, green, blue) = palette_colors[*shade as usize];
                image.put_pixel(x, y, Rgb([red, green, blue]));
            }
        }
        CapturedScreen::Cgb(pixels) => {
            for (index, pixel) in pixels.iter().enumerate() {
                let x = (index % GB_SCREEN_WIDTH) as u32;
                let y = (index / GB_SCREEN_WIDTH) as u32;
                image.put_pixel(x, y, Rgb(*pixel));
            }
        }
    }

    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
    }

    image
        .save(output_path)
        .map_err(|err| format!("failed to save {}: {err}", output_path.display()))
}