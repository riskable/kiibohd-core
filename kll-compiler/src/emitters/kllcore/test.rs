// Copyright 2021-2022 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![cfg(test)]

use crate::emitters::kllcore::KllCoreData;
use crate::types::KllFile;
use layouts_rs::Layouts;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[test]
fn trigger() {
    let test = fs::read_to_string("examples/kllcoretest.kll").unwrap();
    let result = KllFile::from_str(&test);
    let state = result.unwrap().into_struct();

    // Generate trigger guides
    let mut trigger_guides = Vec::new();
    for trigger_list in state.trigger_lists() {
        let mut guide = trigger_list.kll_core_guide();
        trigger_guides.append(&mut guide);
    }

    // TODO Validate
}

#[test]
fn result() {
    let test = fs::read_to_string("examples/kllcoretest.kll").unwrap();
    let result = KllFile::from_str(&test);
    let state = result.unwrap().into_struct();
    let layouts = Layouts::from_dir(PathBuf::from("layouts"));

    // Generate result guides
    let mut result_guides = Vec::new();
    for result_list in state.result_lists() {
        let mut guide = result_list.kll_core_guide(layouts.clone());
        result_guides.append(&mut guide);
    }

    // TODO Validate
}

#[test]
fn trigger_result() {
    let test = fs::read_to_string("examples/kllcoretest.kll").unwrap();
    let result = KllFile::from_str(&test);
    let state = result.unwrap().into_struct();
    let layouts = Layouts::from_dir(PathBuf::from("layouts"));

    // Trigger and Result deduplication hashmaps
    let mut trigger_hash = HashMap::new();
    let mut result_hash = HashMap::new();

    // Trigger:Result mapping hashmap
    let mut trigger_result_hash = HashMap::new();

    // Generate trigger and result guides as well as the trigger result mapping
    let mut trigger_guides = Vec::new();
    let mut result_guides = Vec::new();
    let mut trigger_result_map: Vec<u16> = Vec::new();
    for (trigger_list, result_list) in state.trigger_result_lists() {
        let mut trigger_guide = trigger_list.kll_core_guide();
        // Determine if trigger guide has already been added
        let trigger_pos = match trigger_hash.try_insert(trigger_guide.clone(), trigger_guide.len())
        {
            Ok(pos) => {
                trigger_guides.append(&mut trigger_guide);
                *pos
            }
            Err(err) => err.entry.get().clone(),
        };

        let mut result_guide = result_list.kll_core_guide(layouts.clone());
        // Determine if result guide has already been added
        let result_pos = match result_hash.try_insert(result_guide.clone(), result_guide.len()) {
            Ok(pos) => {
                result_guides.append(&mut result_guide);
                *pos
            }
            Err(err) => err.entry.get().clone(),
        };

        // Add trigger:result mapping
        if trigger_result_hash
            .insert((trigger_guide, result_guide), (trigger_pos, result_pos))
            .is_none()
        {
            trigger_result_map.push(trigger_pos as u16);
            trigger_result_map.push(result_pos as u16);
        }
    }

    // TODO Validate
}

#[test]
fn layer_lookup_simple() {
    let test = fs::read_to_string("examples/kllcoretest.kll").unwrap();
    let result = KllFile::from_str(&test);
    let state = result.unwrap().into_struct();
    let mut layers = vec![state];
    println!("THIS: {:?}", layers);
    let layouts = Layouts::from_dir(PathBuf::from("layouts"));
    let _kdata = KllCoreData::new(&mut layers, layouts);

    // TODO Validate
    // Load data structures into kll-core
    // Pipe valid input commands
    // Verify command outputs
}

#[test]
fn generate_binary() {
    // todo needs an offset table for the firmware to know where the pointers
    // are
}

#[test]
fn generate_rust() {
    // todo
}

#[test]
fn keystone_basemap_rust() {
    let test = fs::read_to_string("examples/keystone_scancode_map.kll").unwrap();
    let result = KllFile::from_str(&test);
    let state = result.unwrap().into_struct();
    let mut layers = vec![state];
    let layouts = Layouts::from_dir(PathBuf::from("layouts"));
    let kdata = KllCoreData::new(&mut layers, layouts);
    dbg!(&kdata.trigger_guides);
    //dbg!(kdata.trigger_hash);
    //dbg!(kdata.trigger_result_hash);

    // Parse trigger_guides to use as all possible kll inputs
    let lookup = kll_core::layout::LayerLookup::<256>::new(
        &kdata.raw_layer_lookup,
        &kdata.trigger_guides,
        &kdata.result_guides,
        &kdata.trigger_result_map,
    );

    for kset in lookup.layer_lookup().keys() {
        dbg!(lookup.trigger_list(*kset));
        dbg!(lookup.lookup_guides::<10>(*kset));
    }
}
