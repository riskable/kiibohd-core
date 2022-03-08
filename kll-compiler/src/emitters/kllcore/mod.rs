// Copyright 2021-2022 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::types::{Key, TriggerType};
use crate::{KllGroups, KllState};
use layouts_rs::Layouts;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod test;

/// Key: (trigger_guide, result_guide)
/// Value: (trigger_pos, result_pos, trigger_result_map pos)
type TriggerResultHash = HashMap<(Vec<u8>, Vec<u8>), (usize, usize, usize)>;

#[allow(dead_code)]
#[derive(Debug)]
pub struct KllCoreData<'a> {
    layers: Vec<KllState<'a>>,
    pub trigger_hash: HashMap<Vec<u8>, usize>,
    pub result_hash: HashMap<Vec<u8>, usize>,
    pub trigger_result_hash: TriggerResultHash,
    pub layer_lookup_hash: HashMap<(u8, u8, u16), Vec<u16>>,
    pub trigger_guides: Vec<u8>,
    pub result_guides: Vec<u8>,
    pub trigger_result_map: Vec<u16>,
    pub raw_layer_lookup: Vec<u8>,
}

impl<'a> KllCoreData<'a> {
    /// Given KllState layers, generate datastructures for kll-core
    pub fn new(layers: &mut [KllState<'a>], layouts: Layouts) -> Self {
        // Trigger and Result deduplication hashmaps
        let mut trigger_hash = HashMap::new();
        let mut result_hash = HashMap::new();

        // Trigger:Result mapping hashmap
        let mut trigger_result_hash = HashMap::new();

        // Layer lookup hashmap
        let mut layer_lookup_hash: HashMap<(u8, u8, u16), Vec<u16>> = HashMap::new();

        // Generate trigger and result guides as well as the trigger result mapping
        let mut trigger_guides = Vec::new();
        let mut result_guides = Vec::new();
        let mut trigger_result_map: Vec<u16> = Vec::new();
        let mut raw_layer_lookup: Vec<u8> = Vec::new();

        for (layer_index, layer) in layers.iter_mut().enumerate() {
            // Generate explicit state in layer
            layer.generate_state_scheduling();

            for (trigger_list, result_list) in layer.trigger_result_lists() {
                let trigger_guide = trigger_list.kll_core_guide();
                // Determine if trigger guide has already been added
                let trigger_pos =
                    match trigger_hash.try_insert(trigger_guide.clone(), trigger_guide.len()) {
                        Ok(pos) => {
                            trigger_guides.append(&mut trigger_guide.clone());
                            *pos
                        }
                        Err(err) => *err.entry.get(),
                    };

                let result_guide = result_list.kll_core_guide(layouts.clone());
                // Determine if result guide has already been added
                let result_pos =
                    match result_hash.try_insert(result_guide.clone(), result_guide.len()) {
                        Ok(pos) => {
                            result_guides.append(&mut result_guide.clone());
                            *pos
                        }
                        Err(err) => *err.entry.get(),
                    };

                // Add trigger:result mapping
                // Maps to the trigger guide index position, result guide index position
                // and the trigger_result_map index position (needed for the layer lookup)
                if trigger_result_hash
                    .try_insert(
                        (trigger_guide.clone(), result_guide),
                        (trigger_pos, result_pos, trigger_result_map.len()),
                    )
                    .is_ok()
                {
                    trigger_result_map.push(trigger_pos as u16);
                    trigger_result_map.push(result_pos as u16);
                }
            }

            // Iterate again to build the necessary layer lookup
            for (trigger_list, result_list) in layer.trigger_result_lists() {
                let trigger_guide = trigger_list.kll_core_guide();
                let result_guide = result_list.kll_core_guide(layouts.clone());

                // Lookup position in trigger:result lookup
                let (_, _, trigger_result_pos) =
                    trigger_result_hash[&(trigger_guide, result_guide)];

                for trigger in trigger_list.iter() {
                    // Determine type and index
                    // TODO - Determine Type (Switch type vs Analog)
                    let (index_type, index) = match &trigger.trigger {
                        TriggerType::Key(key) => {
                            // TODO Determine type
                            let index_type: u8 = 1;
                            let index: u16 = match key {
                                Key::Scancode(index) => *index as u16,
                                _ => {
                                    panic!("{} Not implemented yet", key);
                                }
                            };
                            (index_type, index)
                        }
                        _ => {
                            panic!("{} Not implemented yet", trigger.trigger);
                        }
                    };
                    layer_lookup_hash
                        .entry((layer_index as u8, index_type, index))
                        .and_modify(|e| e.push(trigger_result_pos as u16))
                        .or_insert(Vec::new())
                        .push(trigger_result_pos as u16);
                }
            }
        }

        // After generating the layer lookup hash generate the binary form
        for ((layer, index_type, index), triggers) in &layer_lookup_hash {
            raw_layer_lookup.push(*layer);
            raw_layer_lookup.push(*index_type);
            raw_layer_lookup.append(&mut Vec::from(index.to_le_bytes()));
            raw_layer_lookup.push(triggers.len().try_into().unwrap());
            for trigger in triggers {
                raw_layer_lookup.append(&mut Vec::from(trigger.to_le_bytes()));
            }
        }

        Self {
            layers: layers.to_vec(),
            trigger_hash,
            result_hash,
            trigger_result_hash,
            layer_lookup_hash,
            trigger_guides,
            result_guides,
            trigger_result_map,
            raw_layer_lookup,
        }
    }

    /// Generate rust form of kll-core datastructures
    pub fn rust(&self, filepath: &Path) -> std::io::Result<()> {
        let mut file = File::create(filepath)?;

        let mut trigger_guides = String::new();
        for elem in &self.trigger_guides {
            trigger_guides += &format!("{}, ", elem).to_string();
        }
        let mut result_guides = String::new();
        for elem in &self.result_guides {
            result_guides += &format!("{}, ", elem).to_string();
        }
        let mut trigger_result_mapping = String::new();
        for elem in &self.trigger_result_map {
            trigger_result_mapping += &format!("{}, ", elem).to_string();
        }
        let mut raw_layer_lookup = String::new();
        for elem in &self.raw_layer_lookup {
            raw_layer_lookup += &format!("{}, ", elem).to_string();
        }

        file.write_all(
            &format!(
                "
//
// NOTE: This is is a generated file (from kll-compiler), do not modify!
//

/// Trigger Guides
/// Traces sequences of scancodes
pub const TRIGGER_GUIDES: &'static [u8] = &[{}];

/// Result Guides
/// Traces sequences of capabilities
pub const RESULT_GUIDES: &'static [u8] = &[{}];

/// Trigger:Result Mapping
pub const TRIGGER_RESULT_MAPPING: &'static [u8] = &[{}];

/// Raw Layer Lookup Table
pub const LAYER_LOOKUP: &'static [u8] = &[{}];
",
                trigger_guides, result_guides, trigger_result_mapping, raw_layer_lookup
            )
            .into_bytes(),
        )?;
        Ok(())
    }

    /*
    /// Generate binary form of kll-core datastructures
    pub fn binary(&self, filepath: &Path) -> std::io::Result<()> {
        let mut file = File::create(filepath)?;

        // TODO
        Ok(())
    }
    */
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error {
    ParsingError,
    UnknownError,
}

pub fn verify(_groups: &KllGroups) -> Result<(), Error> {
    Ok(())
}

pub fn write(file: &Path, groups: &KllGroups, layouts: Layouts) {
    // TODO Merge layouts correctly
    let mut layers = groups.base.clone();
    //let layers = &groups.default;

    // Generate kll-core datastructures
    let kdata = KllCoreData::new(&mut layers, layouts);

    // Write rust file
    kdata.rust(file).unwrap();
}
