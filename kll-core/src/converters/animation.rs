// Copyright 2021 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::trigger::Dro;
use crate::{error, warn};
use crate::{CapabilityEvent, CapabilityRun, TriggerEvent};

/// Converts the passed `TriggerEvent` into a `CapabilityRun::PixelAnimationIndex`
///
/// # Arguments
///
/// * `event`: The TriggerEvent to convert.  This should always be `TriggerEvent::Animation`,
/// if it is anything else a CapabilityRun::NoOp will be returned
///
/// returns: CapabilityRun::PixelAnimationIndex
pub(super) fn convert(event: TriggerEvent) -> CapabilityRun {
    if let TriggerEvent::Animation { state, index, .. } = event {
        match state {
            Dro::Off => CapabilityRun::PixelAnimationIndex {
                state: CapabilityEvent::None,
                index,
            },
            Dro::Done => CapabilityRun::PixelAnimationIndex {
                state: CapabilityEvent::Last,
                index,
            },
            Dro::Repeat => CapabilityRun::PixelAnimationIndex {
                state: CapabilityEvent::Initial,
                index,
            },
            _ => {
                warn!("Unexpected state {:?}", state);
                CapabilityRun::NoOp {
                    state: CapabilityEvent::None,
                }
            }
        }
    } else {
        error!("Unexpected event {:?}", event);
        CapabilityRun::NoOp {
            state: CapabilityEvent::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::converters::animation;
    use crate::trigger::{Aodo, Dro};
    use crate::{CapabilityEvent, CapabilityRun, TriggerEvent};
    use kll_hid::Keyboard;

    #[test]
    fn animation_off_to_capability_run_none() {
        let a = TriggerEvent::Animation {
            state: Dro::Off,
            index: Keyboard::A.into(),
            last_state: 0,
        };
        let result = animation::convert(a);

        check_results(result, CapabilityEvent::None, Keyboard::A);
    }

    #[test]
    fn animation_done_to_capability_run_last() {
        let a = TriggerEvent::Animation {
            state: Dro::Done,
            index: Keyboard::B.into(),
            last_state: 0,
        };
        let result = animation::convert(a);

        check_results(result, CapabilityEvent::Last, Keyboard::B);
    }

    #[test]
    fn animation_repeat_to_capability_run_initial() {
        let a = TriggerEvent::Animation {
            state: Dro::Repeat,
            index: Keyboard::C.into(),
            last_state: 0,
        };
        let result = animation::convert(a);
        check_results(result, CapabilityEvent::Initial, Keyboard::C);
    }

    #[test]
    fn convert_unexpected_state_type_returns_noop() {
        let a = TriggerEvent::Animation {
            state: Dro::Passthrough,
            index: Keyboard::A.into(),
            last_state: 0,
        };
        let result = animation::convert(a);

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
        let result = animation::convert(a);

        assert_eq!(
            result,
            CapabilityRun::NoOp {
                state: CapabilityEvent::None
            }
        )
    }

    fn check_results(
        event: CapabilityRun,
        expected_event: CapabilityEvent,
        expected_index: Keyboard,
    ) {
        if let CapabilityRun::PixelAnimationIndex { state, index } = event {
            assert_eq!(state, expected_event);
            assert_eq!(index, expected_index.into());
        } else {
            panic!("failed to return a LayerState")
        }
    }
}
