/* Copyright (C) 2021 by Jacob Alexander */

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
/// TODO
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
