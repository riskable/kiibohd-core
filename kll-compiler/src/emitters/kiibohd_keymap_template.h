/* Copyright (C) 2014-2019 by Jacob Alexander
 *
 * This file is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This file is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this file.  If not, see <http://www.gnu.org/licenses/>.
 */

{information}


#pragma once

// ----- Includes -----

// KLL Include
#include <kll.h>



// ----- Capabilities -----

// Capability function declarations
{capabilities_func_decl}


// Indexed Capabilities Table
const Capability CapabilitiesList[] = {{
{capabilities_list}
}};


// -- Result Macros

// Result Macro Guides
{result_macros}


// -- Result Macro List

// Indexed Table of Result Macros
const ResultMacro ResultMacroList[] = {{
{result_macro_list}
}};


// -- Trigger Macros

// Trigger Macro Guides
{trigger_macros}


// -- Trigger Macro List

// Indexed Table of Trigger Macros
const TriggerMacro TriggerMacroList[] = {{
{trigger_macro_list}
}};


// -- Trigger Macro Record List

// Keeps a record/state of each trigger macro
TriggerMacroRecord TriggerMacroRecordList[ TriggerMacroNum ];



// ----- Trigger Maps -----

// MaxScanCode
// - This is retrieved from the KLL configuration
// - Should be corollated with the max scan code in the scan module
// - Maximum value is 0x100 (0x0 to 0xFF)
// - Increasing it beyond the keyboard's capabilities is just a waste of ram...
#define MaxScanCode {max_scan_code:#04X}

// -- Trigger Lists
//
// Index 0: # of triggers in list
// Index n: pointer to trigger macro - use tm() macro

// - Default Layer -
const nat_ptr_t *default_scanMap[] = {{
{default_layer_trigger_list}
}};


// - Partial Layers -
const nat_ptr_t *layer_scanMap[] = {{
{partial_layer_trigger_lists}
}};


// -- ScanCode Offset Map
// Maps interconnect ids to scancode offsets
//
// Only used for keyboards with an interconnect
const uint8_t InterconnectOffsetList[] = {{
{scancode_interconnect_offset_list}
}};


// -- ScanCode Indexed Maps
// Maps to a trigger list of macro pointers
//                 _
// <scan code> -> |T|
//                |r| -> <trigger macro pointer 1>
//                |i|
//                |g| -> <trigger macro pointer 2>
//                |g|
//                |e| -> <trigger macro pointer 3>
//                |r|
//                |s| -> <trigger macro pointer n>
//                 -

// - Default Map for ScanCode Lookup -
const nat_ptr_t *default_scanMap[] = {{
{default_layer_scanmap}
}};


// - Partial Layer ScanCode Lookup Maps -
const nat_ptr_t *layer1_scanMap[] = {{
{partial_layer_scanmaps}
}};



// ----- Layer Index -----

// -- Layer Index List
//
// Index 0: Default map
// Index n: Additional layers
const Layer LayerIndex[] = {{
{layer_index_list}
}};


// - Layer State
LayerStateType LayerState[ LayerNum ];



// ----- Rotation Parameters -----

// Each position represents the maximum rotation value for the index
const uint8_t Rotation_MaxParameter[] = {{;
{rotation_parameters}
}};



// ----- Key Positions -----

// -- Physical Key Positions
//
// Index 0: Key 1
// Each key has 6 dimensions
// x,y,z and rx,ry,rz (rotation)
// Units are in mm
const Position Key_Positions[] = {{
{key_positions}
}};



// ----- UTF-8 -----

// UTF-8 strings are stored in a single lookup array
// Each unicode string is NULL terminated
// A 16-bit integer is used to lookup each of the UTF-8 strings
// This storage is also used for single characters instead of using a 32-bit integer to represent
// any possible UTF-8 character.
const char* UTF8_Strings[] = {{
{utf8_data}
}};
