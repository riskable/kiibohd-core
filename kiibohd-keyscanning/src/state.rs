// Copyright 2021 Zion Koyl
// Copyright 2021 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[derive(PartialEq, Copy, Clone, Debug, defmt::Format)]
pub enum State {
    On,
    Off,
}

/// The KeyState handles all of the decision making and state changes based on a high or low signal from a GPIO pin
#[derive(Copy, Clone)]
pub struct KeyState<const SCAN_PERIOD_US: u32, const DEBOUNCE_US: u32, const IDLE_MS: u32> {
    /// Most recently GPIO reading (not debounced)
    raw_state: State,

    /// True key state after debounce processing (debounced)
    state: State,

    /// Used to determine if the key is idle (in Off state for IDLE_MS)
    idle: bool,

    /// Used to track the number of cycles since state has changed.
    cycles_since_state_change: u32,

    /// This is used to track the list GPIO read bounce
    ///
    /// If cycles * scan_period > DEBOUNCE_US then raw_state is assigned to state.
    cycles_since_last_bounce: u32,
}

impl<const SCAN_PERIOD_US: u32, const DEBOUNCE_US: u32, const IDLE_MS: u32>
    KeyState<SCAN_PERIOD_US, DEBOUNCE_US, IDLE_MS>
{
    pub fn new() -> Self {
        Self {
            raw_state: State::Off,
            state: State::Off,
            idle: false,
            cycles_since_state_change: 0,
            cycles_since_last_bounce: 0,
        }
    }

    /// Record the GPIO read event and adjust debounce state machine accordingly
    ///
    /// Returns:
    /// (State, idle, cycles_since_state_change)
    pub fn record(&mut self, on: bool) -> (State, bool, u32) {
        // Update the raw state as a bounce event if not the same as the previous scan iteration
        // e.g. GPIO read value has changed since the last iteration
        if on && self.raw_state == State::Off || !on && self.raw_state == State::On {
            // Update raw state
            self.raw_state = if on { State::On } else { State::Off };

            // Reset bounce cycle counter
            self.cycles_since_last_bounce = 0;

            // Return current state
            return self.state();
        }

        // Increment debounce cycle counter
        self.cycles_since_last_bounce += 1;

        // Update the debounced state if it has changed and exceeded the debounce timer
        // (debounce timer resets if there is any bouncing during the debounce interval).
        if self.cycles_since_last_bounce * SCAN_PERIOD_US >= DEBOUNCE_US
            && self.raw_state != self.state
        {
            // Update state
            self.state = self.raw_state;

            // Reset state transition cycle counter
            self.cycles_since_state_change = 0;

            // No longer idle
            self.idle = false;

            // Return current state
            return self.state();
        }

        // Increment state cycle counter
        self.cycles_since_state_change += 1;

        // Determine if key is idle
        // Must be both in the off state and have been off >= IDLE_MS
        self.idle = self.state == State::Off
            && self.cycles_since_state_change * SCAN_PERIOD_US / 1000 >= IDLE_MS;

        // Return current state
        self.state()
    }

    /// Returns thet current state and cycles since the state changed
    ///
    /// (State, idle, cycles_since_state_change)
    pub fn state(&self) -> (State, bool, u32) {
        (self.state, self.idle, self.cycles_since_state_change)
    }
}

impl<const SCAN_PERIOD_US: u32, const DEBOUNCE_US: u32, const IDLE_MS: u32> Default
    for KeyState<SCAN_PERIOD_US, DEBOUNCE_US, IDLE_MS>
{
    fn default() -> Self {
        Self::new()
    }
}
