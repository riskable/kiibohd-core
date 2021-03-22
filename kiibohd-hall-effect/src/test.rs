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

#![cfg(test)]

// ----- Crates -----

use super::*;
use flexi_logger::Logger;
use heapless::consts::{U1, U1000, U2};
use typenum::{UInt, UTerm, B0, B1};

// ----- Types -----

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

// ----- Enumerations -----

enum LogError {
    CouldNotStartLogger,
}

// ----- Functions -----

/// Lite logging setup
fn setup_logging_lite() -> Result<(), LogError> {
    match Logger::with_env_or_str("")
        .format(flexi_logger::colored_default_format)
        .format_for_files(flexi_logger::colored_detailed_format)
        .duplicate_to_stderr(flexi_logger::Duplicate::All)
        .start()
    {
        Err(_) => Err(LogError::CouldNotStartLogger),
        Ok(_) => Ok(()),
    }
}

// ----- Tests -----

#[test]
fn invalid_index() {
    setup_logging_lite().ok();

    // Allocate a single sensor
    let mut sensors = Sensors::<U1>::new().unwrap();

    // Add data to an invalid location
    assert!(sensors
        .add_test::<U1, MinOkThreshold, MaxOkThreshold, NoSensorThreshold>(1, 0)
        .is_err());

    // Retrieve data from an invalid location
    assert!(sensors.get_data(1).is_err());
}

#[test]
fn not_ready() {
    setup_logging_lite().ok();

    // Allocate a single sensor
    let sensors = Sensors::<U1>::new().unwrap();

    // Retrieve before sending any data
    let state = sensors.get_data(0);
    match state.clone() {
        Err(SensorError::CalibrationError(data)) => match data.cal {
            CalibrationStatus::NotReady => {
                return;
            }
            _ => {}
        },
        _ => {}
    }
    assert!(false, "Unexpected state: {:?}", state);
}

#[test]
fn sensor_missing() {
    setup_logging_lite().ok();

    // Allocate a single sensor
    let mut sensors = Sensors::<U1>::new().unwrap();

    // Add a sensor value of 0
    // (needs 2 samples to finish averaging)
    // Once averaging is complete, we'll get a result
    assert!(sensors
        .add_test::<U2, MinOkThreshold, MaxOkThreshold, NoSensorThreshold>(
            0,
            <NoSensorThreshold>::to_u16() - 1
        )
        .is_ok());
    let state = sensors.add_test::<U2, MinOkThreshold, MaxOkThreshold, NoSensorThreshold>(
        0,
        <NoSensorThreshold>::to_u16() - 1,
    );

    match state.clone() {
        Err(SensorError::CalibrationError(data)) => match data.cal {
            CalibrationStatus::SensorMissing => {
                return;
            }
            _ => {}
        },
        _ => {}
    }
    assert!(false, "Unexpected state: {:?}", state);
}

#[test]
fn sensor_broken() {
    setup_logging_lite().ok();

    // Allocate a single sensor
    let mut sensors = Sensors::<U1>::new().unwrap();

    // Add max sensor value
    // (needs 2 samples to finish averaging)
    // Once averaging is complete, we'll get a result
    assert!(sensors
        .add_test::<U2, MinOkThreshold, MaxOkThreshold, NoSensorThreshold>(0, 0xFFFF,)
        .is_ok());
    let state =
        sensors.add_test::<U2, MinOkThreshold, MaxOkThreshold, NoSensorThreshold>(0, 0xFFFF);

    match state.clone() {
        Err(SensorError::CalibrationError(data)) => match data.cal {
            CalibrationStatus::SensorBroken => {
                return;
            }
            _ => {}
        },
        _ => {}
    }
    assert!(false, "Unexpected state: {:?}", state);
}

#[test]
fn magnet_missing() {
    setup_logging_lite().ok();

    // Allocate a single sensor
    let mut sensors = Sensors::<U1>::new().unwrap();

    // Add max sensor value
    // (needs 2 samples to finish averaging)
    // Once averaging is complete, we'll get a result
    assert!(sensors
        .add_test::<U2, MinOkThreshold, MaxOkThreshold, NoSensorThreshold>(
            0,
            <MinOkThreshold>::to_u16() - 1
        )
        .is_ok());
    let state = sensors.add_test::<U2, MinOkThreshold, MaxOkThreshold, NoSensorThreshold>(
        0,
        <MinOkThreshold>::to_u16() - 1,
    );

    match state.clone() {
        Err(SensorError::CalibrationError(data)) => match data.cal {
            CalibrationStatus::MagnetWrongPoleOrMissing => {
                return;
            }
            _ => {}
        },
        _ => {}
    }
    assert!(false, "Unexpected state: {:?}", state);
}

fn magnet_check_calibration<U: ArrayLength<SenseData>>(sensors: &mut Sensors<U>) {
    // Add two values, larger MinOkThreshold
    let val = <MinOkThreshold>::to_u16() + 2;
    // (needs 2 samples to finish averaging)
    // Once averaging is complete, we'll get a result
    assert!(sensors
        .add_test::<U2, MinOkThreshold, MaxOkThreshold, NoSensorThreshold>(0, val)
        .is_ok());
    let state = sensors.add_test::<U2, MinOkThreshold, MaxOkThreshold, NoSensorThreshold>(0, val);

    let mut test = false;
    match state.clone() {
        Ok(rval) => {
            if let Some(rval) = rval {
                if rval.raw == val {
                    test = true;
                }
            }
        }
        _ => {}
    }
    assert!(test, "Unexpected state: {:?}", state);

    // Check calibration
    let mut test = false;
    let state = sensors.get_data(0);
    match state {
        Ok(val) => {
            if val.cal == CalibrationStatus::MagnetDetected {
                test = true;
            }
        }
        _ => {}
    }
    assert!(test, "Unexpected state: {:?}", state);
}

fn magnet_check_normal<U: ArrayLength<SenseData>>(sensors: &mut Sensors<U>) {
    // Add two values, larger MinOkThreshold
    let val = <MinOkThreshold>::to_u16() + 2;
    // (needs 2 samples to finish averaging)
    // Once averaging is complete, we'll get a result
    assert!(sensors
        .add_test::<U2, MinOkThreshold, MaxOkThreshold, NoSensorThreshold>(0, val)
        .is_ok());
    let state = sensors.add_test::<U2, MinOkThreshold, MaxOkThreshold, NoSensorThreshold>(0, val);

    let mut test = false;
    match state.clone() {
        Ok(rval) => {
            if let Some(rval) = rval {
                if rval.raw == val {
                    test = true;
                }
            }
        }
        _ => {}
    }
    assert!(test, "Unexpected state: {:?}", state);

    // Check calibration
    let mut test = false;
    let state = sensors.get_data(0);
    match state {
        Ok(val) => {
            if val.cal == CalibrationStatus::MagnetDetected {
                test = true;
            }
        }
        _ => {}
    }
    assert!(test, "Unexpected state: {:?}", state);
}

fn magnet_calibrate<U: ArrayLength<SenseData>>(sensors: &mut Sensors<U>) {
    // Calibrate sensor
    magnet_check_calibration::<U>(sensors);

    // Check again with normal operation
    magnet_check_normal::<U>(sensors);
}

#[test]
fn magnet_detected() {
    setup_logging_lite().ok();

    // Allocate a single sensor
    let mut sensors = Sensors::<U1>::new().unwrap();

    // Two sets of samples that will put the sensor into normal mode (and check both MagnetDetected
    // states)
    magnet_calibrate::<U1>(&mut sensors);
}

#[test]
fn sensor_min_adjust() {
    setup_logging_lite().ok();

    // Allocate a single sensor
    let mut sensors = Sensors::<U1>::new().unwrap();

    // Baseline
    magnet_check_calibration::<U1>(&mut sensors);
    magnet_check_normal::<U1>(&mut sensors);

    // Send a lower value than the min calibration and make sure it was set
    let old_min = sensors.get_data(0).unwrap().stats.min;
    let val = old_min - 1;

    assert!(sensors
        .add_test::<U2, MinOkThreshold, MaxOkThreshold, NoSensorThreshold>(0, val)
        .is_ok());
    let state = sensors.add_test::<U2, MinOkThreshold, MaxOkThreshold, NoSensorThreshold>(0, val);
    let mut test = false;
    match state.clone() {
        Ok(rval) => {
            if let Some(rval) = rval {
                if rval.raw == val {
                    test = true;
                }
            }
        }
        _ => {}
    }
    assert!(test, "Unexpected state: {:?}", state);

    // Check min calibration
    let new_min = sensors.get_data(0).unwrap().stats.min;
    assert!(val == new_min);
}
