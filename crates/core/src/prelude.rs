// SPDX-FileCopyrightText: 2026 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT
use super::cartridge::{load_cartridge, GbMode};
use super::gb::{Gb, GbTypes, ModelSelection};

pub struct GbInstance;

fn resolve_model_selection(
    cartridge_mode: GbMode,
    model_selection: ModelSelection,
) -> Result<(GbMode, GbTypes), String> {
    match model_selection {
        ModelSelection::Auto => {
            let gb_type = if cartridge_mode == GbMode::CgbMode {
                GbTypes::Cgb
            } else {
                GbTypes::Dmg
            };
            Ok((cartridge_mode, gb_type))
        }
        ModelSelection::Dmg => match cartridge_mode {
            GbMode::CgbMode => Err("This ROM requires a Game Boy Color model".to_string()),
            _ => Ok((GbMode::DmgMode, GbTypes::Dmg)),
        },
        ModelSelection::Cgb => {
            let gb_mode = match cartridge_mode {
                GbMode::DmgMode => GbMode::NonCgbMode,
                _ => GbMode::CgbMode,
            };
            Ok((gb_mode, GbTypes::Cgb))
        }
    }
}

impl GbInstance {
    pub fn build(filename: &str) -> Result<Gb, String> {
        Self::build_with_model(filename, ModelSelection::Auto)
    }

    pub fn build_with_model(filename: &str, model_selection: ModelSelection) -> Result<Gb, String> {
        match load_cartridge(filename) {
            Ok((mbc, gb_mode)) => {
                let (gb_mode, gb_type) = resolve_model_selection(gb_mode, model_selection)?;
                Ok(Gb::new(mbc, gb_mode, gb_type))
            }
            Err(e) => Err(format!("Error loading ROM: {e}")),
        }
    }

    pub fn build_from_bytes(
        rom_data: Vec<u8>,
        save_path: Option<std::path::PathBuf>,
    ) -> Result<Gb, String> {
        Self::build_from_bytes_with_model(rom_data, save_path, ModelSelection::Auto)
    }

    pub fn build_from_bytes_with_model(
        rom_data: Vec<u8>,
        save_path: Option<std::path::PathBuf>,
        model_selection: ModelSelection,
    ) -> Result<Gb, String> {
        use crate::cartridge::load_cartridge_from_bytes;
        match load_cartridge_from_bytes(rom_data, save_path) {
            Ok((mbc, gb_mode)) => {
                let (gb_mode, gb_type) = resolve_model_selection(gb_mode, model_selection)?;
                Ok(Gb::new(mbc, gb_mode, gb_type))
            }
            Err(e) => Err(format!("Error parsing ROM bytes: {e}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_rom(cgb_flag: u8) -> Vec<u8> {
        let mut rom = vec![0u8; 0x8000];
        rom[0x0143] = cgb_flag;
        rom[0x0147] = 0x00;
        rom[0x0148] = 0x00;
        rom[0x0149] = 0x00;

        let mut checksum = 0u8;
        for byte in &rom[0x0134..=0x014C] {
            checksum = checksum.wrapping_sub(*byte).wrapping_sub(1);
        }
        rom[0x014D] = checksum;
        rom
    }

    #[test]
    fn auto_selection_preserves_default_dmg_behavior() {
        let gb = GbInstance::build_from_bytes_with_model(
            make_test_rom(0x00),
            None,
            ModelSelection::Auto,
        )
        .expect("DMG ROM should build");

        assert_eq!(gb.gb_type, GbTypes::Dmg);
        assert_eq!(gb.gb_mode, GbMode::DmgMode);
    }

    #[test]
    fn auto_selection_preserves_default_cgb_behavior() {
        let gb = GbInstance::build_from_bytes_with_model(
            make_test_rom(0xC0),
            None,
            ModelSelection::Auto,
        )
        .expect("CGB ROM should build");

        assert_eq!(gb.gb_type, GbTypes::Cgb);
        assert_eq!(gb.gb_mode, GbMode::CgbMode);
    }

    #[test]
    fn forced_cgb_runs_dmg_rom_in_compatibility_mode() {
        let gb =
            GbInstance::build_from_bytes_with_model(make_test_rom(0x00), None, ModelSelection::Cgb)
                .expect("DMG ROM should run on CGB hardware");

        assert_eq!(gb.gb_type, GbTypes::Cgb);
        assert_eq!(gb.gb_mode, GbMode::NonCgbMode);
        assert_eq!(gb.wram.len(), 0x8000);
        assert_eq!(gb.vram_bank_count(), 2);
    }

    #[test]
    fn forced_cgb_enables_cgb_features_for_enhanced_roms() {
        let gb =
            GbInstance::build_from_bytes_with_model(make_test_rom(0x80), None, ModelSelection::Cgb)
                .expect("Enhanced ROM should run in CGB mode when requested");

        assert_eq!(gb.gb_type, GbTypes::Cgb);
        assert_eq!(gb.gb_mode, GbMode::CgbMode);
    }

    #[test]
    fn forced_dmg_rejects_cgb_only_roms() {
        let err = match GbInstance::build_from_bytes_with_model(
            make_test_rom(0xC0),
            None,
            ModelSelection::Dmg,
        ) {
            Ok(_) => panic!("CGB-only ROM should not run on DMG hardware"),
            Err(err) => err,
        };

        assert!(err.contains("Game Boy Color"));
    }

    #[test]
    fn cgb_hardware_exposes_palette_and_wram_banking_registers() {
        let mut gb =
            GbInstance::build_from_bytes_with_model(make_test_rom(0x00), None, ModelSelection::Cgb)
                .expect("DMG ROM should run on CGB hardware");

        gb.write_byte(0xFF68, 0x80 | 0x02);
        gb.write_byte(0xFF69, 0xAB);
        gb.write_byte(0xFF68, 0x02);
        assert_eq!(gb.read_byte(0xFF69), 0xAB);

        gb.write_byte(0xFF70, 0x01);
        gb.write_byte(0xD000, 0x11);
        gb.write_byte(0xFF70, 0x02);
        gb.write_byte(0xD000, 0x22);
        assert_eq!(gb.read_byte(0xD000), 0x22);
        gb.write_byte(0xFF70, 0x01);
        assert_eq!(gb.read_byte(0xD000), 0x11);
    }

    #[test]
    fn dmg_hardware_blocks_cgb_only_registers() {
        let mut gb =
            GbInstance::build_from_bytes_with_model(make_test_rom(0x00), None, ModelSelection::Dmg)
                .expect("DMG ROM should build");

        gb.write_byte(0xFF68, 0x80 | 0x02);
        gb.write_byte(0xFF69, 0xAB);
        gb.write_byte(0xFF4F, 0x01);
        gb.write_byte(0xFF70, 0x02);

        assert_eq!(gb.read_byte(0xFF68), 0xFF);
        assert_eq!(gb.read_byte(0xFF69), 0xFF);
        assert_eq!(gb.read_byte(0xFF4F), 0xFF);
        assert_eq!(gb.read_byte(0xFF70), 0xFF);
    }
}
