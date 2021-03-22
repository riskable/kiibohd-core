/* Copyright (C) 2021 by Jacob Alexander
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */

// ----- Modules -----

// ----- Crates -----

use heapless::consts::{U1000, U115, U2};
use typenum::{UInt, UTerm, B0, B1};

pub use kiibohd_hall_effect::{
    CalibrationStatus, SenseAnalysis, SenseData, SenseStats, SensorError, Sensors,
};

// ----- Types -----

type NumScanCodes = U115;
type SenseAccumulation = U2;

// --- NOTE ---
// These thresholds were calculated on a Keystone v1.00 TKL pcb

// Calibration Mode Thresholds
type MinOkThreshold = UInt<
    UInt<
        UInt<
            UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>,
            B1,
        >,
        B1,
    >,
    B0,
>; // U1350 - b10101000110 - Switch not pressed (not 100% guaranteed, but the minimum range we can work withA
   // Some sensors will have default values up to 1470 without any magnet and that is within the specs
   // of the datasheet.
type MaxOkThreshold = UInt<
    UInt<
        UInt<
            UInt<
                UInt<
                    UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>,
                    B0,
                >,
                B0,
            >,
            B1,
        >,
        B0,
    >,
    B0,
>; // U2500 - b100111000100 - Switch fully pressed
type NoSensorThreshold = U1000; // Likely invalid ADC level from non-existent sensor (or very low magnet)

// ----- Globals -----

static mut INTF: Option<Sensors<NumScanCodes>> = None;

// ----- External C Callbacks -----

extern "C" {}

// ----- External C Interface -----

#[repr(C)]
#[derive(PartialEq)]
pub enum HeStatus {
    Success,
    AnalysisReady,
    ErrorInvalidIndex,
    ErrorInvalidReading,
    ErrorMagnetWrongPoleOrMissing,
    ErrorNotInitialized,
    ErrorSensorBroken,
    ErrorSensorMissing,
    ErrorSensorNotReady,
    ErrorUnknown,
}

/// Initialize the hall effect interface
/// Must be called first, before any other he_ commands.
#[no_mangle]
pub extern "C" fn he_init() -> HeStatus {
    unsafe {
        INTF = Some(match Sensors::<NumScanCodes>::new() {
            Ok(intf) => intf,
            Err(_) => {
                return HeStatus::ErrorUnknown;
            }
        });
    }
    HeStatus::Success
}

/// Processes an ADC event for the given index (scan code location)
/// This should be the raw value from the ADC
/// Once enough values have been accumulated, data analysis is done
/// automatically
/// Uses normal mode
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn he_scan_event(
    index: u16,
    val: u16,
    analysis: *mut SenseAnalysis,
) -> HeStatus {
    // Retrieve interface
    let intf = match INTF.as_mut() {
        Some(intf) => intf,
        None => {
            return HeStatus::ErrorNotInitialized;
        }
    };

    match intf.add::<SenseAccumulation>(index as usize, val) {
        Ok(data) => {
            if let Some(data) = data {
                *analysis = data.clone();
                HeStatus::AnalysisReady
            } else {
                HeStatus::Success
            }
        }
        Err(err) => match err {
            SensorError::CalibrationError(data) => match data.cal {
                CalibrationStatus::NotReady => HeStatus::ErrorSensorNotReady,
                _ => HeStatus::ErrorUnknown,
            },
            SensorError::InvalidSensor(_) => HeStatus::ErrorInvalidIndex,
            _ => HeStatus::ErrorUnknown,
        },
    }
}

/// Processes an ADC event for the given index (scan code location)
/// This should be the raw value from the ADC
/// Once enough values have been accumulated, data analysis is done
/// automatically
/// Uses test mode
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn he_test_event(
    index: u16,
    val: u16,
    analysis: *mut SenseAnalysis,
) -> HeStatus {
    // Retrieve interface
    let intf = match INTF.as_mut() {
        Some(intf) => intf,
        None => {
            return HeStatus::ErrorNotInitialized;
        }
    };

    match intf.add_test::<SenseAccumulation, MinOkThreshold, MaxOkThreshold, NoSensorThreshold>(
        index as usize,
        val,
    ) {
        Ok(data) => {
            if let Some(data) = data {
                *analysis = data.clone();
                HeStatus::AnalysisReady
            } else {
                HeStatus::Success
            }
        }
        Err(err) => match err {
            SensorError::CalibrationError(data) => match data.cal {
                CalibrationStatus::MagnetWrongPoleOrMissing => {
                    HeStatus::ErrorMagnetWrongPoleOrMissing
                }
                CalibrationStatus::NotReady => HeStatus::ErrorSensorNotReady,
                CalibrationStatus::SensorBroken => HeStatus::ErrorSensorBroken,
                CalibrationStatus::SensorMissing => HeStatus::ErrorSensorMissing,
                _ => HeStatus::ErrorUnknown,
            },
            SensorError::InvalidSensor(_) => HeStatus::ErrorInvalidIndex,
            _ => HeStatus::ErrorUnknown,
        },
    }
}
/// Retrieve calibration status
#[no_mangle]
pub extern "C" fn he_calibration(index: u16) -> CalibrationStatus {
    unsafe {
        // Retrieve interface
        let intf = match INTF.as_mut() {
            Some(intf) => intf,
            None => {
                return CalibrationStatus::NotReady;
            }
        };

        match intf.get_data(index as usize) {
            Ok(results) => results.cal.clone(),
            Err(_) => CalibrationStatus::InvalidIndex,
        }
    }
}

/// Retrieve analysis data
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn he_analysis(index: u16, analysis: *mut SenseAnalysis) -> HeStatus {
    // Retrieve interface
    let intf = match INTF.as_mut() {
        Some(intf) => intf,
        None => {
            return HeStatus::ErrorNotInitialized;
        }
    };

    match intf.get_data(index as usize) {
        Ok(results) => {
            *analysis = results.analysis.clone();
            HeStatus::Success
        }
        Err(_) => HeStatus::ErrorInvalidIndex,
    }
}

/// Retrieve stats data
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn he_stats(index: u16, stats: *mut SenseStats) -> HeStatus {
    // Retrieve interface
    let intf = match INTF.as_mut() {
        Some(intf) => intf,
        None => {
            return HeStatus::ErrorNotInitialized;
        }
    };

    match intf.get_data(index as usize) {
        Ok(results) => {
            *stats = results.stats.clone();
            HeStatus::Success
        }
        Err(_) => HeStatus::ErrorInvalidIndex,
    }
}
