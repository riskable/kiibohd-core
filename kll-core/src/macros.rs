// Copyright 2021 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use heapless::spsc::Queue;

use crate::{CapabilityEvent, CapabilityRun, TriggerEvent};

pub struct Macro<const TSIZE: usize, const CSIZE: usize> {
    inputs: Queue<TriggerEvent, TSIZE>,
    outputs: Queue<CapabilityRun, CSIZE>,
}

impl<const TSIZE: usize, const CSIZE: usize> Macro<TSIZE, CSIZE> {
    fn new(inputs: Queue<TriggerEvent, TSIZE>, outputs: Queue<CapabilityRun, CSIZE>) -> Macro<TSIZE, CSIZE> {
        Macro { inputs, outputs }
    }

    fn process(&mut self) {
        while let Some(input) = self.inputs.dequeue() {
            self.outputs.enqueue(match input {
                _ => CapabilityRun::NoOp { state: CapabilityEvent::Passthrough(input) },
                // TriggerEvent::None => ,
                // TriggerEvent::Switch => ,
                // TriggerEvent::HidLed=> ,
                // TriggerEvent::AnalogDistance=>,
                // TriggerEvent::AnalogVelocity=>,
                // TriggerEvent::AnalogAcceleration=>,
                // TriggerEvent::AnalogJerk=>,
                // TriggerEvent::Layer=>,
                // TriggerEvent::Animation=>,
                // TriggerEvent::Sleep=>,
                // TriggerEvent::Resume=>,
                // TriggerEvent::Inactive=>,
                // TriggerEvent::Active=>,
                // TriggerEvent::Rotation=>,
            }).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use heapless::spsc::Queue;

    use crate::macros::Macro;
    use crate::trigger::Aodo::Activate;
    use crate::TriggerEvent;

    #[test]
    fn processing_empty_input_queue_results_in_nothing_being_added_to_the_output_queue() {
        let mut process_queue: Macro<1024, 1024> = Macro::new(Queue::new(), Queue::new());
        assert_eq!(process_queue.inputs.len(), 0);
        assert_eq!(process_queue.outputs.len(), 0);
        process_queue.process();
        assert_eq!(process_queue.inputs.len(), 0);
        assert_eq!(process_queue.outputs.len(), 0);
    }

    #[test]
    fn processing_an_input_queue_adds_capability_runs_to_the_output_queue() {
        let mut inputs = Queue::new();
        inputs.enqueue(TriggerEvent::None).unwrap();
        inputs.enqueue(TriggerEvent::Resume { state: Activate, last_state: 0 }).unwrap();

        let mut process_queue: Macro<5, 5> = Macro::new(inputs, Queue::new());
        assert_eq!(process_queue.inputs.len(), 2);
        assert_eq!(process_queue.outputs.len(), 0);
        process_queue.process();
        assert_eq!(process_queue.inputs.len(), 0);
        assert_eq!(process_queue.outputs.len(), 2);
    }
}

