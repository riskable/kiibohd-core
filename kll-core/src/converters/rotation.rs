// Copyright 2021-2022 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::error;
use crate::{CapabilityEvent, CapabilityRun, TriggerEvent};

/// Converts the passed TriggerEvent into a CapabilityRun::Rotate
///
/// # Arguments
///
/// * `event`: The TriggerEvent to convert.  This should always be a `TriggerEvent::Rotation`,
/// if it is anything else a CapabilityRun::NoOp will be returned
///
/// returns: CapabilityRun::Rotate
pub(super) fn convert(event: TriggerEvent) -> CapabilityRun {
    if let TriggerEvent::Rotation {
        index, position, ..
    } = event
    {
        CapabilityRun::Rotate {
            state: CapabilityEvent::Any,
            index,
            increment: position,
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
    use crate::converters::rotation::convert;
    use crate::trigger::Aodo;
    use crate::{CapabilityEvent, CapabilityRun, TriggerEvent};

    #[test]
    fn create_rotate_event() {
        let a = TriggerEvent::Rotation {
            index: 5,
            position: 3,
            last_state: 0,
        };
        assert_eq!(
            convert(a),
            CapabilityRun::Rotate {
                state: CapabilityEvent::Any,
                index: 5,
                increment: 3,
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
}
