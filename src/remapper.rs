use std::collections::HashSet;

use arr_macro::arr;
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{
    songs::{Song, V4_1_OFFSETS, V4_OFFSETS}, Instrument, Version, FX
};

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(IntoPrimitive, TryFromPrimitive, PartialEq, Copy, Clone, Default, Debug)]
pub enum MoveKind {
    #[default]
    EQ,
    INS,
    PHR,
    CHN,
    TBL,
}

pub trait RemapperDescriptorBuilder {
    fn moved(&mut self, kind: MoveKind, from: usize, to: usize);
}

fn make_mapping<const C: usize>(offset: u8) -> [u8; C] {
    let mut arr = [0 as u8; C];
    for i in 0..arr.len() {
        arr[i] = i as u8 + offset;
    }

    arr
}

pub struct EqMapping {
    /// List all the command ID referencing an EQ as
    /// value. Depend on the song version number.
    pub eq_tracking_commands : Vec<u8>,

    /// Mapping from the "from" song eq index to the "to" song
    /// eq index
    pub mapping: Vec<u8>,

    /// Eqs to be moved during the remapping
    /// index in the "from" song
    pub to_move: Vec<u8>,
}

impl EqMapping {
    pub fn default_ver(ver: Version) -> EqMapping {
        let command_names = FX::fx_command_names(ver);
        let eq_tracking_commands = command_names.find_indices(&EQ_TRACKING_COMMAND_NAMES);

        if ver.at_least(4, 1) {
            EqMapping {
                eq_tracking_commands,
                mapping: vec![0; V4_1_OFFSETS.instrument_eq_count],
                to_move: vec![],
            }
        } else {
            EqMapping {
                eq_tracking_commands,
                mapping: vec![0; V4_OFFSETS.instrument_eq_count],
                to_move: vec![],
            }
        }
    }

    pub fn describe<T: RemapperDescriptorBuilder>(&self, builder: &mut T) {
        for ix in &self.to_move {
            let ixu = *ix as usize;
            builder.moved(MoveKind::EQ, ixu, self.mapping[ixu] as usize)
        }
    }

    pub fn print(&self) -> String {
        let mut acc = String::new();

        for e in self.to_move.iter() {
            let new_ix = self.mapping[*e as usize];
            acc = format!("{acc} Eq {e} => {new_ix}\n");
        }

        acc
    }
}

/// For every instrument, it's destination instrument
pub struct InstrumentMapping {
    /// List all the command ID referencing an instrument as
    /// value. Depend on the song version number.
    pub instrument_tracking_commands : Vec<u8>,

    /// Mapping from the "from" song instrument index to the "to"
    /// song instrument index
    pub mapping: [u8; Song::N_INSTRUMENTS],

    /// Instruments to be moved during the remapping
    /// index in the "from" song
    pub to_move: Vec<u8>,
}

impl InstrumentMapping {
    pub fn describe<T: RemapperDescriptorBuilder>(&self, builder: &mut T) {
        for ix in &self.to_move {
            let ixu = *ix as usize;
            builder.moved(MoveKind::INS, ixu, self.mapping[ixu] as usize)
        }
    }

    pub fn print(&self) -> String {
        let mut acc = String::new();

        for e in self.to_move.iter() {
            let new_ix = self.mapping[*e as usize];
            acc = format!("{acc} instr {e} => {new_ix}\n");
        }

        acc
    }

    pub fn new(instrument_tracking_commands: Vec<u8>) -> Self {
        Self {
            instrument_tracking_commands,
            mapping: make_mapping(0),
            to_move: vec![],
        }
    }
}

pub struct TableMapping {
    /// List all the command ID referencing a table as
    /// value. Depend on the song version number.
    pub table_tracking_commands : Vec<u8>,

    /// Mapping from the "from" song index to the to
    pub mapping: [u8; Song::N_TABLES],

    /// Table to be moved during remapping
    pub to_move: Vec<u8>,
}

impl TableMapping {
    pub fn describe<T: RemapperDescriptorBuilder>(&self, builder: &mut T) {
        for ix in &self.to_move {
            let ixu = *ix as usize;
            builder.moved(MoveKind::TBL, ixu, self.mapping[ixu] as usize)
        }
    }

    pub fn remap_table(&mut self, from: u8, to: u8) {
        self.mapping[from as usize] = to;
        self.to_move.push(from);
    }

    fn new(table_tracking_commands: Vec<u8>) -> Self {
        Self {
            table_tracking_commands,
            mapping: make_mapping(Song::N_TABLES as u8),
            to_move: vec![],
        }
    }
}

pub struct PhraseMapping {
    /// Mapping from the "from" song phrase index to
    /// the "to" phrase index
    pub mapping: [u8; Song::N_PHRASES],

    /// Phrases to be moved during the remapping
    /// index in the "from" song
    pub to_move: Vec<u8>,
}

impl PhraseMapping {
    pub fn describe<T: RemapperDescriptorBuilder>(&self, builder: &mut T) {
        for ix in &self.to_move {
            let ixu = *ix as usize;
            builder.moved(MoveKind::PHR, ixu, self.mapping[ixu] as usize)
        }
    }

    pub fn print(&self) -> String {
        let mut acc = String::new();

        for e in self.to_move.iter() {
            let new_ix = self.mapping[*e as usize];
            acc = format!("{acc} phrase {e} => {new_ix}\n");
        }

        acc
    }
}

impl Default for PhraseMapping {
    fn default() -> Self {
        Self {
            mapping: make_mapping(0),
            to_move: vec![],
        }
    }
}

pub struct ChainMapping {
    pub mapping: [u8; Song::N_CHAINS],
    pub to_move: Vec<u8>,
}

impl ChainMapping {
    pub fn describe<T: RemapperDescriptorBuilder>(&self, builder: &mut T) {
        for ix in &self.to_move {
            let ixu = *ix as usize;
            builder.moved(MoveKind::CHN, ixu, self.mapping[ixu] as usize)
        }
    }

    pub fn print(&self) -> String {
        let mut acc = String::new();

        for e in self.to_move.iter() {
            let new_ix = self.mapping[*e as usize];
            acc = format!("{acc} chain {e} => {new_ix}\n");
        }

        acc
    }
}

impl Default for ChainMapping {
    fn default() -> Self {
        Self {
            mapping: make_mapping(0),
            to_move: vec![],
        }
    }
}

pub struct Remapper {
    pub eq_mapping: EqMapping,
    pub instrument_mapping: InstrumentMapping,
    pub table_mapping: TableMapping,
    pub phrase_mapping: PhraseMapping,
    pub chain_mapping: ChainMapping,
}

/// Iter on all instruments to find allocated Eqs
fn find_referenced_eq(song: &Song) -> Vec<bool> {
    // flags on eqs in "to"
    let mut allocated_eqs = vec![false; song.eqs.len()];

    for instr in &song.instruments {
        match instr.equ() {
            None => {}
            Some(eq) => {
                let equ = eq as usize;
                if equ < allocated_eqs.len() {
                    allocated_eqs[equ] = true
                }
            }
        }
    }

    // TODO: track eqi command....

    allocated_eqs
}

fn find_allocated_instruments(song: &Song) -> [bool; Song::N_INSTRUMENTS] {
    let mut allocated_instr = arr![false; 128];

    for (i, instr) in song.instruments.iter().enumerate() {
        match instr {
            Instrument::None => {}
            _ => allocated_instr[i] = true,
        }
    }

    allocated_instr
}

fn find_referenced_phrases(song: &Song) -> [bool; Song::N_PHRASES] {
    let mut allocated_phrases = arr![false; 255];
    for chain in &song.chains {
        for step in &chain.steps {
            let phrase = step.phrase as usize;
            if phrase < Song::N_PHRASES {
                allocated_phrases[phrase] = true;
            }
        }
    }

    for (phrase_id, phrase) in song.phrases.iter().enumerate() {
        if !phrase.is_empty() {
            allocated_phrases[phrase_id] = true;
        }
    }

    allocated_phrases
}

fn find_referenced_chains(song: &Song) -> [bool; Song::N_CHAINS] {
    let mut allocated_chains = arr![false; 255];
    for chain in song.song.steps.iter() {
        let chain = *chain as usize;
        if chain < Song::N_CHAINS {
            allocated_chains[chain] = true;
        }
    }

    for (i, chain) in song.chains.iter().enumerate() {
        if !chain.is_empty() {
            allocated_chains[i] = true
        }
    }

    allocated_chains
}

/// Try to allocate in the new song by keeping previous numbers
fn try_allocate(allocation_state: &[bool], previous_id: u8) -> Option<usize> {
    let prev = previous_id as usize;
    if !allocation_state[prev] {
        Some(prev)
    } else {
        match allocation_state[prev..].iter().position(|v| !v) {
            // we take a slot above the existing one
            Some(p) => Some(p + prev),
            // nothing else worked, just try to find any free slot
            None => allocation_state.iter().position(|v| !v),
        }
    }
}

const INSTRUMENT_TRACKING_COMMAND_NAMES : [&'static str; 2] = ["INS", "NXT"];

const TABLE_TRACKING_COMMAND_NAMES : [&'static str; 1] = ["TBX"];

const EQ_TRACKING_COMMAND_NAMES : [&'static str; 1] = ["EQI"];

/// brief struture to hold structures used to allocate instruments
struct InstrumentAllocatorState<'a> {
    from_song: &'a Song,
    to_song: &'a Song,

    /// cycle detection, can happen if we follow
    /// INS/NXT references
    seen_instruments: HashSet<u8>,

    /// cycle detection, can happen if we follow
    /// INS/NXT/TBX references
    seen_tables: HashSet<u8>,

    /// flags on instruments in "from"
    instrument_flags: [bool; Song::N_INSTRUMENTS],
    /// flags on tables in "from"
    table_flags: [bool; Song::N_TABLES],
    /// flags on eqsin "from"
    eq_flags: Vec<bool>,
    // flags on eqsin "to"
    allocated_eqs: Vec<bool>,
    allocated_instruments: [bool; Song::N_INSTRUMENTS],
    instrument_mapping: InstrumentMapping,
    eq_mapping: EqMapping,
    table_mapping: TableMapping,
}


impl<'a> InstrumentAllocatorState<'a> {
    fn new(from_song: &'a Song, to_song: &'a Song) -> InstrumentAllocatorState<'a> {
        let fx_commands_names = crate::FX::fx_command_names(from_song.version);
        let instrument_tracking_commands =
            fx_commands_names.find_indices(&INSTRUMENT_TRACKING_COMMAND_NAMES);
        let table_tracking_commands =
            fx_commands_names.find_indices(&TABLE_TRACKING_COMMAND_NAMES);

        InstrumentAllocatorState {
            from_song,
            to_song,

            table_mapping: TableMapping::new(table_tracking_commands),
            seen_instruments: HashSet::new(),
            seen_tables: HashSet::new(),

            allocated_eqs: find_referenced_eq(to_song),
            allocated_instruments: find_allocated_instruments(to_song),
            eq_flags: vec![false; from_song.eqs.len()],
            instrument_flags: arr![false; 128],
            table_flags: arr![false; 256],
            instrument_mapping: InstrumentMapping::new(instrument_tracking_commands),
            eq_mapping: EqMapping::default_ver(to_song.version)
        }
    }

    fn allocate_eq(&mut self, equ: usize) -> Result<(), String> {
        self.eq_flags[equ as usize] = true;
        let from_eq = &self.from_song.eqs[equ];
        // try to find an already exisint Eq with same parameters
        match self.to_song.eqs.iter().position(|to_eq| to_eq == from_eq) {
            Some(eq_idx) if (eq_idx as usize) < self.eq_mapping.mapping.len() => {
                self.eq_mapping.mapping[equ] = eq_idx as u8
            }
            Some(_) | None => match try_allocate(&self.allocated_eqs, equ as u8) {
                None => {
                    return Err(format!("No more available eqs"))
                }
                Some(eq_slot) => {
                    self.allocated_eqs[eq_slot] = true;
                    self.eq_mapping.mapping[equ] = eq_slot as u8;
                    self.eq_mapping.to_move.push(equ as u8);
                }
            },
        }

        Ok(())
    }

    fn is_touching_instrument(&self, cmd: u8) -> bool {
        self.instrument_mapping.instrument_tracking_commands.contains(&cmd)
    }

    fn is_touching_table(&self, cmd: u8) -> bool {
        self.table_mapping.table_tracking_commands.contains(&cmd)
    }

    fn is_touching_eq(&self, cmd: u8) -> bool {
        self.eq_mapping.eq_tracking_commands.contains(&cmd)
    }

    fn touch_table(&mut self, table_ix: usize) -> Result<(), String> {

        // out of bound instrument, dont bother or if already allocated
        if table_ix >= Song::N_TABLES || self.table_flags[table_ix] {
            return Ok(());
        }

        if self.seen_tables.contains(&(table_ix as u8)) {
            return Err(format!("Detected cycles in tables"))
        }

        self.seen_tables.insert(table_ix as u8);

        // if the table contains NXT command, we need to track NXT'ed command
        let instrument_table = &self.from_song.tables[table_ix];
        for table_step in instrument_table.steps.iter() {
            for fx in table_step.all_fx() {
                if self.is_touching_instrument(fx.command) {
                    self.touch_instrument(fx.value as usize)?;
                }

                if self.is_touching_table(fx.command) {
                    self.touch_table(fx.command as usize)?;
                }

                if self.is_touching_eq(fx.command) {
                    self.touch_eq(fx.command as usize)?;
                }
            }
        }

        // ok so we are not tied to an instruments, we must
        // allocate a slot for ourselves.
        if table_ix > Song::N_INSTRUMENTS {
            todo!("Need to find a fresh slot and mark the mapping")
        }

        self.seen_tables.remove(&(table_ix as u8));

        Ok(())
    }

    fn touch_eq(&mut self, eq_ix: usize) -> Result<(), String> {
        if eq_ix < self.eq_flags.len() && !self.eq_flags[eq_ix] {
            self.allocate_eq(eq_ix)?;
        }
        Ok(())
    }

    fn touch_instrument(
        &mut self,
        instr_ix: usize) -> Result<(), String> {
        let from_song = self.from_song;
        let to_song = self.to_song;

        // out of bound instrument, dont bother or if already allocated
        if instr_ix >= Song::N_INSTRUMENTS || self.instrument_flags[instr_ix] {
            return Ok(());
        }

        if self.seen_instruments.contains(&(instr_ix as u8)) {
            return Err(format!("Detected cycles in instruments"))
        }

        self.seen_instruments.insert(instr_ix as u8);

        let mut instr = from_song.instruments[instr_ix].clone();

        // first we search the new EQ
        if let Some(equ) = instr.equ() {
            let equ = equ as usize;

            if equ < self.eq_flags.len() && !self.eq_flags[equ] {
                self.allocate_eq(equ)?;
            }
            // finally update our Eq in our local copy
            instr.set_eq(self.eq_mapping.mapping[equ]);
        }
        
        self.touch_table(instr_ix)?;

        self.instrument_flags[instr_ix] = true;
        match to_song.instruments.iter().position(|i| i == &instr) {
            // horray we have a matching instrument, reuse it
            Some(to_instr_ix) => {
                self.instrument_mapping.mapping[instr_ix] = to_instr_ix as u8
            }
            // no luck, allocate a fresh one
            None => match try_allocate(&self.allocated_instruments, instr_ix as u8) {
                None => {
                    return Err(format!(
                        "No more available instrument slots for instrument {instr_ix}"
                    ))
                }
                Some(to_instr_ix) => {
                    self.instrument_mapping.mapping[instr_ix] = to_instr_ix as u8;
                    self.allocated_instruments[to_instr_ix] = true;
                    self.instrument_mapping.to_move.push(instr_ix as u8)
                }
            },
        };

        self.seen_instruments.remove(&(instr_ix as u8));
        Ok(())
    }

}

impl Remapper {
    pub fn default_ver(ver: Version) -> Self {
        let command_names = crate::FX::fx_command_names(ver);
        let instrument_tracking_commands =
            command_names.find_indices(&INSTRUMENT_TRACKING_COMMAND_NAMES);
        let table_tracking_commands =
            command_names.find_indices(&TABLE_TRACKING_COMMAND_NAMES);

        Self {
            eq_mapping: EqMapping::default_ver(ver),
            instrument_mapping: InstrumentMapping::new(instrument_tracking_commands),
            table_mapping: TableMapping::new(table_tracking_commands ),
            phrase_mapping: Default::default(),
            chain_mapping: Default::default(),
        }
    }

    pub fn describe<T: RemapperDescriptorBuilder>(&self, builder: &mut T) {
        self.eq_mapping.describe(builder);
        self.instrument_mapping.describe(builder);
        self.table_mapping.describe(builder);
        self.phrase_mapping.describe(builder);
        self.chain_mapping.describe(builder);
    }

    pub fn out_chain(&self, chain_id: u8) -> u8 {
        self.chain_mapping.mapping[chain_id as usize]
    }

    pub fn print(&self) -> String {
        let eq = self.eq_mapping.print();
        let instr = self.instrument_mapping.print();
        let phrase = self.phrase_mapping.print();
        let chain = self.chain_mapping.print();
        format!("{eq}\n{instr}\n{phrase}\n{chain}")
    }

    fn allocate_chains<'a, IT>(
        from_song: &Song,
        to_song: &Song,
        phrase_mapping: &PhraseMapping,
        from_chains_ids: IT,
    ) -> Result<ChainMapping, String>
    where
        IT: Iterator<Item = &'a u8>,
    {
        let mut seen_chain: [bool; Song::N_CHAINS] = arr![false; 255];
        let mut allocated_chains = find_referenced_chains(to_song);
        let mut mapping: [u8; Song::N_CHAINS] = make_mapping(0);
        let mut to_move = vec![];

        for chain_id in from_chains_ids {
            let chain_id = *chain_id as usize;
            if chain_id >= Song::N_CHAINS || seen_chain[chain_id] {
                continue;
            }

            seen_chain[chain_id] = true;
            let to_chain = from_song.chains[chain_id].map(phrase_mapping);

            match to_song
                .chains
                .iter()
                .position(|c| c.steps == to_chain.steps)
            {
                Some(c) => mapping[chain_id] = c as u8,
                None => match try_allocate(&allocated_chains, chain_id as u8) {
                    None => {
                        return Err(format!(
                            "No more available chain slots for chain {chain_id}"
                        ))
                    }
                    Some(free_slot) => {
                        allocated_chains[free_slot] = true;
                        mapping[chain_id] = free_slot as u8;
                        to_move.push(chain_id as u8);
                    }
                },
            }
        }

        Ok(ChainMapping { mapping, to_move })
    }

    fn allocate_phrases<'a, IT>(
        from_song: &Song,
        to_song: &Song,
        instrument_mapping: &InstrumentMapping,
        table_mapping: &TableMapping,
        from_chains_ids: IT,
    ) -> Result<PhraseMapping, String>
    where
        IT: Iterator<Item = &'a u8>,
    {
        let mut allocated_phrases = find_referenced_phrases(to_song);

        let mut seen_phrase: [bool; Song::N_PHRASES] = arr![false; 0xFF];
        let mut phrase_mapping: [u8; Song::N_PHRASES] = arr![0 as u8; 0xFF];

        let mut to_move = vec![];

        for chain_id in from_chains_ids {
            let from_chain = &from_song.chains[*chain_id as usize];

            for chain_step in from_chain.steps.iter() {
                let phrase_ix = chain_step.phrase as usize;

                if phrase_ix >= Song::N_PHRASES || seen_phrase[phrase_ix] {
                    continue;
                }

                seen_phrase[phrase_ix] = true;
                let phrase = from_song.phrases[phrase_ix].map_instruments(instrument_mapping, table_mapping);
                match to_song.phrases.iter().position(|p| p.steps == phrase.steps) {
                    Some(known) => phrase_mapping[phrase_ix] = known as u8,
                    None => match try_allocate(&allocated_phrases, phrase_ix as u8) {
                        None => {
                            return Err(format!(
                                "No more available phrase slots for phrase {phrase_ix}"
                            ))
                        }
                        Some(slot) => {
                            to_move.push(phrase_ix as u8);
                            allocated_phrases[slot] = true;
                            phrase_mapping[phrase_ix] = slot as u8;
                        }
                    },
                }
            }
        }

        Ok(PhraseMapping {
            mapping: phrase_mapping,
            to_move,
        })
    }

    /// Find location in destination song for EQ and instruments
    fn allocate_eq_and_instruments<'a, IT>(
        from_song: &'a Song,
        to_song: &'a Song,
        from_chains_ids: IT,
    ) -> Result<InstrumentAllocatorState<'a>, String>
    where
        IT: Iterator<Item = &'a u8>,
    {
        let mut alloc_state =
            InstrumentAllocatorState::new(from_song, to_song);

        for chain_id in from_chains_ids {
            let from_chain = &from_song.chains[*chain_id as usize];

            for chain_step in &from_chain.steps {
                let phrase_id = chain_step.phrase as usize;
                if phrase_id >= Song::N_PHRASES {
                    continue;
                }

                let phrase = &from_song.phrases[phrase_id];

                for step in &phrase.steps {
                    alloc_state.touch_instrument(step.instrument as usize)?;

                    for fx in step.all_fx() {
                        if alloc_state.is_touching_instrument(fx.command) {
                            alloc_state.touch_instrument(fx.value as usize)?;
                        }

                        if alloc_state.is_touching_table(fx.command) {
                            alloc_state.touch_table(fx.command as usize)?;
                        }

                        if alloc_state.is_touching_eq(fx.command) {
                            alloc_state.touch_eq(fx.command as usize)?;
                        }
                    }
                }
            }
        }

        Ok(alloc_state)
    }

    pub fn create<'a, IT>(from_song: &Song, to_song: &Song, chains: IT) -> Result<Remapper, String>
    where
        IT: Iterator<Item = &'a u8>,
    {
        let chain_vec: Vec<u8> = chains.map(|v| *v).collect();

        // eqs from "from" to "to"
        let alloc_state =
            Remapper::allocate_eq_and_instruments(
                from_song,
                to_song,
                chain_vec.iter())?;

        let phrase_mapping =
            Remapper::allocate_phrases(
                from_song,
                to_song,
                &alloc_state.instrument_mapping,
                &alloc_state.table_mapping,
                chain_vec.iter())?;

        let chain_mapping =
            Remapper::allocate_chains(from_song, to_song, &phrase_mapping, chain_vec.iter())?;

        Ok(Self {
            eq_mapping: alloc_state.eq_mapping,
            instrument_mapping: alloc_state.instrument_mapping,
            table_mapping: alloc_state.table_mapping,
            phrase_mapping,
            chain_mapping,
        })
    }

    /// Same as apply but the same song is the source and destination
    pub fn renumber(&self, song: &mut Song) {
        // move eq
        for equ in self.eq_mapping.to_move.iter() {
            let equ = *equ as usize;
            let to_index = self.eq_mapping.mapping[equ];
            song.eqs[to_index as usize] = song.eqs[equ].clone();
            song.eqs[equ].clear();
        }

        // move instr
        for instr_id in self.instrument_mapping.to_move.iter() {
            let instr_id = *instr_id as usize;
            let to_index = self.instrument_mapping.mapping[instr_id] as usize;
            let instr = song.instruments[instr_id].clone();

            song.tables[to_index] = song.tables[instr_id].clone();
            song.instruments[to_index] = instr;
            song.instruments[instr_id] = Instrument::None;
        }

        // move table
        for table_id in self.table_mapping.to_move.iter() {
            let table_id = *table_id as usize;
            let to_index = self.table_mapping.mapping[table_id] as usize;
            let table = song.tables[table_id].clone();

            song.tables[to_index] = table;
            song.tables[table_id].clear();
        }

        // remap eq in instr
        let eq_count = song.eq_count();
        for instr_id in 0..Song::N_INSTRUMENTS {
            let instr = &mut song.instruments[instr_id];

            if let Some(eq) = instr.equ() {
                let eq = eq as usize;
                if eq < eq_count {
                    instr.set_eq(self.eq_mapping.mapping[eq]);
                }
            }
        }

        // move phrases
        for phrase_id in self.phrase_mapping.to_move.iter() {
            let phrase_id = *phrase_id as usize;
            let to_index = self.phrase_mapping.mapping[phrase_id];
            song.phrases[to_index as usize] = song.phrases[phrase_id].clone();
            song.phrases[phrase_id].clear()
        }

        // remap instr in phrases
        for phrase_id in 0..Song::N_PHRASES {
            song.phrases[phrase_id] =
                song.phrases[phrase_id].map_instruments(&self.instrument_mapping, &self.table_mapping);
        }

        // move chain
        for chain_id in self.chain_mapping.to_move.iter() {
            let chain_id = *chain_id as usize;
            let to_index = self.chain_mapping.mapping[chain_id];
            song.chains[to_index as usize] = song.chains[chain_id].clone();
            song.chains[chain_id].clear();
        }

        // remap chain
        for chain_id in 0..Song::N_CHAINS {
            song.chains[chain_id] = song.chains[chain_id].map(&self.phrase_mapping)
        }
    }

    /// apply the reampping, cannot fail once mapping has been created
    pub fn apply(&self, from: &Song, to: &mut Song) {
        for equ in self.eq_mapping.to_move.iter() {
            let equ = *equ as usize;
            let to_index = self.eq_mapping.mapping[equ];
            to.eqs[to_index as usize] = from.eqs[equ].clone();
        }

        for instr_id in self.instrument_mapping.to_move.iter() {
            let instr_id = *instr_id as usize;
            let to_index = self.instrument_mapping.mapping[instr_id] as usize;
            let mut instr = from.instruments[instr_id].clone();

            if let Some(eq) = instr.equ() {
                let eq = eq as usize;
                if eq < to.eq_count() {
                    instr.set_eq(self.eq_mapping.mapping[eq]);
                }
            }

            to.tables[to_index] = from.tables[instr_id]
                .map_instr(&self.instrument_mapping, &self.table_mapping);
            to.instruments[to_index] = instr;
        }

        // move table
        for table_id in self.table_mapping.to_move.iter() {
            let table_id = *table_id as usize;
            let to_index = self.table_mapping.mapping[table_id] as usize;
            to.tables[to_index] = from.tables[table_id]
                .map_instr(&self.instrument_mapping, &self.table_mapping);
        }

        for phrase_id in self.phrase_mapping.to_move.iter() {
            let phrase_id = *phrase_id as usize;
            let to_index = self.phrase_mapping.mapping[phrase_id];
            to.phrases[to_index as usize] =
                from.phrases[phrase_id].map_instruments(&self.instrument_mapping, &self.table_mapping);
        }

        for chain_id in self.chain_mapping.to_move.iter() {
            let chain_id = *chain_id as usize;
            let to_index = self.chain_mapping.mapping[chain_id];
            to.chains[to_index as usize] = from.chains[chain_id].map(&self.phrase_mapping);
        }
    }
}
