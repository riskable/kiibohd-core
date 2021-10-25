// Copyright 2021 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::trigger::Aodo;
use crate::{CapabilityEvent, CapabilityRun, TriggerEvent};

/// Converts the passed `TriggerEvent` into a `CapabilityRun::HidLed`
///
/// # Arguments
///
/// * `event`: The TriggerEvent to convert.  This should always be `TriggerEvent::HidLed`,
/// if it is anything else a CapabilityRun::NoOp will be returned
///
/// returns: CapabilityRun::HidLed
pub(super) fn convert(event: TriggerEvent) -> CapabilityRun {
    if let TriggerEvent::HidLed { state, index, .. } = event {
        let key = index.into();
        match state {
            Aodo::Activate => CapabilityRun::HidLed {
                state: CapabilityEvent::Initial,
                id: key,
            },
            Aodo::On => CapabilityRun::HidLed {
                state: CapabilityEvent::Any,
                id: key,
            },
            Aodo::Deactivate => CapabilityRun::HidLed {
                state: CapabilityEvent::Last,
                id: key,
            },
            Aodo::Off => CapabilityRun::HidLed {
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
    use crate::converters::led::convert;
    use crate::trigger::Aodo;
    use crate::{CapabilityEvent, CapabilityRun, TriggerEvent};
    use kll_hid::LedIndicator;

    #[test]
    fn convert_activate_returns_an_initial_capability_event() {
        let a = TriggerEvent::HidLed {
            state: Aodo::Activate,
            index: LedIndicator::CapsLock.into(),
            last_state: 0,
        };
        check_switch_results(convert(a), CapabilityEvent::Initial, LedIndicator::CapsLock);
    }

    #[test]
    fn convert_on_returns_an_any_event() {
        let a = TriggerEvent::HidLed {
            state: Aodo::On,
            index: LedIndicator::CapsLock.into(),
            last_state: 0,
        };
        check_switch_results(convert(a), CapabilityEvent::Any, LedIndicator::CapsLock);
    }

    #[test]
    fn convert_deactivate_returns_a_last_capability_event() {
        let a = TriggerEvent::HidLed {
            state: Aodo::Deactivate,
            index: LedIndicator::CapsLock.into(),
            last_state: 0,
        };
        check_switch_results(convert(a), CapabilityEvent::Last, LedIndicator::CapsLock);
    }

    #[test]
    fn convert_off_returns_a_none_capability_event() {
        let a = TriggerEvent::HidLed {
            state: Aodo::Off,
            index: LedIndicator::CapsLock.into(),
            last_state: 0,
        };
        check_switch_results(convert(a), CapabilityEvent::None, LedIndicator::CapsLock);
    }

    #[test]
    fn convert_unexpected_state_type_returns_noop() {
        let a = TriggerEvent::HidLed {
            state: Aodo::Passthrough,
            index: LedIndicator::CameraOn.into(),
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
        expected_led: LedIndicator,
    ) {
        if let CapabilityRun::HidLed { state, id } = event {
            assert_eq!(state, expected_event);
            assert_eq!(id, expected_led);
        } else {
            panic!("process_state failed to return a HidLed")
        }
    }
}
