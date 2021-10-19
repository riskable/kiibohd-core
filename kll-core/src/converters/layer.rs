// Copyright 2021 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::layer::State;
use crate::trigger::LayerState;
use crate::{CapabilityEvent, CapabilityRun, TriggerEvent};

/// Converts the passed `TriggerEvent` into a `CapabilityRun::LayerState`
///
/// # Arguments
///
/// * `event`: The TriggerEvent to convert.  This should always be `TriggerEvent::Layer`,
/// if it is anything else a CapabilityRun::NoOp will be returned
///
/// returns: CapabilityRun::LayerState
pub(super) fn convert(event: TriggerEvent) -> CapabilityRun {
    if let TriggerEvent::Layer { state, layer, .. } = event {
        match state {
            LayerState::ShiftActivate => CapabilityRun::LayerState {
                state: CapabilityEvent::Initial,
                layer,
                layer_state: State::Shift,
            },
            LayerState::LatchActivate => CapabilityRun::LayerState {
                state: CapabilityEvent::Initial,
                layer,
                layer_state: State::Latch,
            },
            LayerState::ShiftLatchActivate => CapabilityRun::LayerState {
                state: CapabilityEvent::Initial,
                layer,
                layer_state: State::ShiftLatch,
            },
            LayerState::LockActivate => CapabilityRun::LayerState {
                state: CapabilityEvent::Initial,
                layer,
                layer_state: State::Lock,
            },
            LayerState::ShiftLockActivate => CapabilityRun::LayerState {
                state: CapabilityEvent::Initial,
                layer,
                layer_state: State::ShiftLock,
            },
            LayerState::LatchLockActivate => CapabilityRun::LayerState {
                state: CapabilityEvent::Initial,
                layer,
                layer_state: State::LatchLock,
            },
            LayerState::ShiftLatchLockActivate => CapabilityRun::LayerState {
                state: CapabilityEvent::Initial,
                layer,
                layer_state: State::ShiftLatchLock,
            },
            LayerState::ShiftOn => CapabilityRun::LayerState {
                state: CapabilityEvent::Any,
                layer,
                layer_state: State::Shift,
            },
            LayerState::LatchOn => CapabilityRun::LayerState {
                state: CapabilityEvent::Any,
                layer,
                layer_state: State::Latch,
            },
            LayerState::ShiftLatchOn => CapabilityRun::LayerState {
                state: CapabilityEvent::Any,
                layer,
                layer_state: State::ShiftLatch,
            },
            LayerState::LockOn => CapabilityRun::LayerState {
                state: CapabilityEvent::Any,
                layer,
                layer_state: State::Lock,
            },
            LayerState::ShiftLockOn => CapabilityRun::LayerState {
                state: CapabilityEvent::Any,
                layer,
                layer_state: State::ShiftLock,
            },
            LayerState::LatchLockOn => CapabilityRun::LayerState {
                state: CapabilityEvent::Any,
                layer,
                layer_state: State::LatchLock,
            },
            LayerState::ShiftLatchLockOn => CapabilityRun::LayerState {
                state: CapabilityEvent::Any,
                layer,
                layer_state: State::ShiftLatchLock,
            },
            LayerState::ShiftDeactivate => CapabilityRun::LayerState {
                state: CapabilityEvent::Last,
                layer,
                layer_state: State::Shift,
            },
            LayerState::LatchDeactivate => CapabilityRun::LayerState {
                state: CapabilityEvent::Last,
                layer,
                layer_state: State::Latch,
            },
            LayerState::ShiftLatchDeactivate => CapabilityRun::LayerState {
                state: CapabilityEvent::Last,
                layer,
                layer_state: State::ShiftLatch,
            },
            LayerState::LockDeactivate => CapabilityRun::LayerState {
                state: CapabilityEvent::Last,
                layer,
                layer_state: State::Lock,
            },
            LayerState::ShiftLockDeactivate => CapabilityRun::LayerState {
                state: CapabilityEvent::Last,
                layer,
                layer_state: State::ShiftLock,
            },
            LayerState::LatchLockDeactivate => CapabilityRun::LayerState {
                state: CapabilityEvent::Last,
                layer,
                layer_state: State::LatchLock,
            },
            LayerState::ShiftLatchLockDeactivate => CapabilityRun::LayerState {
                state: CapabilityEvent::Last,
                layer,
                layer_state: State::ShiftLatchLock,
            },
            LayerState::ShiftOff => CapabilityRun::LayerState {
                state: CapabilityEvent::None,
                layer,
                layer_state: State::Shift,
            },
            LayerState::LatchOff => CapabilityRun::LayerState {
                state: CapabilityEvent::None,
                layer,
                layer_state: State::Latch,
            },
            LayerState::ShiftLatchOff => CapabilityRun::LayerState {
                state: CapabilityEvent::None,
                layer,
                layer_state: State::ShiftLatch,
            },
            LayerState::LockOff => CapabilityRun::LayerState {
                state: CapabilityEvent::None,
                layer,
                layer_state: State::Lock,
            },
            LayerState::ShiftLockOff => CapabilityRun::LayerState {
                state: CapabilityEvent::None,
                layer,
                layer_state: State::ShiftLock,
            },
            LayerState::LatchLockOff => CapabilityRun::LayerState {
                state: CapabilityEvent::None,
                layer,
                layer_state: State::LatchLock,
            },
            LayerState::ShiftLatchLockOff => CapabilityRun::LayerState {
                state: CapabilityEvent::None,
                layer,
                layer_state: State::ShiftLatchLock,
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
    use crate::converters::layer::convert;
    use crate::layer::State;
    use crate::trigger::{Aodo, LayerState};
    use crate::{CapabilityEvent, CapabilityRun, TriggerEvent};

    #[test]
    fn convert_shift_activate_to_initial_shift_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::ShiftActivate,
            layer: 1,
            last_state: 0,
        };
        check_results(convert(a), CapabilityEvent::Initial, 1, State::Shift)
    }

    #[test]
    fn convert_latch_activate_to_initial_latch_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::LatchActivate,
            layer: 1,
            last_state: 0,
        };
        check_results(convert(a), CapabilityEvent::Initial, 1, State::Latch)
    }

    #[test]
    fn convert_shift_latch_activate_to_initial_shift_latch_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::ShiftLatchActivate,
            layer: 1,
            last_state: 0,
        };
        check_results(convert(a), CapabilityEvent::Initial, 1, State::ShiftLatch)
    }

    #[test]
    fn convert_lock_activate_to_initial_lock_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::LockActivate,
            layer: 1,
            last_state: 0,
        };
        check_results(convert(a), CapabilityEvent::Initial, 1, State::Lock)
    }

    #[test]
    fn convert_shift_lock_activate_to_initial_shift_lock_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::ShiftLockActivate,
            layer: 1,
            last_state: 0,
        };
        check_results(convert(a), CapabilityEvent::Initial, 1, State::ShiftLock)
    }

    #[test]
    fn convert_latch_lock_activate_to_initial_latch_lock_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::LatchLockActivate,
            layer: 1,
            last_state: 0,
        };
        check_results(convert(a), CapabilityEvent::Initial, 1, State::LatchLock)
    }

    #[test]
    fn convert_shift_latch_lock_activate_to_initial_shift_latch_lock_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::ShiftLatchLockActivate,
            layer: 1,
            last_state: 0,
        };
        check_results(
            convert(a),
            CapabilityEvent::Initial,
            1,
            State::ShiftLatchLock,
        )
    }

    #[test]
    fn convert_shift_on_to_any_shift_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::ShiftOn,
            layer: 1,
            last_state: 0,
        };
        check_results(convert(a), CapabilityEvent::Any, 1, State::Shift)
    }

    #[test]
    fn convert_latch_on_to_any_latch_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::LatchOn,
            layer: 1,
            last_state: 0,
        };
        check_results(convert(a), CapabilityEvent::Any, 1, State::Latch)
    }

    #[test]
    fn convert_shift_latch_on_to_any_shift_latch_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::ShiftLatchOn,
            layer: 1,
            last_state: 0,
        };
        check_results(convert(a), CapabilityEvent::Any, 1, State::ShiftLatch)
    }

    #[test]
    fn convert_lock_on_to_any_lock_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::LockOn,
            layer: 1,
            last_state: 0,
        };
        check_results(convert(a), CapabilityEvent::Any, 1, State::Lock)
    }

    #[test]
    fn convert_shift_lock_on_to_any_shift_lock_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::ShiftLockOn,
            layer: 1,
            last_state: 0,
        };
        check_results(convert(a), CapabilityEvent::Any, 1, State::ShiftLock)
    }

    #[test]
    fn convert_latch_lock_on_to_any_latch_lock_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::LatchLockOn,
            layer: 1,
            last_state: 0,
        };
        check_results(convert(a), CapabilityEvent::Any, 1, State::LatchLock)
    }

    #[test]
    fn convert_shift_latch_lock_on_to_any_shift_latch_lock_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::ShiftLatchLockOn,
            layer: 1,
            last_state: 0,
        };
        check_results(convert(a), CapabilityEvent::Any, 1, State::ShiftLatchLock)
    }

    #[test]
    fn convert_shift_deactivate_to_last_shift_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::ShiftDeactivate,
            layer: 1,
            last_state: 0,
        };
        check_results(convert(a), CapabilityEvent::Last, 1, State::Shift)
    }

    #[test]
    fn convert_latch_deactivate_to_last_latch_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::LatchDeactivate,
            layer: 1,
            last_state: 0,
        };
        check_results(convert(a), CapabilityEvent::Last, 1, State::Latch)
    }

    #[test]
    fn convert_shift_latch_deactivate_to_last_shift_latch_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::ShiftLatchDeactivate,
            layer: 1,
            last_state: 0,
        };
        check_results(convert(a), CapabilityEvent::Last, 1, State::ShiftLatch)
    }

    #[test]
    fn convert_lock_deactivate_to_last_lock_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::LockDeactivate,
            layer: 1,
            last_state: 0,
        };
        check_results(convert(a), CapabilityEvent::Last, 1, State::Lock)
    }

    #[test]
    fn convert_shift_lock_deactivate_to_last_shift_lock_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::ShiftLockDeactivate,
            layer: 1,
            last_state: 0,
        };
        check_results(convert(a), CapabilityEvent::Last, 1, State::ShiftLock)
    }

    #[test]
    fn convert_latch_lock_deactivate_to_last_latch_lock_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::LatchLockDeactivate,
            layer: 1,
            last_state: 0,
        };
        check_results(convert(a), CapabilityEvent::Last, 1, State::LatchLock)
    }

    #[test]
    fn convert_shift_latch_lock_deactivate_to_last_shift_latch_lock_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::ShiftLatchLockDeactivate,
            layer: 1,
            last_state: 0,
        };
        check_results(convert(a), CapabilityEvent::Last, 1, State::ShiftLatchLock)
    }

    #[test]
    fn convert_shift_off_to_none_shift_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::ShiftOff,
            layer: 1,
            last_state: 0,
        };
        check_results(convert(a), CapabilityEvent::None, 1, State::Shift)
    }

    #[test]
    fn convert_latch_off_to_none_latch_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::LatchOff,
            layer: 1,
            last_state: 0,
        };
        check_results(convert(a), CapabilityEvent::None, 1, State::Latch)
    }

    #[test]
    fn convert_shift_latch_off_to_none_shift_latch_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::ShiftLatchOff,
            layer: 1,
            last_state: 0,
        };
        check_results(convert(a), CapabilityEvent::None, 1, State::ShiftLatch)
    }

    #[test]
    fn convert_lock_off_to_none_lock_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::LockOff,
            layer: 1,
            last_state: 0,
        };
        check_results(convert(a), CapabilityEvent::None, 1, State::Lock)
    }

    #[test]
    fn convert_shift_lock_off_to_none_shift_lock_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::ShiftLockOff,
            layer: 1,
            last_state: 0,
        };
        check_results(convert(a), CapabilityEvent::None, 1, State::ShiftLock)
    }

    #[test]
    fn convert_latch_lock_off_to_none_latch_lock_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::LatchLockOff,
            layer: 1,
            last_state: 0,
        };
        check_results(convert(a), CapabilityEvent::None, 1, State::LatchLock)
    }

    #[test]
    fn convert_shift_latch_lock_off_to_none_shift_latch_lock_layer_state() {
        let a = TriggerEvent::Layer {
            state: LayerState::ShiftLatchLockOff,
            layer: 1,
            last_state: 0,
        };
        check_results(convert(a), CapabilityEvent::None, 1, State::ShiftLatchLock)
    }

    #[test]
    fn convert_unexpected_state_type_returns_noop() {
        let a = TriggerEvent::Layer {
            state: LayerState::Passthrough,
            layer: 2,
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

    fn check_results(
        event: CapabilityRun,
        expected_event: CapabilityEvent,
        expected_layer: u8,
        expected_layer_state: State,
    ) {
        if let CapabilityRun::LayerState {
            state,
            layer,
            layer_state,
        } = event
        {
            assert_eq!(state, expected_event);
            assert_eq!(layer, expected_layer);
            assert_eq!(layer_state, expected_layer_state);
        } else {
            panic!("process_state failed to return a LayerState")
        }
    }
}
