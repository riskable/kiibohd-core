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
use heapless::consts::{U1, U10, U2, U4096};

// ----- Types -----

type MaxAdc = U4096;
type MagnetBounds = U10; // TODO This needs to be calculated

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
    assert!(sensors.add::<U1, MaxAdc, MagnetBounds>(1, 0).is_err());

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

fn no_magnet<U: ArrayLength<SenseData>>(sensors: &mut Sensors<U>) {
    // Add a sensor value that's exactly half of the total range
    // Once averaging is complete, we'll get a result (1 value)
    assert!(sensors
        .add::<U2, MaxAdc, MagnetBounds>(0, <MaxAdc>::to_u16() / 2)
        .is_ok());
    let state = sensors.add::<U2, MaxAdc, MagnetBounds>(0, <MaxAdc>::to_u16() / 2);

    match state.clone() {
        Err(SensorError::CalibrationError(data)) => match data.cal {
            CalibrationStatus::SensorDetected => {
                return;
            }
            _ => {}
        },
        _ => {}
    }
    assert!(false, "Unexpected state: {:?}", state);
}

#[test]
fn sensor_detected() {
    setup_logging_lite().ok();

    // Allocate a single sensor
    let mut sensors = Sensors::<U1>::new().unwrap();

    no_magnet::<U1>(&mut sensors);
}

#[test]
fn sensor_missing() {
    setup_logging_lite().ok();

    // Allocate a single sensor
    let mut sensors = Sensors::<U1>::new().unwrap();

    // Add a sensor value of 0
    // (needs 2 samples to finish averaging)
    // Once averaging is complete, we'll get a result
    assert!(sensors.add::<U2, MaxAdc, MagnetBounds>(0, 0).is_ok());
    let state = sensors.add::<U2, MaxAdc, MagnetBounds>(0, 0);

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
fn invalid_reading() {
    setup_logging_lite().ok();

    // Allocate a single sensor
    let mut sensors = Sensors::<U1>::new().unwrap();

    // Add max sensor value
    // (needs 2 samples to finish averaging)
    // Once averaging is complete, we'll get a result
    assert!(sensors
        .add::<U2, MaxAdc, MagnetBounds>(0, <MaxAdc>::to_u16() - 1)
        .is_ok());
    let state = sensors.add::<U2, MaxAdc, MagnetBounds>(0, <MaxAdc>::to_u16() - 1);

    match state.clone() {
        Err(SensorError::CalibrationError(data)) => match data.cal {
            CalibrationStatus::InvalidReading => {
                return;
            }
            _ => {}
        },
        _ => {}
    }
    assert!(false, "Unexpected state: {:?}", state);
}

fn magnet_positive_check<U: ArrayLength<SenseData>>(sensors: &mut Sensors<U>) {
    // Add two values, larger than MaxAdc / 2 + MagnetBounds
    let val = <MaxAdc>::to_u16() / 2 + <MagnetBounds>::to_u16() + 1;
    // (needs 2 samples to finish averaging)
    // Once averaging is complete, we'll get a result
    assert!(sensors.add::<U2, MaxAdc, MagnetBounds>(0, val).is_ok());
    let state = sensors.add::<U2, MaxAdc, MagnetBounds>(0, val);

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
            if val.cal == CalibrationStatus::MagnetDetectedPositive {
                test = true;
            }
        }
        _ => {}
    }
    assert!(test, "Unexpected state: {:?}", state);
}

fn magnet_negative_check<U: ArrayLength<SenseData>>(sensors: &mut Sensors<U>) {
    // Add two values, smaller than MaxAdc / 2 - MagnetBounds
    let val = <MaxAdc>::to_u16() / 2 - <MagnetBounds>::to_u16() - 1;
    // (needs 2 samples to finish averaging)
    // Once averaging is complete, we'll get a result
    assert!(sensors.add::<U2, MaxAdc, MagnetBounds>(0, val).is_ok());
    let state = sensors.add::<U2, MaxAdc, MagnetBounds>(0, val);

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
            if val.cal == CalibrationStatus::MagnetDetectedNegative {
                test = true;
            }
        }
        _ => {}
    }
    assert!(test, "Unexpected state: {:?}", state);
}

#[test]
fn magnet_positive() {
    setup_logging_lite().ok();

    // Allocate a single sensor
    let mut sensors = Sensors::<U1>::new().unwrap();

    magnet_positive_check::<U1>(&mut sensors);
}

#[test]
fn magnet_negative() {
    setup_logging_lite().ok();

    // Allocate a single sensor
    let mut sensors = Sensors::<U1>::new().unwrap();

    magnet_negative_check::<U1>(&mut sensors);
}

#[test]
fn magnet_positive_remove_negative() {
    setup_logging_lite().ok();

    // Allocate a single sensor
    let mut sensors = Sensors::<U1>::new().unwrap();

    magnet_positive_check::<U1>(&mut sensors);
    no_magnet::<U1>(&mut sensors);
    magnet_negative_check::<U1>(&mut sensors);
}

#[test]
fn magnet_negative_remove_positive() {
    setup_logging_lite().ok();

    // Allocate a single sensor
    let mut sensors = Sensors::<U1>::new().unwrap();

    magnet_negative_check::<U1>(&mut sensors);
    no_magnet::<U1>(&mut sensors);
    magnet_positive_check::<U1>(&mut sensors);
}

// TODO Tests
// - Positive adjustments (repeat for negative adjustments)
//   * When magnet detected (coming from SensorDetected), reset min/max
//   * Default min -> lower min
//   * Default min (recalibrates when going into Magnet+ state) -> higher min
//   * SensorMissing should be a dead state
