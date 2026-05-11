use std::sync::Arc;

use aion_event::prelude::{EventBuffer, EventHistory, EventSystem};
use aion_processor::prelude::Processor;
use aion_program::prelude::ProgramRegistry;

// should be called *after* BlockingProcessor because it might acquire conflicting accesses which are held for the entire duration.

// split into Start/Finish if systems want to know if a background system is active? (since even if it has finished it will still be active until it is joined at the start of this execute)

pub struct NonBlockingProcessor;

impl EventSystem for NonBlockingProcessor {
    fn execute(
        &self,
        program_registry: &Arc<ProgramRegistry>, 
        current_events: &EventBuffer,
        event_history: &EventHistory,
    ) -> EventBuffer {
        todo!()
        // FINISH
        // for each join handle
        // if complete
        // get result
        // put system cell back into stored system
        // spawn events using system_event_registry





        // START
        // for each program
        // for system metadata in non blocking system registry
        // if system.test passes
        // systems.insert(program, resource, system metadata)

        // map systems into queue input

        // get runtime

        // for each system in system queue
        // put into active system registry

        // let join_handles = Processor::process_non_blocking(
        //     system_queue, 
        //     program_registry, 
        //     runtime
        // );

        // store join handles
    }
}