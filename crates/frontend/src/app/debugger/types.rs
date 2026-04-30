use egui_dock::DockState;
use sturdygb_core::gb::{
    ApuDebugSnapshot, DebugSnapshot, DisassemblyLine, MemoryWriteEvent, PpuDebugSnapshot,
};

#[derive(Clone)]
pub(super) struct DebuggerWindowData {
    pub(super) snapshot: DebugSnapshot,
    pub(super) ppu_snapshot: PpuDebugSnapshot,
    pub(super) apu_snapshot: ApuDebugSnapshot,
    pub(super) memory: Vec<u8>,
    pub(super) disassembly: Vec<DisassemblyLine>,
    pub(super) last_writes: Vec<MemoryWriteEvent>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub(crate) enum DebuggerTab {
    Cpu,
    Ppu,
    Apu,
    Breakpoints,
    Disassembly,
    Memory,
    MemoryMap,
    Writes,
    Vram,
    BgMap,
    Oam,
}

impl DebuggerTab {
    pub(crate) fn all() -> &'static [Self] {
        &[
            Self::Cpu,
            Self::Ppu,
            Self::Apu,
            Self::Breakpoints,
            Self::Disassembly,
            Self::Memory,
            Self::MemoryMap,
            Self::Writes,
            Self::Vram,
            Self::BgMap,
            Self::Oam,
        ]
    }

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Cpu => "CPU",
            Self::Ppu => "PPU",
            Self::Apu => "APU",
            Self::Breakpoints => "Breakpoints",
            Self::Disassembly => "Disassembly",
            Self::Memory => "Memory",
            Self::MemoryMap => "Memory Map",
            Self::Writes => "Writes",
            Self::Vram => "VRAM",
            Self::BgMap => "BG Map",
            Self::Oam => "OAM",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DebuggerStepKind {
    Over,
    Into,
    Out,
}

impl DebuggerStepKind {
    pub(crate) fn all() -> &'static [Self] {
        &[Self::Over, Self::Into, Self::Out]
    }

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Over => "⏩ Step",
            Self::Into => "↪ Step In",
            Self::Out => "↩ Step Out",
        }
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct PersistedDebuggerLayout {
    pub(super) dock_state: DockState<DebuggerTab>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum MemoryRegion {
    Rom0,
    RomX,
    Vram,
    ExtRam,
    Wram0,
    WramX,
    Echo,
    Oam,
    Unusable,
    Io,
    Hram,
    InterruptEnable,
}

impl MemoryRegion {
    pub(super) fn all() -> &'static [(Self, u16, u16)] {
        &[
            (Self::Rom0, 0x0000, 0x3FFF),
            (Self::RomX, 0x4000, 0x7FFF),
            (Self::Vram, 0x8000, 0x9FFF),
            (Self::ExtRam, 0xA000, 0xBFFF),
            (Self::Wram0, 0xC000, 0xCFFF),
            (Self::WramX, 0xD000, 0xDFFF),
            (Self::Echo, 0xE000, 0xFDFF),
            (Self::Oam, 0xFE00, 0xFE9F),
            (Self::Unusable, 0xFEA0, 0xFEFF),
            (Self::Io, 0xFF00, 0xFF7F),
            (Self::Hram, 0xFF80, 0xFFFE),
            (Self::InterruptEnable, 0xFFFF, 0xFFFF),
        ]
    }

    pub(super) fn label(self) -> &'static str {
        match self {
            Self::Rom0 => "ROM0",
            Self::RomX => "ROMX",
            Self::Vram => "VRAM",
            Self::ExtRam => "SRAM",
            Self::Wram0 => "WRAM0",
            Self::WramX => "WRAMX",
            Self::Echo => "ECHO",
            Self::Oam => "OAM",
            Self::Unusable => "UNUSABLE",
            Self::Io => "IO",
            Self::Hram => "HRAM",
            Self::InterruptEnable => "IE",
        }
    }

    pub(super) fn for_address(address: u16) -> Self {
        match address {
            0x0000..=0x3FFF => Self::Rom0,
            0x4000..=0x7FFF => Self::RomX,
            0x8000..=0x9FFF => Self::Vram,
            0xA000..=0xBFFF => Self::ExtRam,
            0xC000..=0xCFFF => Self::Wram0,
            0xD000..=0xDFFF => Self::WramX,
            0xE000..=0xFDFF => Self::Echo,
            0xFE00..=0xFE9F => Self::Oam,
            0xFEA0..=0xFEFF => Self::Unusable,
            0xFF00..=0xFF7F => Self::Io,
            0xFF80..=0xFFFE => Self::Hram,
            _ => Self::InterruptEnable,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PendingStep {
    Into,
    Over { target_pc: u16, initial_sp: u16 },
    Out { target_pc: u16, target_sp: u16 },
}

impl PendingStep {
    pub(super) fn label(self) -> &'static str {
        match self {
            Self::Into => "Step In",
            Self::Over { .. } => "Step",
            Self::Out { .. } => "Step Out",
        }
    }
}
