// Copyright 2021 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::trigger::Phro;
use crate::{CapabilityEvent, CapabilityRun, TriggerEvent};

/// Converts the passed `TriggerEvent` into a `CapabilityRun::HidKeyboard`
///
/// # Arguments
///
/// * `event`: The TriggerEvent to convert.  This should always be `TriggerEvent::Switch`,
/// if it is anything else a CapabilityRun::NoOp will be returned
///
/// returns: CapabilityRun::HidKeyboard
pub(super) fn convert(event: TriggerEvent) -> CapabilityRun {
    if let TriggerEvent::Switch { state, index, .. } = event {
        let key = index.into();
        match state {
            Phro::Press => CapabilityRun::HidKeyboard {
                state: CapabilityEvent::Initial,
                id: key,
            },
            Phro::Hold => CapabilityRun::HidKeyboard {
                state: CapabilityEvent::Any,
                id: key,
            },
            Phro::Release => CapabilityRun::HidKeyboard {
                state: CapabilityEvent::Last,
                id: key,
            },
            Phro::Off => CapabilityRun::HidKeyboard {
                state: CapabilityEvent::None,
                id: key,
            },
            _ => {
                log::warn!("Unexpected state {:?}", state);
                CapabilityRun::NoOp {
                    state: CapabilityEvent::None,
                }
            }
        }
    } else {
        log::error!("Unexpected event {:?}", event);
        CapabilityRun::NoOp {
            state: CapabilityEvent::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::converters::switch::convert;
    use crate::trigger::{Aodo, Phro};
    use crate::{CapabilityEvent, CapabilityRun, TriggerEvent};
    use kll_hid::Keyboard;

    #[test]
    fn convert_press_returns_an_initial_capability_event() {
        let a = TriggerEvent::Switch {
            state: Phro::Press,
            index: Keyboard::A.into(),
            last_state: 0,
        };
        check_switch_results(convert(a), CapabilityEvent::Initial, Keyboard::A);
    }

    #[test]
    fn convert_hold_returns_an_any_event() {
        let a = TriggerEvent::Switch {
            state: Phro::Hold,
            index: Keyboard::A.into(),
            last_state: 0,
        };
        check_switch_results(convert(a), CapabilityEvent::Any, Keyboard::A);
    }

    #[test]
    fn convert_release_returns_a_last_capability_event() {
        let a = TriggerEvent::Switch {
            state: Phro::Release,
            index: Keyboard::A.into(),
            last_state: 0,
        };
        check_switch_results(convert(a), CapabilityEvent::Last, Keyboard::A);
    }

    #[test]
    fn convert_off_returns_a_none_capability_event() {
        let a = TriggerEvent::Switch {
            state: Phro::Off,
            index: Keyboard::A.into(),
            last_state: 0,
        };
        check_switch_results(convert(a), CapabilityEvent::None, Keyboard::A);
    }

    #[test]
    fn convert_unexpected_state_type_returns_noop() {
        let a = TriggerEvent::Switch {
            state: Phro::Passthrough,
            index: Keyboard::A.into(),
            last_state: 0,
        };
        let result = convert(a);

        assert_eq!(
            result,
            CapabilityRun::NoOp {
                state: CapabilityEvent::None
            }
        )
    }

    #[test]
    fn convert_unexpected_trigger_event_type_returns_noop() {
        let a = TriggerEvent::Sleep {
            state: Aodo::Activate,
            last_state: 0,
        };
        let result = convert(a);

        assert_eq!(
            result,
            CapabilityRun::NoOp {
                state: CapabilityEvent::None
            }
        )
    }

    fn check_switch_results(
        event: CapabilityRun,
        expected_event: CapabilityEvent,
        expected_key: Keyboard,
    ) {
        if let CapabilityRun::HidKeyboard { state, id } = event {
            assert_eq!(state, expected_event);
            assert_eq!(id, expected_key);
        } else {
            panic!("process_state failed to return a HidKeyboard")
        }
    }
}
