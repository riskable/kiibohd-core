// Copyright 2021 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

mod animation;
mod layer;
mod led;
mod rotation;
mod switch;

mod convert {
    use crate::converters::{animation, layer, led, rotation, switch};
    use crate::{CapabilityEvent, CapabilityRun, TriggerEvent};

    impl From<TriggerEvent> for CapabilityRun {
        fn from(event: TriggerEvent) -> Self {
            match event {
                TriggerEvent::Switch { .. } => switch::convert(event),
                TriggerEvent::Layer { .. } => layer::convert(event),
                TriggerEvent::HidLed { .. } => led::convert(event),
                TriggerEvent::Animation { .. } => animation::convert(event),
                TriggerEvent::Rotation { .. } => rotation::convert(event),
                TriggerEvent::None => CapabilityRun::NoOp {
                    state: CapabilityEvent::None,
                },
                // TriggerEvent::AnalogDistance=>,
                // TriggerEvent::AnalogVelocity=>,
                // TriggerEvent::AnalogAcceleration=>,
                // TriggerEvent::AnalogJerk=>,
                // TriggerEvent::Sleep =>,
                // TriggerEvent::Resume=>,
                // TriggerEvent::Inactive=>,
                // TriggerEvent::Active=>,
                other => {
                    panic!("*** remove once all events are handled ***\n TriggerEvent {:?} not recognised", other)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{CapabilityEvent, CapabilityRun, TriggerEvent};

    #[test]
    fn non_event_converted_to_noop_run() {
        let expected = CapabilityRun::NoOp {
            state: CapabilityEvent::None,
        };
        let result: CapabilityRun = TriggerEvent::None.into();

        assert_eq!(result, expected);
    }
}
