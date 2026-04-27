// SPDX-FileCopyrightText: 2026 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

use crate::gb::{Gb, ModelSelection, ScreenData};
use crate::prelude::GbInstance;

pub const GB_SCREEN_WIDTH: usize = 160;
pub const GB_SCREEN_HEIGHT: usize = 144;
pub const DEFAULT_VISUAL_STEP_LIMIT: usize = 3_000_000;
const MOONEYE_PASS_REGISTERS: [u8; 6] = [3, 5, 8, 13, 21, 34];
const MOONEYE_FAIL_REGISTERS: [u8; 6] = [0x42; 6];

fn mooneye_register_signature(gb: &Gb) -> [u8; 6] {
    [
        gb.cpu.b(),
        gb.cpu.c(),
        gb.cpu.d(),
        gb.cpu.e(),
        gb.cpu.h(),
        gb.cpu.l(),
    ]
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum VisualStopCondition {
    Frames {
        frames: usize,
    },
    OpcodeBreakpoint {
        opcode: u8,
        #[serde(default)]
        min_frames: usize,
        #[serde(default)]
        register_b: Option<u8>,
    },
    SerialContains {
        needle: String,
    },
    MooneyeResult {
        #[serde(default)]
        min_steps: usize,
    },
    SerialQuietSteps {
        quiet_steps: usize,
        #[serde(default)]
        min_steps: usize,
        #[serde(default = "default_serial_min_bytes")]
        min_serial_bytes: usize,
    },
    SerialQuietFrames {
        quiet_frames: usize,
        #[serde(default)]
        min_frames: usize,
        #[serde(default = "default_serial_min_bytes")]
        min_serial_bytes: usize,
    },
    ScreenQuietFrames {
        quiet_frames: usize,
        #[serde(default)]
        min_frames: usize,
    },
    Any {
        conditions: Vec<VisualStopCondition>,
    },
}

impl VisualStopCondition {
    fn evaluate(
        &self,
        gb: &Gb,
        opcode: u8,
        completed_frames: usize,
        executed_steps: usize,
        serial_output: &str,
        steps_since_serial_change: Option<usize>,
        frames_since_serial_change: Option<usize>,
        frames_since_screen_change: Option<usize>,
    ) -> Option<Option<String>> {
        match self {
            Self::Frames { frames } => (completed_frames >= *frames).then_some(None),
            Self::OpcodeBreakpoint {
                opcode: expected_opcode,
                min_frames,
                register_b,
            } => (opcode == *expected_opcode
                && completed_frames >= *min_frames
                && register_b.is_none_or(|expected_b| gb.cpu.b() == expected_b))
            .then_some(None),
            Self::SerialContains { needle } => serial_output.contains(needle).then_some(None),
            Self::MooneyeResult { min_steps } => {
                if executed_steps < *min_steps || opcode != 0x40 {
                    return None;
                }

                let signature = mooneye_register_signature(gb);
                if signature == MOONEYE_PASS_REGISTERS {
                    Some(Some("mooneye: pass".to_string()))
                } else if signature == MOONEYE_FAIL_REGISTERS {
                    Some(Some("mooneye: fail".to_string()))
                } else {
                    None
                }
            }
            Self::SerialQuietSteps {
                quiet_steps,
                min_steps,
                min_serial_bytes,
            } => (serial_output.len() >= *min_serial_bytes
                && executed_steps >= *min_steps
                && steps_since_serial_change.is_some_and(|steps| steps >= *quiet_steps))
            .then_some(None),
            Self::SerialQuietFrames {
                quiet_frames,
                min_frames,
                min_serial_bytes,
            } => (serial_output.len() >= *min_serial_bytes
                && completed_frames >= *min_frames
                && frames_since_serial_change.is_some_and(|frames| frames >= *quiet_frames))
            .then_some(None),
            Self::ScreenQuietFrames {
                quiet_frames,
                min_frames,
            } => (completed_frames >= *min_frames
                && frames_since_screen_change.is_some_and(|frames| frames >= *quiet_frames))
            .then_some(None),
            Self::Any { conditions } => conditions.iter().find_map(|condition| {
                condition.evaluate(
                    gb,
                    opcode,
                    completed_frames,
                    executed_steps,
                    serial_output,
                    steps_since_serial_change,
                    frames_since_serial_change,
                    frames_since_screen_change,
                )
            }),
        }
    }
}

fn default_visual_step_limit() -> usize {
    DEFAULT_VISUAL_STEP_LIMIT
}

fn default_serial_min_bytes() -> usize {
    1
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct VisualCaptureConfig {
    #[serde(alias = "rom")]
    pub rom_relative_path: String,
    #[serde(default)]
    pub model_selection: ModelSelection,
    #[serde(default = "default_visual_step_limit")]
    pub step_limit: usize,
    pub stop_condition: VisualStopCondition,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CapturedScreen {
    Dmg(Vec<u8>),
    Cgb(Vec<[u8; 3]>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VisualCapture {
    pub rom_relative_path: String,
    pub model_selection: ModelSelection,
    pub outcome: VisualCaptureOutcome,
    pub result_summary: Option<String>,
    pub completed_frames: usize,
    pub executed_steps: usize,
    pub final_pc: u16,
    pub serial_output: String,
    pub screen: CapturedScreen,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VisualCaptureOutcome {
    StopConditionMet,
    StepLimitReached,
}

impl VisualCaptureOutcome {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::StopConditionMet => "stop-condition-met",
            Self::StepLimitReached => "step-limit-reached",
        }
    }
}

pub fn rom_path(relative_path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("roms")
        .join(relative_path)
}

pub fn load_test_gb(relative_path: &str, model_selection: ModelSelection) -> Gb {
    let path = rom_path(relative_path);
    GbInstance::build_with_model(path.to_string_lossy().as_ref(), model_selection)
        .unwrap_or_else(|err| panic!("failed to load {}: {err}", path.display()))
}

fn drain_serial_output(gb: &mut Gb) -> String {
    let mut serial = String::new();
    while let Some(chunk) = gb.take_serial_output() {
        serial.push_str(&chunk);
    }
    serial
}

fn capture_screen(gb: &mut Gb) -> CapturedScreen {
    match gb.get_screen_data() {
        ScreenData::Dmg(frame) => {
            let mut pixels = Vec::with_capacity(GB_SCREEN_WIDTH * GB_SCREEN_HEIGHT);
            for row in frame {
                pixels.extend(row.iter().copied());
            }
            CapturedScreen::Dmg(pixels)
        }
        ScreenData::Cgb(frame) => {
            let mut pixels = Vec::with_capacity(GB_SCREEN_WIDTH * GB_SCREEN_HEIGHT);
            for row in frame {
                pixels.extend(row.iter().copied());
            }
            CapturedScreen::Cgb(pixels)
        }
    }
}

fn build_visual_capture(
    config: &VisualCaptureConfig,
    gb: &mut Gb,
    completed_frames: usize,
    executed_steps: usize,
    serial_output: String,
    result_summary: Option<String>,
    outcome: VisualCaptureOutcome,
) -> VisualCapture {
    VisualCapture {
        rom_relative_path: config.rom_relative_path.clone(),
        model_selection: config.model_selection,
        outcome,
        result_summary,
        completed_frames,
        executed_steps,
        final_pc: gb.current_pc(),
        serial_output,
        screen: capture_screen(gb),
    }
}

pub fn capture_visual_test(config: &VisualCaptureConfig) -> VisualCapture {
    let mut gb = load_test_gb(&config.rom_relative_path, config.model_selection);
    gb.set_serial_stdout_enabled(false);
    let mut serial_output = String::new();
    let mut completed_frames: usize = 0;
    let mut last_serial_change_step: Option<usize> = None;
    let mut last_serial_change_frame: Option<usize> = None;
    let mut previous_screen: Option<CapturedScreen> = None;
    let mut last_screen_change_frame: Option<usize> = None;

    for step in 0..config.step_limit {
        gb.step_instruction();
        let new_serial_output = drain_serial_output(&mut gb);
        if !new_serial_output.is_empty() {
            serial_output.push_str(&new_serial_output);
            last_serial_change_step = Some(step + 1);
            last_serial_change_frame = Some(completed_frames);
        }

        if gb.frame_ready() {
            completed_frames += 1;
            let current_screen = capture_screen(&mut gb);
            if previous_screen.as_ref() != Some(&current_screen) {
                last_screen_change_frame = Some(completed_frames);
                previous_screen = Some(current_screen);
            }
        }

        let snapshot = gb.debug_snapshot();
        let executed_steps = step + 1;
        let steps_since_serial_change =
            last_serial_change_step.map(|serial_step| executed_steps.saturating_sub(serial_step));
        let frames_since_serial_change =
            last_serial_change_frame.map(|frame| completed_frames.saturating_sub(frame));
        let frames_since_screen_change =
            last_screen_change_frame.map(|frame| completed_frames.saturating_sub(frame));
        if let Some(result_summary) = config.stop_condition.evaluate(
            &gb,
            snapshot.opcode,
            completed_frames,
            executed_steps,
            &serial_output,
            steps_since_serial_change,
            frames_since_serial_change,
            frames_since_screen_change,
        ) {
            return build_visual_capture(
                config,
                &mut gb,
                completed_frames,
                executed_steps,
                serial_output,
                result_summary,
                VisualCaptureOutcome::StopConditionMet,
            );
        }
    }

    build_visual_capture(
        config,
        &mut gb,
        completed_frames,
        config.step_limit,
        serial_output,
        None,
        VisualCaptureOutcome::StepLimitReached,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    const MOONEYE_STEP_LIMIT: usize = 2_000_000;

    fn run_mooneye_rom(relative_path: &str, model_selection: ModelSelection) -> Result<(), String> {
        let mut gb = load_test_gb(relative_path, model_selection);
        let mut serial = String::new();

        for step in 0..MOONEYE_STEP_LIMIT {
            let snapshot = gb.debug_snapshot();
            if snapshot.opcode == 0x40 {
                let registers = mooneye_register_signature(&gb);
                if registers == MOONEYE_PASS_REGISTERS {
                    return Ok(());
                }
                if registers == MOONEYE_FAIL_REGISTERS {
                    return Err(format!(
                        "{relative_path} reported failure after {step} steps; serial={serial:?}"
                    ));
                }
            }

            gb.step_instruction();
            serial.push_str(&drain_serial_output(&mut gb));
        }

        Err(format!(
            "{relative_path} did not reach a mooneye result within {MOONEYE_STEP_LIMIT} steps; pc={:#06X} serial={serial:?}",
            gb.current_pc()
        ))
    }

    #[test]
    fn reference_roms_build_in_expected_modes() {
        let dmg = load_test_gb("dmg-acid2.gb", ModelSelection::Auto);
        assert_eq!(dmg.gb_type, crate::gb::GbTypes::Dmg);

        let cgb = load_test_gb("cgb-acid2.gbc", ModelSelection::Auto);
        assert_eq!(cgb.gb_type, crate::gb::GbTypes::Cgb);
    }

    #[test]
    #[ignore = "diagnostic: exposes remaining CGB boot-state mismatches"]
    fn mooneye_boot_regs_cgb_passes() {
        run_mooneye_rom(
            "mooneye-test-suite/misc/boot_regs-cgb.gb",
            ModelSelection::Cgb,
        )
        .unwrap();
    }

    #[test]
    #[ignore = "diagnostic: inspect boot_regs-cgb stall state"]
    fn diagnose_mooneye_boot_regs_cgb_stall() {
        let mut gb = load_test_gb(
            "mooneye-test-suite/misc/boot_regs-cgb.gb",
            ModelSelection::Cgb,
        );

        for _ in 0..MOONEYE_STEP_LIMIT {
            let snapshot = gb.debug_snapshot();
            if snapshot.opcode == 0x40 {
                println!(
                    "reached_magic_breakpoint registers={:02X?}",
                    mooneye_register_signature(&gb)
                );
                break;
            }

            gb.step_instruction();
        }

        let snapshot = gb.debug_snapshot();
        println!(
            "pc={:#06X} opcode={:#04X} af={:#06X} bc={:#06X} de={:#06X} hl={:#06X} sp={:#06X} ly={:#04X} stat={:#04X}",
            snapshot.pc,
            snapshot.opcode,
            snapshot.af,
            snapshot.bc,
            snapshot.de,
            snapshot.hl,
            snapshot.sp,
            snapshot.ly,
            snapshot.stat,
        );
        println!("ff00={:#04X}", gb.read_byte(0xFF00));
        println!("ff04={:#04X}", gb.read_byte(0xFF04));
        println!("ff05={:#04X}", gb.read_byte(0xFF05));
        println!("ff06={:#04X}", gb.read_byte(0xFF06));
        println!("ff07={:#04X}", gb.read_byte(0xFF07));
        println!("ff0f={:#04X}", gb.read_byte(0xFF0F));
        println!("ff26={:#04X}", gb.read_byte(0xFF26));
        println!("ff40={:#04X}", gb.read_byte(0xFF40));
        println!("ff41={:#04X}", gb.read_byte(0xFF41));
        println!("ff42={:#04X}", gb.read_byte(0xFF42));
        println!("ff43={:#04X}", gb.read_byte(0xFF43));
        println!("ff44={:#04X}", gb.read_byte(0xFF44));
        println!("ff45={:#04X}", gb.read_byte(0xFF45));
        println!("ff46={:#04X}", gb.read_byte(0xFF46));
        println!("ff47={:#04X}", gb.read_byte(0xFF47));
        println!("ff48={:#04X}", gb.read_byte(0xFF48));
        println!("ff49={:#04X}", gb.read_byte(0xFF49));
        println!("ff4a={:#04X}", gb.read_byte(0xFF4A));
        println!("ff4b={:#04X}", gb.read_byte(0xFF4B));
        println!("ff4d={:#04X}", gb.read_byte(0xFF4D));
        println!("ff4f={:#04X}", gb.read_byte(0xFF4F));
        println!("ff56={:#04X}", gb.read_byte(0xFF56));
        println!("ff68={:#04X}", gb.read_byte(0xFF68));
        println!("ff69={:#04X}", gb.read_byte(0xFF69));
        println!("ff6a={:#04X}", gb.read_byte(0xFF6A));
        println!("ff6b={:#04X}", gb.read_byte(0xFF6B));
        println!("ff70={:#04X}", gb.read_byte(0xFF70));
        println!("ffff={:#04X}", gb.read_byte(0xFFFF));

        let mut min_ly = u8::MAX;
        let mut max_ly = 0;
        let mut saw_ly_143 = false;
        let mut saw_ly_144 = false;
        let mut left_wait_loop = false;

        for _ in 0..100_000 {
            let ly = gb.read_byte(0xFF44);
            min_ly = min_ly.min(ly);
            max_ly = max_ly.max(ly);
            saw_ly_143 |= ly == 143;
            saw_ly_144 |= ly == 144;

            let pc = gb.current_pc();
            if !(0x4ACA..=0x4AD4).contains(&pc) {
                left_wait_loop = true;
                break;
            }

            gb.step_instruction();
        }

        println!(
            "wait_loop min_ly={:#04X} max_ly={:#04X} saw_ly_143={} saw_ly_144={} left_wait_loop={}",
            min_ly, max_ly, saw_ly_143, saw_ly_144, left_wait_loop
        );

        let disasm_start = snapshot.pc.saturating_sub(6);
        for line in gb.disassemble_range(disasm_start, 8) {
            println!("{:#06X}: {:02X?} {}", line.address, line.bytes, line.text);
        }
    }

    #[test]
    fn capture_visual_test_returns_state_when_stop_condition_is_met() {
        let capture = capture_visual_test(&VisualCaptureConfig {
            rom_relative_path: "cgb-acid2.gbc".to_string(),
            model_selection: ModelSelection::Cgb,
            step_limit: 1,
            stop_condition: VisualStopCondition::Frames { frames: 0 },
        });
        assert_eq!(capture.outcome, VisualCaptureOutcome::StopConditionMet);
        assert_eq!(capture.result_summary, None);
        assert_eq!(capture.executed_steps, 1);
        assert!(matches!(capture.screen, CapturedScreen::Cgb(_)));
    }

    #[test]
    fn capture_visual_test_preserves_state_on_step_limit() {
        let capture = capture_visual_test(&VisualCaptureConfig {
            rom_relative_path: "cgb-acid2.gbc".to_string(),
            model_selection: ModelSelection::Cgb,
            step_limit: 1,
            stop_condition: VisualStopCondition::OpcodeBreakpoint {
                opcode: 0x40,
                min_frames: 9,
                register_b: Some(0),
            },
        });

        assert_eq!(capture.outcome, VisualCaptureOutcome::StepLimitReached);
        assert_eq!(capture.result_summary, None);
        assert_eq!(capture.executed_steps, 1);
        assert!(matches!(capture.screen, CapturedScreen::Cgb(_)));
    }

    #[test]
    fn blargg_instr_timing_produces_visible_screen_output() {
        let capture = capture_visual_test(&VisualCaptureConfig {
            rom_relative_path: "gb-test-roms/instr_timing/instr_timing.gb".to_string(),
            model_selection: ModelSelection::Auto,
            step_limit: 12_000_000,
            stop_condition: VisualStopCondition::SerialContains {
                needle: "Passed".to_string(),
            },
        });

        let non_zero_pixels = match &capture.screen {
            CapturedScreen::Dmg(pixels) => pixels.iter().filter(|&&pixel| pixel != 0).count(),
            CapturedScreen::Cgb(pixels) => pixels
                .iter()
                .filter(|&&pixel| pixel != [0, 0, 0])
                .count(),
        };

        assert_eq!(capture.outcome, VisualCaptureOutcome::StopConditionMet);
        assert!(capture.completed_frames >= 10);
        assert!(non_zero_pixels > 0);
    }

    #[test]
    fn any_stop_condition_allows_mooneye_capture_to_finish_on_quiet_serial() {
        let capture = capture_visual_test(&VisualCaptureConfig {
            rom_relative_path: "mooneye-test-suite/acceptance/ei_sequence.gb".to_string(),
            model_selection: ModelSelection::Auto,
            step_limit: 4_000_000,
            stop_condition: VisualStopCondition::Any {
                conditions: vec![
                    VisualStopCondition::MooneyeResult { min_steps: 0 },
                    VisualStopCondition::SerialQuietSteps {
                        quiet_steps: 200_000,
                        min_steps: 100_000,
                        min_serial_bytes: 1,
                    },
                ],
            },
        });

        assert_eq!(capture.outcome, VisualCaptureOutcome::StopConditionMet);
        assert!(capture.executed_steps < 4_000_000);
        assert_eq!(capture.serial_output.trim(), "BBBBBB");
    }
}
