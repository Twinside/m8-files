use crate::reader::*;
use crate::remapper::{EqMapping, InstrumentMapping, TableMapping};
use crate::version::*;
use crate::writer::Writer;
use crate::CommandPack;
use array_concat::*;

#[derive(Copy, Clone)]
pub struct FxCommands {
    pub commands: &'static [&'static str],
}

impl FxCommands {
    pub fn find_indices(&self, to_find: &[&str]) -> Vec<u8> {
        let mut out = vec![];

        for (i, cmd) in self.commands.iter().enumerate() {
            if to_find.contains(cmd) {
                out.push(i as u8)
            }
        }

        out
    }

    pub fn try_render(self, cmd: u8) -> Option<&'static str> {
        let cmd = cmd as usize;

        if cmd < self.commands.len() {
            Some(self.commands[cmd])
        } else {
            None
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct FX {
    pub command: u8,
    pub value: u8,
}

impl FX {
    pub fn map_instr(
        self,
        instrument_mapping: &InstrumentMapping,
        table_mapping: &TableMapping,
        eq_mapping: &EqMapping,
    ) -> Self {
        let uval = self.value as usize;

        if instrument_mapping
            .instrument_tracking_commands
            .contains(&self.command)
            && uval < instrument_mapping.mapping.len()
        {
            Self {
                command: self.command,
                value: instrument_mapping.mapping[uval],
            }
        } else if table_mapping
            .table_tracking_commands
            .contains(&self.command)
            && uval < table_mapping.mapping.len()
        {
            Self {
                command: self.command,
                value: table_mapping.mapping[uval],
            }
        } else if eq_mapping.eq_tracking_commands.contains(&self.command)
            && uval < eq_mapping.mapping.len()
        {
            Self {
                command: self.command,
                value: eq_mapping.mapping[uval],
            }
        } else {
            self
        }
    }
}

impl Default for FX {
    fn default() -> Self {
        Self {
            command: 0xFF,
            value: 0,
        }
    }
}

//////////////////////////////////////////
// MARK: V2 commands
//////////////////////////////////////////

#[rustfmt::skip] // Keep constants with important order vertical for maintenance
const SEQ_COMMAND_V2 : [&'static str; 23] = [
    "ARP",
    "CHA",
    "DEL",
    "GRV",
    "HOP",
    "KIL",
    "RAN",
    "RET",
    "REP",
    "NTH",
    "PSL",
    "PSN",
    "PVB",
    "PVX",
    "SCA",
    "SCG",
    "SED",
    "SNG",
    "TBL",
    "THO",
    "TIC",
    "TPO",
    "TSP",
];

#[rustfmt::skip] // Keep constants with important order vertical for maintenance
const FX_MIXER_COMMAND_V2 : [&'static str; 36] = [
    "VMV",
    "XCM",
    "XCF",
    "XCW",
    "XCR",
    "XDT",
    "XDF",
    "XDW",
    "XDR",
    "XRS",
    "XRD",
    "XRM",
    "XRF",
    "XRW",
    "XRZ",
    "VCH",
    "VCD",
    "VRE",
    "VT1",
    "VT2",
    "VT3",
    "VT4",
    "VT5",
    "VT6",
    "VT7",
    "VT8",
    "DJF",
    "IVO",
    "ICH",
    "IDE",
    "IRE",
    "IV2",
    "IC2",
    "ID2",
    "IR2",
    "USB",
];

const COMMANDS_V2: [&'static str; concat_arrays_size!(SEQ_COMMAND_V2, FX_MIXER_COMMAND_V2)] =
    concat_arrays!(SEQ_COMMAND_V2, FX_MIXER_COMMAND_V2);

//////////////////////////////////////////
// MARK: V3 commands
//////////////////////////////////////////

#[rustfmt::skip] // Keep constants with important order vertical for maintenance
const SEQ_COMMAND_V3 : [&'static str; 27] = [
    "ARP",
    "CHA",
    "DEL",
    "GRV",
    "HOP",
    "KIL",
    "RND",
    "RNL",
    "RET",
    "REP",
    "RMX",
    "NTH",
    "PSL",
    "PBN",
    "PVB",
    "PVX",
    "SCA",
    "SCG",
    "SED",
    "SNG",
    "TBL",
    "THO",
    "TIC",
    "TBX",
    "TPO",
    "TSP",
    "OFF"
];

#[rustfmt::skip] // Keep constants with important order vertical for maintenance
const FX_MIXER_COMMAND_V3 : [&'static str; 36] = [
    "VMV",
    "XCM",
    "XCF",
    "XCW",
    "XCR",
    "XDT",
    "XDF",
    "XDW",
    "XDR",
    "XRS",
    "XRD",
    "XRM",
    "XRF",
    "XRW",
    "XRZ",
    "VCH",
    "VCD",
    "VRE",
    "VT1",
    "VT2",
    "VT3",
    "VT4",
    "VT5",
    "VT6",
    "VT7",
    "VT8",
    "DJF",
    "IVO",
    "ICH",
    "IDE",
    "IRE",
    "IV2",
    "IC2",
    "ID2",
    "IR2",
    "USB",
];

const COMMANDS_V3: [&'static str; concat_arrays_size!(SEQ_COMMAND_V3, FX_MIXER_COMMAND_V3)] =
    concat_arrays!(SEQ_COMMAND_V3, FX_MIXER_COMMAND_V3);

//////////////////////////////////////////
// MARK: V4 commands
//////////////////////////////////////////

#[rustfmt::skip] // Keep constants with important order vertical for maintenance
const FX_MIXER_COMMAND_V4 : [&'static str; 45] = [
    "VMV",
    "XCM",
    "XCF",
    "XCW",
    "XCR",
    "XDT",
    "XDF",
    "XDW",
    "XDR",
    "XRS",
    "XRD",
    "XRM",
    "XRF",
    "XRW",
    "XRZ",
    "VCH",
    "VDE",
    "VRE",
    "VT1",
    "VT2",
    "VT3",
    "VT4",
    "VT5",
    "VT6",
    "VT7",
    "VT8",
    "DJC",
    "VIN",
    "ICH",
    "IDE",
    "IRE",
    "VI2",
    "IC2",
    "ID2",
    "IR2",
    "USB",

    "DJR", // 0x3F
    "DJT", // 0x40
    "EQM", // 0x41
    "EQI", // 0x42
    "INS", // 0x43
    "RTO", // 0x44
    "ARC", // 0x45
    "GGR", // 0x46
    "NXT", // 0x47
];

const COMMANDS_V4: [&'static str; concat_arrays_size!(SEQ_COMMAND_V3, FX_MIXER_COMMAND_V4)] =
    concat_arrays!(SEQ_COMMAND_V3, FX_MIXER_COMMAND_V4);

impl FX {
    pub const V4_SIZE: usize = 2;

    pub(crate) fn from_reader(reader: &mut Reader) -> M8Result<Self> {
        Ok(Self {
            command: reader.read(),
            value: reader.read(),
        })
    }

    pub fn write(self, w: &mut Writer) {
        w.write(self.command);
        w.write(self.value);
    }

    pub fn is_empty(self) -> bool {
        self.command == 0xFF
    }

    pub fn print(&self, fx: FxCommands, pack: CommandPack) -> String {
        if self.is_empty() {
            format!("---  ")
        } else {
            let c = self.format_command(fx, pack);
            format!("{}{:02x}", c, self.value)
        }
    }

    /// Retrieve command names for a given version
    pub fn fx_command_names(ver: Version) -> FxCommands {
        if ver.at_least(4, 0) {
            FxCommands {
                commands: &COMMANDS_V4,
            }
        } else if ver.at_least(3, 0) {
            FxCommands {
                commands: &COMMANDS_V3,
            }
        } else {
            FxCommands {
                commands: &COMMANDS_V2,
            }
        }
    }

    fn format_command(&self, fx: FxCommands, instr: CommandPack) -> String {
        match fx.try_render(self.command) {
            Some(s) => String::from(s),
            None => {
                if instr.accepts(self.command) {
                    match instr.try_render(self.command) {
                        Some(v) => String::from(v),
                        None => format!("I{:02X}", self.command - 0x80),
                    }
                } else {
                    format!("?{:02x}", self.command)
                }
            }
        }
    }
}
