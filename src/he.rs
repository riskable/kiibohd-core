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

use heapless::consts::{U10, U115, U2, U4096};
use kiibohd_hall_effect::{CalibrationStatus, SenseAnalysis, SenseData, SensorError, Sensors};

// ----- Types -----

type NumScanCodes = U115;
type MaxAdc = U4096;
type MagnetBounds = U10; // TODO This needs to be calculated
type SenseAccumulation = U2;

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
    ErrorNotInitialized,
    ErrorSensorDetectedNoMagnet,
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
#[no_mangle]
pub extern "C" fn he_scan_event(index: u16, val: u16, analysis: *mut SenseAnalysis) -> HeStatus {
    unsafe {
        // Retrieve interface
        let intf = match INTF.as_mut() {
            Some(intf) => intf,
            None => {
                return HeStatus::ErrorNotInitialized;
            }
        };

        match intf.add::<SenseAccumulation, MaxAdc, MagnetBounds>(index as usize, val) {
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
                    CalibrationStatus::SensorDetected => HeStatus::ErrorSensorDetectedNoMagnet,
                    CalibrationStatus::SensorMissing => HeStatus::ErrorSensorMissing,
                    CalibrationStatus::InvalidReading => HeStatus::ErrorInvalidReading,
                    _ => HeStatus::ErrorUnknown,
                },
                SensorError::InvalidSensor(_) => HeStatus::ErrorInvalidIndex,
                _ => HeStatus::ErrorUnknown,
            },
        }
    }
}

/// Retrieve calibration data
#[no_mangle]
pub extern "C" fn he_calibration(index: u16, data: *mut SenseData) -> HeStatus {
    unsafe {
        // Retrieve interface
        let intf = match INTF.as_mut() {
            Some(intf) => intf,
            None => {
                return HeStatus::ErrorNotInitialized;
            }
        };

        match intf.get_data(index as usize) {
            Ok(results) => {
                *data = results.clone();
                HeStatus::Success
            }
            Err(_) => HeStatus::ErrorInvalidIndex,
        }
    }
}
