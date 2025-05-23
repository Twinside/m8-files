use crate::instruments::common::*;
use crate::reader::*;
use crate::version::*;
use crate::writer::Writer;
use array_concat::concat_arrays;
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;

use arr_macro::arr;

use super::dests;
use super::CommandPack;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct FmAlgo(pub u8);

const FM_ALGO_STRINGS: [&str; 0x0C] = [
    "A>B>C>D",
    "[A+B]>C>D",
    "[A>B+C]>D",
    "[A>B+A>C]>D",
    "[A+B+C]>D",
    "[A>B>C]+D",
    "[A>B>C]+[A>B>D]",
    "[A>B]+[C>D]",
    "[A>B]+[A>C]+[A>D]",
    "[A>B]+[A>C]+D",
    "[A>B]+C+D",
    "A+B+C+D",
];

impl TryFrom<u8> for FmAlgo {
    type Error = ParseError;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        if (value as usize) < FM_ALGO_STRINGS.len() {
            Ok(FmAlgo(value))
        } else {
            Err(ParseError(format!("Invalid fm algo {}", value)))
        }
    }
}

impl FmAlgo {
    pub fn id(self) -> u8 {
        let FmAlgo(v) = self;
        v
    }

    pub fn str(self) -> &'static str {
        FM_ALGO_STRINGS[self.id() as usize]
    }
}

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(IntoPrimitive, TryFromPrimitive, PartialEq, Copy, Clone, Default, Debug)]
pub enum FMWave {
    #[default]
    SIN,
    SW2,
    SW3,
    SW4,
    SW5,
    SW6,
    TRI,
    SAW,
    SQR,
    PUL,
    IMP,
    NOI,
    NLP,
    NHP,
    NBP,
    CLK,
    // v4.1 addition
    W09,
    W0A,
    W0B,
    W0C,
    W0D,
    W0E,
    W0F,
    W10,
    W11,
    W12,
    W13,
    W14,
    W15,
    W16,
    W17,
    W18,
    W19,
    W1A,
    W1B,
    W1C,
    W1D,
    W1E,
    W1F,
    W20,
    W21,
    W22,
    W23,
    W24,
    W25,
    W26,
    W27,
    W28,
    W29,
    W2A,
    W2B,
    W2C,
    W2D,
    W2E,
    W2F,
    W30,
    W31,
    W32,
    W33,
    W34,
    W35,
    W36,
    W37,
    W38,
    W39,
    W3A,
    W3B,
    W3C,
    W3D,
    W3E,
    W3F,
    W40,
    W41,
    W42,
    W43,
    W44,
    W45,
}

#[rustfmt::skip] // Keep constants with important order vertical for maintenance
const FM_FX_BASE_COMMANDS : [&'static str; CommandPack::BASE_INSTRUMENT_COMMAND_COUNT] = [
    "VOL",
    "PIT",
    "FIN",
    "ALG",
    "FM1",
    "FM2",
    "FM3",
    "FM4",
    "FLT",
    "CUT",
    "RES",
    "AMP",
    "LIM",
    "PAN",
    "DRY",
    
    "SCH",
    "SDL",
    "SRV",
];

#[rustfmt::skip] // Keep constants with important order vertical for maintenance
const FM_FX_COMMANDS_UPTO_5 : [&'static str; CommandPack::BASE_INSTRUMENT_COMMAND_COUNT + 1] =
    concat_arrays!(FM_FX_BASE_COMMANDS, ["FMP"]);

#[rustfmt::skip] // Keep constants with important order vertical for maintenance
const FM_FX_COMMANDS_FROM_6 : [&'static str; CommandPack::BASE_INSTRUMENT_COMMAND_COUNT + 2] =
    concat_arrays!(FM_FX_BASE_COMMANDS, ["SNC", "ERR"]);

#[rustfmt::skip] // Keep constants with important order vertical for maintenance
const DESTINATIONS : [&'static str; 15] = [
    dests::OFF,
    dests::VOLUME,
    dests::PITCH,

    "MOD1",
    "MOD2",
    "MOD3",
    "MOD4",
    dests::CUTOFF,
    dests::RES,
    dests::AMP,
    dests::PAN,
    dests::MOD_AMT,
    dests::MOD_RATE,
    dests::MOD_BOTH,
    dests::MOD_BINV,
];

#[derive(PartialEq, Debug, Default, Clone)]
pub struct Operator {
    pub shape: FMWave,
    pub ratio: u8,
    pub ratio_fine: u8,
    pub level: u8,
    pub feedback: u8,
    pub retrigger: u8,
    pub mod_a: u8,
    pub mod_b: u8,
}

#[derive(PartialEq, Debug, Clone)]
pub struct FMSynth {
    pub number: u8,
    pub name: String,
    pub transpose: bool,
    pub table_tick: u8,
    pub synth_params: SynthParams,

    pub algo: FmAlgo,
    pub operators: [Operator; 4],
    pub mod1: u8,
    pub mod2: u8,
    pub mod3: u8,
    pub mod4: u8,
}

impl FMSynth {
    const MOD_OFFSET: usize = 2;

    pub fn command_name(&self, ver: Version) -> &'static [&'static str] {
        if ver.at_least(6, 0) {
            &FM_FX_COMMANDS_FROM_6
        } else {
            &FM_FX_COMMANDS_UPTO_5
        }
    }

    pub fn destination_names(&self, _ver: Version) -> &'static [&'static str] {
        &DESTINATIONS
    }

    /// List of all the applyable filter types for the instrument
    pub fn filter_types(&self, _ver: Version) -> &'static [&'static str] {
        &COMMON_FILTER_TYPES
    }

    pub fn human_readable_filter(&self) -> &'static str {
        COMMON_FILTER_TYPES[self.synth_params.filter_type as usize]
    }

    pub fn write(&self, ver: Version, w: &mut Writer) {
        w.write_string(&self.name, 12);
        w.write(TranspEq::from(ver, self.transpose, self.synth_params.associated_eq).into());
        w.write(self.table_tick);
        w.write(self.synth_params.volume);
        w.write(self.synth_params.pitch);
        w.write(self.synth_params.fine_tune);

        w.write(self.algo.0);

        for op in &self.operators {
            w.write(op.shape.into());
        }

        for op in &self.operators {
            w.write(op.ratio);
            w.write(op.ratio_fine);
        }

        for op in &self.operators {
            w.write(op.level);
            w.write(op.feedback);
        }

        for op in &self.operators {
            w.write(op.mod_a);
        }

        for op in &self.operators {
            w.write(op.mod_b);
        }

        w.write(self.mod1);
        w.write(self.mod2);
        w.write(self.mod3);
        w.write(self.mod4);

        self.synth_params.write(ver, w, FMSynth::MOD_OFFSET);
    }

    pub fn from_reader(
        ver: Version,
        reader: &mut Reader,
        number: u8,
        version: Version,
    ) -> M8Result<Self> {
        let name = reader.read_string(12);
        let transp_eq = TranspEq::from_version(ver, reader.read());
        let table_tick = reader.read();
        let volume = reader.read();
        let pitch = reader.read();
        let fine_tune = reader.read();

        let algo = reader.read();
        let mut operators: [Operator; 4] = arr![Operator::default(); 4];
        if version.at_least(1, 4) {
            for i in 0..4 {
                let wav_code = reader.read();
                operators[i].shape = FMWave::try_from(wav_code)
                    .map_err(|_| ParseError(format!("Invalid fm wave {}", wav_code)))?;
            }
        }
        for i in 0..4 {
            operators[i].ratio = reader.read();
            operators[i].ratio_fine = reader.read();
        }
        for i in 0..4 {
            operators[i].level = reader.read();
            operators[i].feedback = reader.read();
        }
        for i in 0..4 {
            operators[i].mod_a = reader.read();
        }
        for i in 0..4 {
            operators[i].mod_b = reader.read();
        }
        let mod1 = reader.read();
        let mod2 = reader.read();
        let mod3 = reader.read();
        let mod4 = reader.read();

        let synth_params = if version.at_least(3, 0) {
            SynthParams::from_reader3(
                ver,
                reader,
                volume,
                pitch,
                fine_tune,
                transp_eq.eq,
                FMSynth::MOD_OFFSET,
            )?
        } else {
            SynthParams::from_reader2(reader, volume, pitch, fine_tune)?
        };

        Ok(FMSynth {
            number,
            name,
            transpose: transp_eq.transpose,
            table_tick,
            synth_params,

            algo: FmAlgo(algo),
            operators,
            mod1,
            mod2,
            mod3,
            mod4,
        })
    }
}
