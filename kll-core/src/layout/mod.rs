// Copyright 2021 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

mod test;

// ----- Crates -----

use super::*;
use heapless::FnvIndexMap;
use log::*;

// ----- Enums -----

enum LayerProcessMode {
    Layer,
    TriggerType,
    IndexA,
    IndexB,
    TriggerSize,
    Triggers(u8),
}

// ----- Structs -----

/// The LayerLookup struct is used as a guide for the KLL state machine
/// It is a (mostly) constant lookup table which can give you all possible
/// TriggerGuides for a specified input.
/// Each TriggerGuide has a connected ResultGuide which is also stored in this datastructure.
///
/// In most cases a (layer, ttype, index) tuple is provided and a list of TriggerGuide:ResultGuide
/// mappings
/// is provided. See lookup_guides().
pub struct LayerLookup<'a, const SIZE: usize> {
    layer_lookup: FnvIndexMap<(u8, u8, u16), usize, SIZE>,
    raw_layer_lookup: &'a [u8],
    trigger_guides: &'a [u8],
    result_guides: &'a [u8],
    trigger_result_mapping: &'a [u16],
}

impl<'a, const SIZE: usize> LayerLookup<'a, SIZE> {
    pub fn new(
        raw_layer_lookup: &'a [u8],
        trigger_guides: &'a [u8],
        result_guides: &'a [u8],
        trigger_result_mapping: &'a [u16],
    ) -> Self {
        // Build layer lookup from array
        // The purpose of this hash table is to quickly find the trigger list in LAYER_LOOKUP
        // Mapping
        //   (<layer>, <ttype>, <index>) -> LAYER_LOOKUP index
        let mut layer_lookup = FnvIndexMap::<(u8, u8, u16), usize, SIZE>::new();

        let mut mode = LayerProcessMode::Layer;
        let mut layer = 0;
        let mut ttype = 0;
        let mut index: u16 = 0;
        for (i, val) in raw_layer_lookup.iter().enumerate() {
            match mode {
                LayerProcessMode::Layer => {
                    layer = *val;
                    mode = LayerProcessMode::TriggerType;
                }
                LayerProcessMode::TriggerType => {
                    ttype = *val;
                    mode = LayerProcessMode::IndexA;
                }
                LayerProcessMode::IndexA => {
                    index = *val as u16;
                    mode = LayerProcessMode::IndexB;
                }
                LayerProcessMode::IndexB => {
                    index |= (*val as u16) << 8;
                    mode = LayerProcessMode::TriggerSize;
                }
                LayerProcessMode::TriggerSize => {
                    let size = *val;
                    let lookup = i;
                    // We only add to the hash table if triggers actually exist
                    // The KLL compiler should optimize these out, but it's still valid array syntax
                    mode = if size > 0 {
                        // Attempt to insert the key
                        match layer_lookup.insert((layer, ttype, index), lookup) {
                            // Success, no existing key
                            Ok(None) => {}
                            // Success, replace existing key (this is bad, warn)
                            Ok(Some(old_lookup)) => {
                                warn!("Duplicate layer lookup key! ({}, {}). {} has been replaced by {}", layer, index, old_lookup, lookup);
                            }
                            Err(e) => {
                                error!(
                                    "Failed to add lookup key ({}, {}) -> {}: {:?}",
                                    layer, index, lookup, e
                                );
                            }
                        }
                        // Triggers are u16, so multiple by 2
                        LayerProcessMode::Triggers(size * 2)
                    } else {
                        LayerProcessMode::Layer
                    }
                }
                LayerProcessMode::Triggers(size) => {
                    mode = if size <= 1 {
                        LayerProcessMode::Layer
                    } else {
                        LayerProcessMode::Triggers(size - 1)
                    };
                }
            }
        }
        Self {
            layer_lookup,
            raw_layer_lookup,
            trigger_guides,
            result_guides,
            trigger_result_mapping,
        }
    }

    /// Retrieves a TriggerList
    /// A TriggerList is a list of indices that correspond to a specific TriggerGuide -> ResultGuide
    /// mapping.
    pub fn trigger_list(&self, (layer, ttype, index): (u8, u8, u16)) -> Option<&'a [u8]> {
        match self.layer_lookup.get(&(layer, ttype, index)) {
            Some(lookup) => {
                // Determine size of trigger list
                let size: usize = self.raw_layer_lookup[*lookup].into();

                // If the size is 0, just return None
                if size == 0 {
                    return None;
                }

                // Each trigger list id is a u16
                let size = size * 2;

                // Build TriggerList slice
                let initial: usize = lookup + 1;
                Some(&self.raw_layer_lookup[initial..initial + size])
            }
            None => None,
        }
    }

    /// Retrieves a list of TriggerGuide:ResultGuide mappings
    /// Will need to be called for every new TriggerEvent.
    pub fn lookup_guides<const LSIZE: usize>(
        &self,
        (layer, ttype, index): (u8, u8, u16),
    ) -> heapless::Vec<(u16, u16), LSIZE> {
        let mut guides = heapless::Vec::<_, LSIZE>::new();

        // Lookup TriggerList
        match self.trigger_list((layer, ttype, index)) {
            Some(mlookup) => {
                // Iterate over each trigger to locate guides
                // Each value is a u16 (hence chunking by 2)
                for chunk in mlookup.chunks_exact(2) {
                    // Determine guide lookup index
                    let index = u16::from_le_bytes([chunk[0], chunk[1]]) as usize;

                    // Push guide pair
                    assert!(
                        guides
                            .push((
                                self.trigger_result_mapping[index],
                                self.trigger_result_mapping[index + 1]
                            ))
                            .is_ok(),
                        "lookup_guides vector is full, increase LSIZE: {}",
                        LSIZE
                    );
                }
                guides
            }
            None => guides,
        }
    }

    /// Retrieves the TriggerGuide for a given TriggerGuide:ResultGuide pair
    ///
    /// offset indicates the number of u8 positions the sequence is currently at.
    /// trigger + offset will always point to the start of a combination
    pub fn trigger_guide(
        &self,
        (trigger, _result): (u16, u16),
        offset: u16,
    ) -> Option<&[TriggerCondition]> {
        // Determine size of offset combo in the sequence
        let count = self.trigger_guides[trigger as usize + offset as usize] as usize;
        if count == 0 {
            return None;
        }

        // Determine starting position of combo
        let start = trigger as usize + offset as usize + 1;

        // Convert u8 combo list to TriggerCondition list
        let ptr: *const u8 =
            self.trigger_guides[start..start + core::mem::size_of::<TriggerCondition>()].as_ptr();
        let cond = unsafe { core::slice::from_raw_parts(ptr as *const TriggerCondition, count) };
        Some(cond)
    }

    /// Retrieves the ResultGuide for a given TriggerGuide:ResultGuide pair
    ///
    /// offset indicates the number of u8 positions the sequence is currently at.
    /// result + offset will always point to the start of a combination
    pub fn result_guide(
        &self,
        (_trigger, result): (u16, u16),
        offset: u8,
    ) -> Option<&[Capability]> {
        // Determine size of offset combo in the sequence
        let count = self.result_guides[result as usize + offset as usize] as usize;
        if count == 0 {
            return None;
        }

        // Determine starting position of combo
        let start = result as usize + offset as usize + 1;

        // Convert u8 combo list to Capability list
        let ptr: *const u8 =
            self.result_guides[start..start + core::mem::size_of::<Capability>()].as_ptr();
        let cond = unsafe { core::slice::from_raw_parts(ptr as *const Capability, count) };
        Some(cond)
    }

    /// Convience access for layer_lookup
    /// Useful when trying to get a list of all possible triggers
    pub fn layer_lookup(&self) -> &FnvIndexMap<(u8, u8, u16), usize, SIZE> {
        &self.layer_lookup
    }
}
