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

#![no_std]

mod rawlookup;
mod test;

// ----- Crates -----

use heapless::{ArrayLength, Vec};
use log::trace;
use typenum::Unsigned;

// TODO Use features to determine which lookup table to use
use rawlookup::MODEL;

// ----- Sense Data -----

/// Calibration status indicates if a sensor position is ready to send
/// analysis for a particular key.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum CalibrationStatus {
    NotReady,       // Still trying to determine status (from power-on)
    SensorDetected, // Sensor detected, but no magnet
    // ADC value 50% of max
    SensorMissing, // ADC value at 0
    // Generally this is not possible with a magnet
    // unless the magnet is far to strong.
    MagnetDetectedPositive, // Magnet detected, min calibrated, positive range
    MagnetDetectedNegative, // Magnet detected, max calibrated, negative range
    InvalidReading,         // Reading higher than ADC supports (invalid)
}

#[derive(Clone, Debug)]
pub enum SensorError {
    CalibrationError(SenseData),
    FailedToResize(usize),
    InvalidSensor(usize),
}

/// Calculations:
///  d = linearized(adc sample) --> distance
///  v = (d - d_prev) / 1       --> velocity
///  a = (v - v_prev) / 2       --> acceleration
///  j = (a - a_prev) / 3       --> jerk
///
/// These calculations assume constant time delta of 1
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SenseAnalysis {
    raw: u16,          // Raw ADC reading
    distance: i16,     // Distance value (lookup + min/max alignment)
    velocity: i16,     // Velocity calculation (*)
    acceleration: i16, // Acceleration calculation (*)
    jerk: i16,         // Jerk calculation (*)
}

impl SenseAnalysis {
    /// Using the raw value do calculations
    /// Requires the previous analysis
    fn new(raw: u16, data: &SenseData) -> SenseAnalysis {
        // Do raw lookup (we've already checked the bounds)
        let initial_distance = MODEL[raw as usize];

        // Min/max adjustment
        let distance_offset = match data.cal {
            CalibrationStatus::MagnetDetectedPositive => {
                // Subtract the min lookup
                // Lookup table has negative values for unexpectedly
                // small values (greater than sensor center)
                MODEL[data.min as usize]
            }
            CalibrationStatus::MagnetDetectedNegative => {
                // Subtract the max lookup
                // Lookup table has negative values for unexpectedly
                // small values (greater than sensor center)
                MODEL[data.max as usize]
            }
            _ => {
                // Invalid reading
                return SenseAnalysis::null();
            }
        };
        let distance = initial_distance - distance_offset;
        let velocity = (distance - data.analysis.distance) / 1;
        let acceleration = (velocity - data.analysis.velocity) / 2;
        // NOTE: To use jerk, the compile-time thresholds will need to be
        //       multiplied by 3 (to account for the missing / 3)
        let jerk = acceleration - data.analysis.acceleration;
        SenseAnalysis {
            raw,
            distance,
            velocity,
            acceleration,
            jerk,
        }
    }

    /// Null entry
    fn null() -> SenseAnalysis {
        SenseAnalysis {
            raw: 0,
            distance: 0,
            velocity: 0,
            acceleration: 0,
            jerk: 0,
        }
    }
}

/// Stores incoming raw samples
#[repr(C)]
#[derive(Clone, Debug)]
pub struct RawData {
    scratch_samples: u8,
    scratch: u32,
}

impl RawData {
    fn new() -> RawData {
        RawData {
            scratch_samples: 0,
            scratch: 0,
        }
    }

    /// Adds to the internal scratch location
    /// Designed to accumulate until a set number of readings added
    /// SC: specifies the number of scratch samples until ready to average
    ///     Should be a power of two (1, 2, 4, 8, 16...) for the compiler to
    ///     optimize.
    fn add<SC: Unsigned>(&mut self, reading: u16) -> Option<u16> {
        self.scratch += reading as u32;
        self.scratch_samples += 1;
        trace!(
            "Reading: {}  Sample: {}/{}",
            reading,
            self.scratch_samples,
            <SC>::to_u8()
        );

        if self.scratch_samples == <SC>::U8 {
            let val = self.scratch / <SC>::U32;
            self.scratch = 0;
            self.scratch_samples = 0;
            Some(val as u16)
        } else {
            None
        }
    }
}

/// Sense data is store per ADC source element (e.g. per key)
/// The analysis is stored in a queue, where old values expire out
/// min/max is used to handle offsets from the distance lookups
/// Higher order calculations assume a constant unit of time between measurements
/// Any division is left to compile-time comparisions as it's not necessary
/// to actually compute the final higher order values in order to make a decision.
/// This diagram can give a sense of how the incoming data is used.
/// The number represents the last ADC sample required to calculate the value.
///
/// ```text,ignore
///
///            4  5 ... <- Jerk (e.g. m/2^3)
///          / | /|
///         3  4  5 ... <- Acceleration (e.g. m/2^2)
///       / | /| /|
///      2  3  4  5 ... <- Velocity (e.g. m/s)
///    / | /| /| /|
///   1  2  3  4  5 ... <- Distance (e.g. m)
///  ----------------------
///   1  2  3  4  5 ... <== ADC Averaged Sample
///
/// ```
///
/// Distance     => Min/Max adjusted lookup
/// Velocity     => (d_current - d_previous) / 1 (constant time)
///                 There is 1 time unit between samples 1 and 2
/// Acceleration => (v_current - v_previous) / 2 (constant time)
///                 There are 2 time units between samples 1 and 3
/// Jerk         => (a_current - a_previous) / 3 (constant time)
///                 There are 3 time units between samples 1 and 4
///
/// NOTE: Division is computed at compile time for jerk (/ 3)
///
/// Time is simplified to 1 unit (normally sampling will be at a constant time-rate, so this should be somewhat accurate).
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SenseData {
    pub analysis: SenseAnalysis,
    pub cal: CalibrationStatus,
    pub data: RawData,
    pub min: u16,
    pub max: u16,
}

impl SenseData {
    fn new() -> SenseData {
        SenseData {
            analysis: SenseAnalysis::null(),
            cal: CalibrationStatus::NotReady,
            data: RawData::new(),
            min: 0xFFFF,
            max: 0x0000,
        }
    }

    /// Acculumate a new sensor reading
    /// Once the required number of samples is retrieved, do analysis
    /// Analysis does a few more addition, subtraction and comparisions
    /// so it's a more expensive operation.
    fn add<SC: Unsigned, MX: Unsigned, RNG: Unsigned>(
        &mut self,
        reading: u16,
    ) -> Result<Option<&SenseAnalysis>, SensorError> {
        // Add value to accumulator
        if let Some(data) = self.data.add::<SC>(reading) {
            // Check min/max values
            if data > self.max {
                self.max = data;
            }
            if data < self.min {
                self.min = data;
            }

            // Check calibration
            self.cal = self.check_calibration::<MX, RNG>(data);
            trace!("Reading: {}  Cal: {:?}", reading, self.cal);
            match self.cal {
                // Don't bother doing calculations if magnet+sensor isn't ready
                CalibrationStatus::NotReady
                | CalibrationStatus::InvalidReading
                | CalibrationStatus::SensorDetected
                | CalibrationStatus::SensorMissing => {
                    // Reset min/max
                    self.min = 0xFFFF;
                    self.max = 0x0000;
                    return Err(SensorError::CalibrationError(self.clone()));
                }
                _ => {}
            }

            // Calculate new analysis (requires previous results + min/max)
            self.analysis = SenseAnalysis::new(data, &self);
            Ok(Some(&self.analysis))
        } else {
            Ok(None)
        }
    }

    /// Update calibration state
    /// MX:  Max sensor value
    /// RNG: +/- center threshold for magnet detection (Range Magnet)
    fn check_calibration<MX: Unsigned, RNG: Unsigned>(&self, data: u16) -> CalibrationStatus {
        // TODO(HaaTa): Should we force full recalibration periodically?
        //              (drop min/max values)
        let lowmid = <MX>::U16 / 2 - <RNG>::U16; // Middle - range
        let highmid = <MX>::U16 / 2 + <RNG>::U16; // Middle + range

        // Check if value is invalid
        if data >= <MX>::U16 - 1 {
            return CalibrationStatus::InvalidReading;
        }

        // No sensor found
        if data == 0 {
            return CalibrationStatus::SensorMissing;
        }

        // Check if magnet found
        if data > highmid {
            return CalibrationStatus::MagnetDetectedPositive;
        }
        if data < lowmid {
            return CalibrationStatus::MagnetDetectedNegative;
        }

        // Check for sensor, no magnet detected
        if data <= highmid && data >= lowmid {
            return CalibrationStatus::SensorDetected;
        }

        CalibrationStatus::NotReady
    }
}

impl Default for SenseData {
    fn default() -> Self {
        SenseData::new()
    }
}

// ----- Hall Effect Interface ------

pub struct Sensors<S: ArrayLength<SenseData>> {
    sensors: Vec<SenseData, S>,
}

impl<S: ArrayLength<SenseData>> Sensors<S> {
    /// Initializes full Sensor array
    /// Only fails if static allocation fails (very unlikely)
    pub fn new() -> Result<Sensors<S>, SensorError> {
        let mut sensors = Vec::new();
        if sensors.resize_default(<S>::to_usize()).is_err() {
            Err(SensorError::FailedToResize(<S>::to_usize()))
        } else {
            Ok(Sensors { sensors })
        }
    }

    pub fn add<SC: Unsigned, MX: Unsigned, RNG: Unsigned>(
        &mut self,
        index: usize,
        reading: u16,
    ) -> Result<Option<&SenseAnalysis>, SensorError> {
        trace!("Index: {}  Reading: {}", index, reading);
        if index < self.sensors.len() {
            self.sensors[index].add::<SC, MX, RNG>(reading)
        } else {
            Err(SensorError::InvalidSensor(index))
        }
    }

    pub fn get_data(&self, index: usize) -> Result<&SenseData, SensorError> {
        if index < self.sensors.len() {
            if self.sensors[index].cal == CalibrationStatus::NotReady {
                Err(SensorError::CalibrationError(self.sensors[index].clone()))
            } else {
                Ok(&self.sensors[index])
            }
        } else {
            Err(SensorError::InvalidSensor(index))
        }
    }
}
